//! Utility functions.

#[macro_export]
macro_rules! index_type {
    ($t:ident) => {
        #[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy, Debug, Hash)]
        pub struct $t(u32);

        impl From<u32> for $t {
            fn from(index: u32) -> $t {
                $t(index as u32)
            }
        }

        impl Into<u32> for $t {
            fn into(self) -> u32 {
                self.0 as u32
            }
        }
    };
}
