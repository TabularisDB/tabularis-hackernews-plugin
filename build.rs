fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        println!("cargo:rustc-link-lib=rstrtmgr");
    }

    if target_os == "linux" && std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "gnu" {
        // DuckDB's static extension registrar is linked after its implementation.
        // GNU ld needs the symbol up front to extract it from the earlier archive.
        println!("cargo:rustc-link-arg=-Wl,--undefined=duckdb_register_static_extensions");
    }
}
