fn main() {
println!("cargo::rustc-check-cfg=cfg(allow_gnu)");
println!("cargo::rustc-check-cfg=cfg(i_know_what_im_doing)");
}
