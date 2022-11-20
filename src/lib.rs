#![cfg(all(target_os = "linux", not(target_env = "kernel")))]
#![doc = include_str!("../README.md")]
#![doc(html_root_url = "https://docs.rs/sensors-sys/0.2.3")]

include!(concat!(env!("OUT_DIR"), "/sensors-sys.rs"));

#[cfg(test)]
mod tests;
