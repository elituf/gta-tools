fn main() {
    static_vcruntime::metabuild();
    tauri_winres::WindowsResource::new()
        .set("FileDescription", "GTA Tools")
        .set("ProductName", "GTA Tools")
        .set("LegalCopyright", "futile <git@futile.eu>")
        .set_language(0x0009)
        .set_icon("assets/icon.ico")
        .compile()
        .unwrap();
}
