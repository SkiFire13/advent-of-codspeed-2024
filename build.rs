fn main() {
    println!("cargo::rustc-check-cfg=cfg(avx512_available)");
    if is_x86_feature_detected!("avx512vl") {
        println!("cargo::rustc-cfg=avx512_available");
    }
}
