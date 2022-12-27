use sealed_typestate::sealed_typestate;

mod private {
    pub trait Sealed {}
}

sealed_typestate!(Foo<wrong::path::Sealed> { Bar, Baz });

fn main() {}
