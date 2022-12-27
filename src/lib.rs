#[macro_export]
macro_rules! sealed_typestate {
    ($state:ident { $($states:ident),* }) => {
        sealed_typestate!($state crate::private::Sealed { $($states),* });
    };
    ($state:ident $sealed:path { $($states:ident),* }) => {
        pub trait $state: $sealed {}
        $(
            pub enum $states {}
            impl $sealed for $states {}
            impl $state for $states {}
        )*
    };
}

mod private {
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
