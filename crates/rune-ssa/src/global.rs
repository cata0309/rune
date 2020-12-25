use crate::Constant;
use hashbrown::HashMap;
use std::cell::{Cell, Ref, RefCell};
use std::fmt;
use std::rc::Rc;

/// The identifier of a constant.
#[derive(Debug, Clone, Copy)]
pub struct ConstId(usize);

impl fmt::Display for ConstId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "c{}", self.0)
    }
}

/// A variable that can be used as block entries or temporaries.
/// Instructions typically produce and use vars.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ValueId(usize);

impl fmt::Display for ValueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

/// Identifier to a block.
#[derive(Debug, Clone, Copy)]
pub struct BlockId(usize);

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "block{}", self.0)
    }
}

/// Global construction state of the state machine.
#[derive(Debug, Clone, Default)]
pub(crate) struct Global {
    inner: Rc<GlobalInner>,
}

impl Global {
    /// Allocate a global variable.
    pub(crate) fn value(&self) -> ValueId {
        let id = self.inner.value.get();
        self.inner.value.set(id + 1);
        ValueId(id)
    }

    /// Allocate a global block identifier.
    pub(crate) fn block(&self) -> BlockId {
        let id = self.inner.block.get();
        self.inner.block.set(id + 1);
        BlockId(id)
    }

    /// Allocate a constant.
    pub(crate) fn constant(&self, constant: Constant) -> ConstId {
        let mut constants = self.inner.constants.borrow_mut();

        match &constant {
            Constant::Unit => return ConstId(0),
            Constant::String(s) => {
                let mut string_rev = self.inner.constant_string_rev.borrow_mut();

                if let Some(const_id) = string_rev.get(s) {
                    return *const_id;
                }

                let const_id = ConstId(constants.len());
                string_rev.insert(s.clone(), const_id);
                constants.push(constant);
                return const_id;
            }
            Constant::Bytes(b) => {
                let mut bytes_rev = self.inner.constant_bytes_rev.borrow_mut();

                if let Some(const_id) = bytes_rev.get(b) {
                    return *const_id;
                }

                let const_id = ConstId(constants.len());
                bytes_rev.insert(b.clone(), const_id);
                constants.push(constant);
                return const_id;
            }
            _ => (),
        }

        let const_id = ConstId(constants.len());
        constants.push(constant);
        const_id
    }

    /// Access the collection of available constants.
    pub(crate) fn constants(&self) -> Ref<'_, [Constant]> {
        Ref::map(self.inner.constants.borrow(), |c| c.as_slice())
    }
}

/// Inner state of the global.
#[derive(Debug)]
struct GlobalInner {
    /// Variable allocator.
    value: Cell<usize>,
    /// Block allocator.
    block: Cell<usize>,
    /// The values of constants.
    constants: RefCell<Vec<Constant>>,
    /// Constant strings that have already been allocated.
    constant_string_rev: RefCell<HashMap<Box<str>, ConstId>>,
    /// Constant byte arrays that have already been allocated.
    constant_bytes_rev: RefCell<HashMap<Box<[u8]>, ConstId>>,
}

impl Default for GlobalInner {
    fn default() -> Self {
        Self {
            value: Default::default(),
            block: Default::default(),
            constants: RefCell::new(vec![Constant::Unit]),
            constant_string_rev: Default::default(),
            constant_bytes_rev: Default::default(),
        }
    }
}