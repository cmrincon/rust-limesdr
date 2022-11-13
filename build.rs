

fn main() {

    println!("cargo:rustc-link-search=/home/cmri/programming/pw1/lib");
    println!("cargo:rustc-link-lib=dylib=LimeSuite");
    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=usb-1.0");
    println!("cargo:rustc-link-lib=dylib=udev");
}