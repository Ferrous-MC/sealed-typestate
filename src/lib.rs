#![warn(missing_docs)]
// #![doc(html_playground_url = "https://play.rust-lang.org/")]
//! Declare enum-based typestates easily

#[macro_export]
/// # Declare a new sealed typestate
///
/// # Example
/// ```
/// mod private {
///     pub trait Sealed {}
/// }
/// use std::marker::PhantomData;
/// use sealed_typestate::sealed_typestate;
///
/// sealed_typestate!(State {
///     State1,
///     State2
/// });
///
/// struct Foo<S>
/// where S: State {
///     _marker: PhantomData<S>
/// }
/// impl Foo<State1> {
///     fn new() -> Self {
///         Self {  _marker: PhantomData  }
///   }
/// }
/// impl Foo<State1> {
///     fn state1_only(&self) {}
/// }
///
/// impl Foo<State2> {
///    fn state2_only(&self) {}
/// }
/// ```
/// For this setups, this will compile:
/// ```
/// # mod private {
/// #     pub trait Sealed {}
/// # }
/// # use std::marker::PhantomData;
/// # use sealed_typestate::sealed_typestate;
/// #
/// # sealed_typestate!(State {
/// #    State1,
/// #    State2
/// # });
/// #
/// # struct Foo<S>
/// # where S: State {
/// #     _marker: PhantomData<S>
/// # }
/// # impl Foo<State1> {
/// #     fn new() -> Self {
/// #        Self {
/// #           _marker: PhantomData
/// #       }
/// #   }
/// # }
/// # impl Foo<State1> {
/// #     fn state1_only(&self) {}
/// # }
/// #
/// # impl Foo<State2> {
/// #    fn state2_only(&self) {}
/// # }
/// let foo = Foo::new();
/// foo.state1_only();
/// ```
/// But this will not:
/// ```
/// # mod private {
/// #    pub trait Sealed {}
/// # }
/// # use std::marker::PhantomData;
/// # use sealed_typestate::sealed_typestate;
///
/// # sealed_typestate!(State {
/// #    State1,
/// #    State2
/// # });
/// #
/// # struct Foo<S>
/// # where S: State {
/// #     _marker: PhantomData<S>
/// # }
/// # impl<S> Foo<S> {
/// #     fn new() -> Self<State1> {
/// #        Self {
/// #           _marker: PhantomData
/// #       }
/// #   }
/// # }
/// # impl Foo<State1> {
/// #     fn state1_only(&self) {}
/// # }
/// #
/// # impl Foo<State2> {
/// #    fn state2_only(&self) {}
/// # }
/// let foo = Foo::new();
/// foo.state2_only();
/// ```
///
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
