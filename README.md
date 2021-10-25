[![crates.io](https://img.shields.io/crates/v/sensors-sys.svg)](https://crates.io/crates/sensors-sys)
[![docs.rs](https://docs.rs/sensors-sys/badge.svg)](https://docs.rs/sensors-sys)
[![license](https://img.shields.io/github/license/koutheir/sensors-sys?color=black)](https://raw.githubusercontent.com/koutheir/sensors-sys/master/LICENSE.txt)

# `sensors-sys`: Unsafe Rust bindings for `libsensors`

`lm-sensors` provides user-space support for the hardware monitoring drivers
in Linux.

This crate is Linux-specific. Building it for non-Linux platforms, or for
the Linux kernel, results in an empty crate.

## Supported environment variables

This crate depends on some environment variables, and *variants* of those.
For each environment variable (e.g., `CC`), the following are the accepted
variants of it:
- `<var>_<target>`, e.g., `CC_aarch64-unknown-linux-gnu`.
- `<var>_<target-with-underscores>`, e.g., `CC_aarch64_unknown_linux_gnu`.
- `TARGET_<var>`, e.g., `TARGET_CC`.
- `<var>`, e.g., `CC`.

The following environment variables (and their variants) affect how this crate
is built:
- `LMSENSORS_STATIC`
- `LMSENSORS_PATH`
- `LMSENSORS_INCLUDE_DIR`
- `LMSENSORS_LIB_DIR`
- `CC`
- `CFLAGS`

## Dynamic or static linking

This crate links to `libsensors` dynamically if possible, except when targeting
platforms based on the `musl` C library.

This behavior can be changed either by setting the environment variable
`LMSENSORS_STATIC` to `1`, or by enabling the crate feature `static`.
If both are defined, then the value of `LMSENSORS_STATIC` takes precedence.

Setting `LMSENSORS_STATIC` to `0` mandates dynamic linking.

## Finding SELinux library and headers

By default, this crate finds SELinux headers and library based on the default
target C compiler.

This behavior can be changed by:
- Either defining the environment variable `LMSENSORS_PATH` to the path of
  a directory containing the sub-directories `include` and `lib` where
  the headers and library are installed.
- Or by defining one or both of the environment variables `LMSENSORS_INCLUDE_DIR`
  and `LMSENSORS_LIB_DIR` to paths to the directories where headers and library
  are present. If `LMSENSORS_PATH` is also defined, then `LMSENSORS_INCLUDE_DIR`
  and `LMSENSORS_LIB_DIR` take precedence.

## Depending on this crate

This crate provides the following variables to other crates that depend on it:
- `DEP_LMSENSORS_INCLUDE`: Path of the directory where library C header files reside.
- `DEP_LMSENSORS_LIB`: Path of the directory where the library binary resides.

# Documentation-only build mode

The *documentation-only* build mode allows building documentation even if
`libsensors` and its headers are unavailable.
To build in this mode, set the environment variable `DOCS_RS` to `1`:
```bash
$ env DOCS_RS=1 cargo doc --open
```

The generated documentation is based on `libsensors` version `3.6.0`.

> ⚠️ The generated crate might be **unusable** in this mode.

## Versioning

This project adheres to [Semantic Versioning].
The `CHANGELOG.md` file details notable changes over time.

[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
