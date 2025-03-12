extern crate winresource;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("images/trashdov.ico")
            .set("InternalName", "TDDESK.EXE")
            // manually set version 1.0.0.0
            .set_version_info(winresource::VersionInfo::PRODUCTVERSION, 0x0001000000000000);
        res.compile().unwrap();
    }
}
