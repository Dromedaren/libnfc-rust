
extern crate pkg_config;

use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS")
        .expect(r#"The CARGO_CFG_TARGET_OS environment is not set in the build script."#);

    // Prefer the built-in service/library if available, otherwise try
    // libpcsclite.
    match &*target_os {
        _ => {
            if let Ok(lib_dir) = env::var("LIBNFC_LIB_DIR") {
                println!("cargo:rustc-link-search=native={}", lib_dir);
                println!("cargo:rustc-link-lib={}", env::var("LIBNFC_LIB_NAME").unwrap_or("libnfc".to_string()));
            } else {
                pkg_config::Config::new()
                    .atleast_version("1.7")
                    .probe("libnfc")
                    .expect(&format!(
                        r#"Could not find a libnfc library.
For the target OS `{}`, I tried to use pkg-config to find libnfc.
Do you have pkg-config and libnfc configured for this target?"#,
                        target_os
                    ));
            }
        }
    };
}