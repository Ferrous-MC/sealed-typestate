use sealed_typestate::sealed_typestate;

sealed_typestate!(Foo<_> { Bar, Baz });

fn main() {}
