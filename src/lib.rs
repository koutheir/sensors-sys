#![cfg(all(target_os = "linux", not(target_env = "kernel")))]
#![doc = include_str!("../README.md")]
#![doc(html_root_url = "https://docs.rs/sensors-sys/0.2.8")]
#![warn(unsafe_op_in_unsafe_fn)]

include!(concat!(env!("OUT_DIR"), "/sensors-sys.rs"));

#[cfg(test)]
mod tests;
