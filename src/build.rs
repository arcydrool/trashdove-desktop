extern crate winresource;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        let version: u64 = 0 << 48 | 1 << 32 | 0 << 16 | 0;
        res.set_icon("images/trashdov.ico")
            .set("InternalName", "TDDESK.EXE")
            // manually set version 1.0.0.0
            .set_version_info(winresource::VersionInfo::PRODUCTVERSION, version);
        res.compile().unwrap();
    }
}
