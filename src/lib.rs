#[macro_export]
macro_rules! sealed_typestate {
    ($state:ident $sealed:ty { $($states:ident)* }) => {
        pub trait $state: $sealed {}
        $(
            pub enum $states {}
            impl $sealed for $states {}
            impl $state for $states {}
        )*
    };
}

#[cfg(test)]
mod tests {}
