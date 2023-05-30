fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let clippy_args = std::env::var("CLIPPY_ARGS");

    // Not build with clippy or doc
    if target_os == "none" && clippy_args.is_err() {
        let mut base_config = cc::Build::new();

        base_config
            .file("tests/test.c")
            .file("tests/mitest/mitest.c")
            .file("tests/malloc_large/malloc_large.c")
            .file("tests/glibc_bench/glibc_bench.c")
            .file("tests/multi_thread_c/multi_thread_c.c");

        base_config.warnings(true).compile("liballocator_test.a");
    }

    println!("cargo:rustc-link-lib=static=allocator_test");
    println!("cargo:rerun-if-changed=tests/test.c");
    println!("cargo:rerun-if-changed=tests/test.h");
    println!("cargo:rerun-if-changed=tests/mitest/mitest.c");
    println!("cargo:rerun-if-changed=tests/mitest/mitest.h");
    println!("cargo:rerun-if-changed=tests/malloc_large/malloc_large.c");
    println!("cargo:rerun-if-changed=tests/malloc_large/malloc_large.h");
    println!("cargo:rerun-if-changed=tests/glibc_bench/glibc_bench.c");
    println!("cargo:rerun-if-changed=tests/glibc_bench/glibc_bench.h");
    println!("cargo:rerun-if-changed=tests/multi_thread_c/multi_thread_c.c");
    println!("cargo:rerun-if-changed=ttests/multi_thread_c/multi_thread_c.h");
}