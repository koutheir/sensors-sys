use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::{env, io, process, str};

use walkdir::WalkDir;

fn main() {
    let target =
        env::var("TARGET").expect("sensors-sys: Environment variable 'TARGET' was not defined");

    let out_dir = env::var_os("OUT_DIR")
        .map(PathBuf::from)
        .expect("sensors-sys: Environment variable 'OUT_DIR' was not defined");

    println!("cargo:root={}", path_to_str(&out_dir));

    if !target.contains("-linux-") || target.ends_with("-kernel") {
        return; // Nothing to build for this architecture.
    }

    let explicit_static = get_static_linking(&target);
    let compiler_search_paths = get_compiler_search_paths(&target);

    find_and_output_include_dir(&compiler_search_paths.include_paths);
    find_and_output_lib_dir(&compiler_search_paths.link_paths, &target, explicit_static);

    generate_bindings(&out_dir)
}

fn path_to_str(path: &Path) -> &str {
    path.to_str().expect("sensors-sys: Path is not valid UTF-8")
}

#[cfg(feature = "static")]
fn get_static_linking(target: &str) -> Option<bool> {
    target_env_var_os("LMSENSORS_STATIC", target)
        .map(|v| v == "1" || v == "true")
        .or_else(|| Some(true))
}

#[cfg(not(feature = "static"))]
fn get_static_linking(target: &str) -> Option<bool> {
    target_env_var_os("LMSENSORS_STATIC", target).map(|v| v == "1" || v == "true")
}

fn get_compiler_search_paths(target: &str) -> CompilerSearchPaths {
    let explicit_path = target_env_var_os("LMSENSORS_PATH", target).map(PathBuf::from);

    let mut include_dir = target_env_var_os("LMSENSORS_INCLUDE_DIR", target).map(PathBuf::from);
    let mut link_dir = target_env_var_os("LMSENSORS_LIB_DIR", target).map(PathBuf::from);

    for &name in &["CC", "CFLAGS"] {
        target_env_var_os(name, target);
    }

    if let Some(explicit_path) = explicit_path {
        if include_dir.is_none() {
            include_dir = Some(explicit_path.join("include"));
        }

        if link_dir.is_none() {
            link_dir = Some(explicit_path.join("lib"));
        }
    }

    CompilerSearchPaths::new(include_dir, link_dir)
}

#[derive(Debug)]
struct CompilerSearchPaths {
    include_paths: Vec<PathBuf>,
    link_paths: Vec<PathBuf>,
}

impl CompilerSearchPaths {
    fn new(include_dir: Option<PathBuf>, link_dir: Option<PathBuf>) -> Self {
        env::set_var("LANG", "C");

        let include_paths = if let Some(include_dir) = include_dir {
            vec![include_dir]
        } else {
            Self::get_compiler_include_paths()
                .expect("sensors-sys: Failed to discover default compiler search paths")
        };

        let link_paths = if let Some(link_dir) = link_dir {
            vec![link_dir]
        } else {
            Self::get_compiler_link_paths()
                .expect("sensors-sys: Failed to discover default linker search paths")
        };

        CompilerSearchPaths {
            include_paths,
            link_paths,
        }
    }

    fn get_compiler_include_paths() -> io::Result<Vec<PathBuf>> {
        let compiler = cc::Build::new()
            .flag("-E")
            .flag("-v")
            .flag("-x")
            .flag("c")
            .get_compiler();

        let child = compiler
            .to_command()
            .arg("-")
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::null())
            .stderr(process::Stdio::piped())
            .env("LANG", "C")
            .spawn()?;

        let output = child.wait_with_output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Compiler failed to print search directories",
            ));
        }

        let mut paths: Vec<PathBuf> = output
            .stderr
            .split(|&b| b == b'\n')
            .skip_while(|&line| line != b"#include <...> search starts here:")
            .take_while(|&line| line != b"End of search list.")
            .filter_map(|bytes| str::from_utf8(bytes).ok())
            .map(str::trim)
            .filter_map(|s| dunce::canonicalize(s).ok())
            .collect();

        paths.dedup();
        Ok(paths)
    }

    fn get_compiler_link_paths() -> io::Result<Vec<PathBuf>> {
        let compiler = cc::Build::new()
            .flag("-v")
            .flag("-print-search-dirs")
            .get_compiler();

        let child = compiler
            .to_command()
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::null())
            .env("LANG", "C")
            .spawn()?;

        let output = child.wait_with_output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Compiler failed to print search directories",
            ));
        }

        let line = output
            .stdout
            .split(|&b| b == b'\n')
            .find_map(|line| line.strip_prefix(b"libraries:"))
            .and_then(|bytes| str::from_utf8(bytes).ok())
            .map(str::trim)
            .map(|line| line.trim_start_matches('='))
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::Other,
                    "Compiler search directories format is unrecognized",
                )
            })?;

        let mut paths = Vec::<PathBuf>::with_capacity(8);

        if let Some(lib_paths) = env::var_os("LIBRARY_PATH") {
            paths.extend(env::split_paths(&lib_paths).filter_map(|s| dunce::canonicalize(s).ok()));
        }

        paths.extend(env::split_paths(line).filter_map(|s| dunce::canonicalize(s).ok()));

        paths.dedup();
        Ok(paths)
    }
}

fn target_env_var_os(name: &str, target: &str) -> Option<OsString> {
    rerun_if_env_changed(name, target);

    let target_underscores = target.replace('-', "_");

    env::var_os(format!("{}_{}", name, target))
        .or_else(|| env::var_os(format!("{}_{}", name, target_underscores)))
        .or_else(|| env::var_os(format!("TARGET_{}", name)))
        .or_else(|| env::var_os(name.to_string()))
}

fn rerun_if_env_changed(name: &str, target: &str) {
    let target_underscores = target.replace('-', "_");

    println!("cargo:rerun-if-env-changed={}_{}", name, target);
    println!("cargo:rerun-if-env-changed={}_{}", name, target_underscores);
    println!("cargo:rerun-if-env-changed=TARGET_{}", name);
    println!("cargo:rerun-if-env-changed={}", name);
}

fn rerun_if_dir_changed(dir: &Path) {
    for file in WalkDir::new(dir).follow_links(false).same_file_system(true) {
        if let Ok(file) = file {
            println!("cargo:rerun-if-changed={}", file.path().display());
        } else {
            panic!(
                "sensors-sys: Failed to list directory contents: {}",
                dir.display()
            );
        }
    }
}

fn find_and_output_include_dir(include_paths: &[PathBuf]) -> PathBuf {
    let include_path = find_file_in_dirs("sensors/sensors.h", include_paths)
        .expect("sensors-sys: Failed to find 'sensors/sensors.h'");

    rerun_if_dir_changed(&include_path.join("sensors"));

    println!("cargo:include={}", path_to_str(&include_path));

    include_path
}

fn output_lib_dir(dir: &Path, file: &Path, static_lib: bool) {
    println!("cargo:rerun-if-changed={}", file.display());

    println!("cargo:lib={}", path_to_str(dir));

    println!("cargo:rustc-link-search=native={}", path_to_str(dir));

    println!(
        "cargo:rustc-link-lib={}=sensors",
        if static_lib { "static" } else { "dylib" }
    );
}

fn find_and_output_lib_dir(link_paths: &[PathBuf], target: &str, explicit_static: Option<bool>) {
    let lib_configs = match explicit_static {
        Some(false) => vec![false],

        Some(true) => vec![true],

        None => {
            if target.contains("-musl") {
                vec![true, false]
            } else {
                vec![false, true]
            }
        }
    };

    for static_lib in lib_configs {
        let file_name = format!("libsensors{}", if static_lib { ".a" } else { ".so" });

        if let Ok(lib_path) = find_file_in_dirs(&file_name, link_paths) {
            output_lib_dir(&lib_path, &lib_path.join(&file_name), static_lib);
            return;
        }

        if let Some(link_path) = link_paths.get(0) {
            let triplet = target.replace("-unknown-", "-").replace("-none-", "-");

            for &lib_dir in &[
                link_path,
                &link_path.join(&target),
                &link_path.join(&triplet),
            ] {
                let lib_path = lib_dir.join(&file_name);
                if let Ok(md) = lib_path.metadata() {
                    if md.is_file() {
                        output_lib_dir(lib_dir, &lib_path, static_lib);
                        return;
                    }
                }
            }
        }
    }
}

fn generate_bindings(out_dir: &Path) {
    let mut builder = bindgen::Builder::default()
        .rustfmt_bindings(true)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        .size_t_is_usize(true)
        .derive_debug(true)
        .derive_copy(true)
        .derive_eq(true)
        .derive_ord(true)
        .impl_debug(true);

    // Make the `FILE` type opaque, so bindgen does not pull other types from the
    // standard library.
    // Then block `FILE`, so bindgen does not emit it.
    // Then define `FILE` explicitly so that it can only be used behind a pointer.
    builder = builder.opaque_type("^FILE$");
    builder = builder.blocklist_type("^FILE$");
    builder = builder.raw_line("pub type FILE = ::std::ffi::c_void;");

    // Expose documented types.
    builder = builder.allowlist_type("^sensors_.+$");

    // Expose documented variables.
    for name in ["^sensors_.+$", "^libsensors_.+$", "^SENSORS_.+$"] {
        builder = builder.allowlist_var(name);
    }

    // Expose documented functions.
    builder = builder.allowlist_function("^sensors_.+$");

    // Include all LM Sensors headers.
    builder = builder.header("src/sensors-sys.h");

    let bindings = builder.generate().expect(
        "sensors-sys: Failed to generate Rust bindings for 'sensors/sensors.h' and other headers",
    );

    bindings
        .write_to_file(out_dir.join("sensors-sys.rs"))
        .expect("sensors-sys: Failed to write 'sensors-sys.rs'")
}

fn find_file_in_dirs(path_suffix: &str, dirs: &[PathBuf]) -> io::Result<PathBuf> {
    for dir in dirs {
        if let Ok(md) = dir.join(path_suffix).metadata() {
            if md.file_type().is_file() {
                return Ok(dir.clone());
            }
        }
    }

    Err(io::ErrorKind::NotFound.into())
}
