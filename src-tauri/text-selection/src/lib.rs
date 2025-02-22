pub mod types;
pub mod utils;

#[cfg(unix)]
mod unix_impl;
#[cfg(windows)]
mod win_impl;
