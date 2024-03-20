extern crate cc;

fn main(){
    cc::Build::new()
    .file("bin/main.rs")
    .ar_flag("-03")
    .compile("Tiks")

}