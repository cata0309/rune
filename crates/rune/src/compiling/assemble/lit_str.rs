use crate::compiling::assemble::prelude::*;

/// Compile a literal string `"Hello World"`.
impl Assemble for ast::LitStr {
    fn assemble(&self, c: &mut Compiler<'_>, needs: Needs) -> CompileResult<Value> {
        let span = self.span();
        log::trace!("LitStr => {:?}", c.source.source(span));

        // NB: Elide the entire literal if it's not needed.
        if !needs.value() {
            c.warnings.not_used(c.source_id, span, c.context());
            return Ok(Value::top(span));
        }

        let string = self.resolve(&c.storage, &*c.source)?;
        let slot = c.unit.new_static_string(span, &*string)?;
        c.asm.push(Inst::String { slot }, span);
        Ok(Value::top(span))
    }
}
