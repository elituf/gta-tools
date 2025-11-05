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
    println!(
        "cargo:rustc-env=LATEST_GIT_COMMIT_HASH={}",
        latest_git_commit_hash()
    )
}

fn latest_git_commit_hash() -> String {
    let git_rev_parse = std::process::Command::new("git")
        .args(&["rev-parse", "--short=8", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8(git_rev_parse.stdout).unwrap()
}
