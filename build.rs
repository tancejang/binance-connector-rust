use rustc_version::version;

fn main() {
    let rustc_ver = version().expect("Failed to get rustc version");
    println!("cargo:rustc-env=RUSTC_VERSION=rustc {rustc_ver}");
}
