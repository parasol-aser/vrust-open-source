// #![feature(const_generics)]
#![allow(non_upper_case_globals)]
#![allow(incomplete_features)]

pub mod api;

use solitaire::*;

#[cfg(feature = "no-entrypoint")]
pub mod instructions;

pub use api::{
    post_message,
    PostMessage,
    PostMessageData,
};

solitaire! {
    PostMessage(PostMessageData)                => post_message,
}
