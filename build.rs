fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("WebPicGet.ico");
    res.compile().unwrap();
}