use sealed_typestate::sealed_typestate;

mod private {
    pub trait Sealed {}
}

sealed_typestate!(Foo<_> { Bar, Baz });

fn main() {}
