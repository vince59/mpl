fn main() {
    // Déclare à rustc que `cfg(rust_analyzer)` est un cfg attendu
    println!("cargo::rustc-check-cfg=cfg(rust_analyzer)");

    println!("cargo:rerun-if-changed=src/grammar.lalrpop");
    lalrpop::process_root().unwrap();
}