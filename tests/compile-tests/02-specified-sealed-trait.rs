mod custom_sealed_trait {
    pub trait CustomSealed {}
}
use custom_sealed_trait::CustomSealed;
use sealed_typestate::sealed_typestate;
sealed_typestate!(Foo<CustomSealed> { Bar, Baz });

fn main() {}
