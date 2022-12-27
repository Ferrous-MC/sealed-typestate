//!
#[macro_export]
macro_rules! sealed_typestate {
    ($state:ident<_> { $($states:ident),* }) => {
        sealed_typestate!($state<crate::private::Sealed> { $($states),* });
    };
    ($state:ident<$sealed:path> { $($states:ident),* }) => {
        pub trait $state: $sealed {}
        $(
            pub enum $states {}
            impl $sealed for $states {}
            impl $state for $states {}
        )*
    };
    ($state:ident { $($states:ident),* }) => {
        sealed_typestate!($state<private::Sealed> { $($states),* });
        mod private {
            pub trait Sealed {}
        }
    };
}

#[cfg(test)]
mod tests {
    use trybuild::TestCases;

    #[test]
    fn test_compiles() {
        let t = TestCases::new();
        t.pass("tests/compile-tests/01-default-sealed-trait.rs");
        t.pass("tests/compile-tests/02-specified-sealed-trait.rs");
        t.pass("tests/compile-tests/03-in-place-declared-sealed-trait.rs");
        t.compile_fail("tests/compile-tests/04-default-sealed-trait-does-not-exist.rs");
        t.compile_fail("tests/compile-tests/05-specified-sealed-trait-does-not-exist.rs");
    }
}
