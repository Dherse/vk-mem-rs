#[cfg(feature = "generate_bindings")]
extern crate bindgen;
extern crate cc;

use std::env;

fn main() {
    let mut build = cc::Build::new();

    build.include("vendor/src");
    build.include("wrapper");
    build.include("wrapper/vulkan");

    // TODO: Add some configuration options under crate features
    //#define VMA_HEAVY_ASSERT(expr) assert(expr)
    //#define VMA_USE_STL_CONTAINERS 1
    //#define VMA_DEDICATED_ALLOCATION 0
    //#define VMA_DEBUG_MARGIN 16
    //#define VMA_DEBUG_DETECT_CORRUPTION 1
    //#define VMA_DEBUG_INITIALIZE_ALLOCATIONS 1
    //#define VMA_DEBUG_MIN_BUFFER_IMAGE_GRANULARITY 256

    #[cfg(feature = "recording")]
    build.define("VMA_RECORDING_ENABLED", "1");

    // Add the files we build
    let source_files = ["wrapper/vma_lib.cpp"];

    for source_file in &source_files {
        build.file(&source_file);
    }

    let target = env::var("TARGET").unwrap();
    if target.contains("darwin") {
        build
            .flag("-std=c++11")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .flag("-Wno-nullability-completeness")
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .cpp(true);
    } else if target.contains("ios") {
        build
            .flag("-std=c++11")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .cpp(true);
    } else if target.contains("android") {
        build
            .flag("-std=c++11")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("c++")
            .cpp(true);
    } else if target.contains("linux") {
        build
            .flag("-std=c++11")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("stdc++")
            .cpp(true);
    } else if target.contains("windows") && target.contains("gnu") {
        build
            .flag("-std=c++11")
            .flag("-Wno-missing-field-initializers")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-reorder")
            .cpp_link_stdlib("stdc++")
            .cpp(true);
    }

    build.compile("vma_cpp");

    generate_bindings("gen/bindings.rs");
}

#[cfg(feature = "generate_bindings")]
fn generate_bindings(output_file: &str) {
    let bindings = bindgen::Builder::default()
        .clang_arg("-I./wrapper")
        .header("vendor/src/vk_mem_alloc.h")
        .rustfmt_bindings(true)
        .size_t_is_usize(true)
        .blacklist_type("__darwin_.*")
        .whitelist_function("vma.*")
        .trust_clang_mangling(false)
        .layout_tests(false)
        .generate()
        .expect("Unable to generate bindings!");

    bindings
        .write_to_file(std::path::Path::new(output_file))
        .expect("Unable to write bindings!");
}

#[cfg(not(feature = "generate_bindings"))]
fn generate_bindings(_: &str) {}
