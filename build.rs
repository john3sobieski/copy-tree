// Embeds the icon and the translated texts into the exe: Explorer shows the icon in the menu
// and as the file icon, Windows picks the text language at run time.
fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        let strings = std::fs::read_to_string("assets/strings.rc")
            .expect("failed to read assets/strings.rc");
        res.append_rc_content(&strings);
        res.compile().expect("failed to embed the resources");
    }
    println!("cargo:rerun-if-changed=assets/icon.ico");
    println!("cargo:rerun-if-changed=assets/strings.rc");
}
