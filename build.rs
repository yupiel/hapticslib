fn main() {
   println!("cargo:rustc-link-search=/lib");
   println!("cargo::rustc-link-lib=deps/sblt_plugin");
}