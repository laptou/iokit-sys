use std::{fmt::Write, path::PathBuf, process::Command};

fn main() {
    if std::env::var("TARGET").unwrap().contains("-apple") {
        println!("cargo:rustc-link-lib=framework=IOKit");
    }

    let framework_path = String::from_utf8(
        Command::new("xcrun")
            .args(["--sdk", "macosx", "--show-sdk-path"])
            .output()
            .expect("could not run xcrun; are xcode command line tools installed?")
            .stdout,
    )
    .unwrap();
    let framework_path = framework_path.trim();

    println!("framework path = {framework_path:?}");

    // these types have alignment issues due to rust-bindgen#2240
    let opaque_types = [
        "FndrOpaqueInfo",
        "HFSCatalogFolder",
        "HFSPlusCatalogFolder",
        "HFSCatalogFile",
        "HFSPlusCatalogFile",
    ];

    let debug_types = ["IORPCMessage"];

    let blocked_files = [
        // ".*sys/cdefs.h",
        // ".*sys/types.h",
        // ".*mach/mach_types.h",
        // ".*mach/mach_init.h",
        ".*CoreFoundation/CFBase.h",
        ".*CoreFoundation/CFDictionary.h",
        ".*CoreFoundation/CFRunLoop.h",
    ];

    let mut builder = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_args([
            "-x",
            "objective-c",
            "-fblocks",
            "-fmodules",
            "-isysroot",
            framework_path,
        ])
        .objc_extern_crate(true)
        .block_extern_crate(true)
        .generate_block(true);

    for opaque_ty in opaque_types {
        builder = builder.opaque_type(opaque_ty);
    }

    for blocked_file in blocked_files {
        builder = builder.blocklist_file(blocked_file);
    }

    let mut generated_code = String::new();

    for debug_ty in debug_types {
        write!(
            &mut generated_code,
            "impl ::std::fmt::Debug for {debug_ty} {{
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{
                    f.debug_struct(stringify!({debug_ty}))
                        .finish()
                }}
            }}"
        )
        .unwrap();
    }

    let bindings = builder
        .formatter(bindgen::Formatter::Prettyplease)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_item("CF.*")
        .blocklist_item("__CF.*")
        .blocklist_item("kCF.*")
        .blocklist_item("mach_.*")
        .blocklist_item("kern_.*")
        .blocklist_item("task_t")
        .blocklist_item("darwin_.*")
        .blocklist_item("__darwin_.*")
        .blocklist_item("uuid_t")
        .blocklist_item("natural_t")
        .allowlist_item("IO.*")
        .allowlist_item("kIO.*")
        .allowlist_item("__IO.*")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("iokit.rs"))
        .expect("Couldn't write bindings!");

    std::fs::write(out_path.join("iokit-extra.rs"), generated_code).unwrap();
}
