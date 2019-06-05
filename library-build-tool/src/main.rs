use std::env;
use std::fs;
use std::io;
use std::path::Path;

macro_rules! target_features {
    (@to $res:ident @rest ) => {};

    (@to $res:ident @rest $name:literal: [$($features:literal),*], $($rest:tt)*) => {
        $res.push(($name, vec![$(if $features == "" { "".to_string() } else { format!("+{}", $features) }),*]));
        target_features!(@to $res @rest $($rest)*);
    };

    ($($tt:tt)*) => {{
            let mut res = Vec::new();
            target_features!(@to res @rest $($tt)*);
            res
    }};
}

macro_rules! cmd {
    ($cmd:tt $($arg:tt)*) => {
        std::process::Command::new($cmd)
        $(
            .arg($arg)
        )*
    };
}

fn main() {
    if env::args().count() == 1 {
        println!("Usage: library-build-tool [path-to-Cargo.toml.skel] [path-to-main.rs.skel] [path-to-target-features-file] [path-to-install]");
        return;
    }

    let path_cargo_toml_skel = env::args()
        .nth(1)
        .and_then(|path| to_absolute::to_absolute_from_current_dir(path).ok())
        .expect("failed to get absolute path for Cargo.toml.skel");

    let path_main_rs_skel = env::args()
        .nth(2)
        .and_then(|path| to_absolute::to_absolute_from_current_dir(path).ok())
        .expect("failed to get absolute path for Cargo.toml.skel");

    let path_target_features = env::args()
        .nth(3)
        .and_then(|path| to_absolute::to_absolute_from_current_dir(path).ok())
        .expect("failed to get absolute path for target-features");

    let path_to_install = env::args()
        .nth(4)
        .and_then(|path| to_absolute::to_absolute_from_current_dir(path).ok())
        .expect("failed to get absolute path for install directory");

    let target_features = include!("../../target_features");

    // ensure the install path exists.
    fs::create_dir_all(&path_to_install).expect("failed to create install directory");
    fs::copy(
        &path_target_features,
        &path_to_install.join("target_features"),
    )
    .expect("failed to place target_features file");
    env::set_current_dir(&path_to_install).expect("failed to chdir to install directory");

    for (name, features) in target_features {
        build_library_for(&path_cargo_toml_skel, &path_main_rs_skel, name, &features)
            .unwrap_or_else(|e| panic!("failed to build library for {}: {}", name, e));
    }
}

fn build_library_for(
    path_cargo_toml_skel: &Path,
    path_main_rs_skel: &Path,
    target: &str,
    features: &[String],
) -> io::Result<()> {
    //--------------------------------------------------------------------------------
    // create a project for the library
    //--------------------------------------------------------------------------------
    let creation = cmd!("cargo" "new" target).status()?;

    // check if creation failed or not
    if !creation.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "failed to create a new project",
        ));
    }

    //--------------------------------------------------------------------------------
    // enter the library dir
    //--------------------------------------------------------------------------------
    env::set_current_dir(target)?;

    //--------------------------------------------------------------------------------
    // setup files
    //--------------------------------------------------------------------------------
    // Cargo.toml
    fs::remove_file("Cargo.toml")?;
    fs::copy(path_cargo_toml_skel, "Cargo.toml")?;

    // main.rs
    fs::remove_file("src/main.rs")?;
    fs::copy(path_main_rs_skel, "src/main.rs")?;

    //--------------------------------------------------------------------------------
    // setup RUSTFLAGS
    //--------------------------------------------------------------------------------
    let rustflags = format!("-C target_feature={}", features.join(","));

    //--------------------------------------------------------------------------------
    // build the library
    //--------------------------------------------------------------------------------
    let compilation = cmd!("cargo" "run" "--release")
        .env("RUSTFLAGS", rustflags)
        .status()?;

    // check if build failed or not
    if !compilation.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "failed to compile a library project",
        ));
    }

    //--------------------------------------------------------------------------------
    // leave the library dir
    //--------------------------------------------------------------------------------
    env::set_current_dir("..")?;
    Ok(())
}
