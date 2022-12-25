#[macro_export]
macro_rules! sealed_typestate {
    ($state:ident { $($states:ident),* }) => {
        pub trait $state: crate::sealed::Sealed {}
        $(
            pub enum $states {}
            impl crate::sealed::Sealed for $states {}
            impl $state for $states {}
        )*
    };
    ($state:ident $sealed:ty { $($states:ident),* }) => {
        pub trait $state: $sealed {}
        $(
            pub enum $states {}
            impl $sealed for $states {}
            impl $state for $states {}
        )*
    };
}

mod sealed {
    pub trait Sealed {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        sealed_typestate!(Side { Client, Server });
    }
}
