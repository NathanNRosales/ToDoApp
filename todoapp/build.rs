fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("todoapp_icon.ico");
    res.compile().unwrap();
}