use sealed_typestate::sealed_typestate;

mod private {
    pub trait Sealed {}
}

sealed_typestate!(Foo { Bar, Baz });

fn main() {}
