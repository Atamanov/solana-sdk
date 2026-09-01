use std::path::PathBuf;

const BACKENDS: [&str; 4] = ["ark05", "ark06", "mcl", "narsil"];

fn main() {
    println!("cargo:rerun-if-env-changed=SOLANA_BN254_BACKEND");
    println!(
        "cargo::rustc-check-cfg=cfg(solana_bn254_backend, values(\"ark05\", \"ark06\", \"mcl\", \"narsil\"))"
    );

    let backend = std::env::var("SOLANA_BN254_BACKEND").unwrap_or_else(|_| "ark05".to_string());
    assert!(
        BACKENDS.contains(&backend.as_str()),
        "SOLANA_BN254_BACKEND must be one of ark05, ark06, mcl, narsil, got {backend}"
    );
    println!("cargo:rustc-cfg=solana_bn254_backend=\"{backend}\"");
    println!("cargo:warning=solana-bn254 backend {backend}");

    if backend == "mcl" {
        compile_mcl_bridge();
    }
}

fn compile_mcl_bridge() {
    println!("cargo:rerun-if-env-changed=MCL_DIR");
    println!("cargo:rerun-if-changed=mcl_bridge/bridge.cpp");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("solana") {
        return;
    }

    let root =
        PathBuf::from(std::env::var_os("MCL_DIR").expect("MCL_DIR must point at a built MCL tree"));
    let include = root.join("include");
    let archive = root.join("lib256/libmcl.a");
    assert!(
        include.join("mcl/bn.h").is_file() && archive.is_file(),
        "MCL_DIR must contain include/mcl/bn.h and lib256/libmcl.a"
    );
    // Built by helius-narsil bench/scripts/provision.sh.
    const MCL_PIN: &str = "b70288469cd065b28a31764450f85238508c4d1e";
    let head = std::process::Command::new("git")
        .args(["-C", root.to_str().expect("MCL_DIR is not unicode"), "rev-parse", "HEAD"])
        .output()
        .expect("git must resolve the MCL_DIR revision");
    assert!(head.status.success(), "MCL_DIR is not a git checkout");
    let head = String::from_utf8_lossy(&head.stdout);
    assert!(
        head.trim() == MCL_PIN,
        "MCL_DIR is at {} instead of the pin {MCL_PIN}",
        head.trim()
    );

    cc::Build::new()
        .cpp(true)
        .file("mcl_bridge/bridge.cpp")
        .include(include)
        .define("MCL_FP_BIT", "256")
        .define("MCL_FR_BIT", "256")
        .define("NDEBUG", None)
        .flag_if_supported("-std=c++17")
        .compile("narsil_mcl_bridge");
    println!(
        "cargo:rustc-link-search=native={}",
        archive.parent().unwrap().display()
    );
    println!("cargo:rustc-link-lib=static=mcl");
}
