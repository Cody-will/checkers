

fn main() {
    slint_build::compile("ui/app.slint").expect("slint failed to build");
    println!("cargo:rerun-if-changed=ui/app.slint");
    println!("cargo:rerun-if-changed=ui/")
}
