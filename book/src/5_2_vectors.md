# Vectors

A vector is a native data structure of Rune which is a dynamic list of values. A
vector isn't typed, and can store *any* rune values.

```rust,noplaypen
{{#include ../../scripts/book/5_2/vectors.rn}}
```

```text
$> cargo run -- scripts/book/5_2/vectors.rn
"Hello"
42
"Hello"
42
== () (5.0674ms)
```

As you can see, you can iterate over a vector because it implements the iterator
protocol. It is also possible to create and use an iterator manually using
`Vec::iter`, giving you more control over it.

```rust,noplaypen
{{#include ../../scripts/book/5_2/vectors_rev.rn}}
```

```text
$> cargo run -- scripts/book/5_2/vectors_rev.rn
42
"Hello"
== () (2.9116ms)
```

## Using vectors from Rust

Vectors are represented externally as the standard [`Vec`].

```rust,noplaypen
{{#include ../../crates/rune/examples/vector.rs}}
```

```text
$> cargo run --example vector
[10]
```

If you have a vector which have values of non-uniform types, you can use 
[`VecTuple`] to deal with them.

```rust,noplaypen
{{#include ../../crates/rune/examples/vec_tuple.rs}}
```

```text
$> cargo run --example vec_tuple
(2, "Hello World")
```

[`Vec`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[`VecTuple`]: https://docs.rs/runestick/0/runestick/struct.VecTuple.html