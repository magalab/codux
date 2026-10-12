fn main() {
    println!("cargo:rerun-if-env-changed=CODUX_RELEASE_REPOSITORY");
}
