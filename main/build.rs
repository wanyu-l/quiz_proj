#[cfg(windows)]
extern crate winres;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("app_icon.ico");
    res.compile().unwrap();
}
