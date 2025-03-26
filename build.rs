fn main() {
    static_vcruntime::metabuild();
    winresource::WindowsResource::new()
        .set_icon("assets/icon.ico")
        .compile()
        .unwrap();
}
