//! Configuration structs required by the analysis phase.

#[derive(Default, Clone)]
pub(crate) struct MiravConfig {
    /// The maximum window size between two atomic ops in the same thread.
    pub window_len: u32,
}

pub const MIRAV_DEFAULT_ARGS: &[&str] = &[
    "-Zalways-encode-mir",
    "-Zsymbol-mangling-version=v0",
    "-C",
    "panic=abort",
    // "-Cdebug-assertions=on",
];
