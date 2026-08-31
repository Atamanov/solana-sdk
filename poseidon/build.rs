fn main() {
    println!("cargo:rerun-if-env-changed=SOLANA_BN254_BACKEND");
    println!("cargo::rustc-check-cfg=cfg(solana_poseidon_backend, values(\"ark05\", \"narsil\"))");

    let requested = std::env::var("SOLANA_BN254_BACKEND").unwrap_or_else(|_| "ark05".to_string());
    let backend = match requested.as_str() {
        "ark05" | "narsil" => requested.as_str(),
        "ark06" | "mcl" => {
            println!("cargo:warning=solana-poseidon has no {requested} backend, using ark05");
            "ark05"
        }
        _ => {
            panic!("SOLANA_BN254_BACKEND must be one of ark05, ark06, mcl, narsil, got {requested}")
        }
    };
    println!("cargo:rustc-cfg=solana_poseidon_backend=\"{backend}\"");
    println!("cargo:warning=solana-poseidon backend {backend}");
}
