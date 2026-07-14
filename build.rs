fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_manifest_file("app.manifest");
        res.set("CompanyName", "shirushimori");
        res.set("FileDescription", "Chams Installer - Open Source");
        res.set("ProductName", "Chams");
        res.set("OriginalFilename", "Chams.exe");
        res.set("InternalName", "Chams");
        res.set("FileVersion", "0.1.0.0");
        res.set("ProductVersion", "0.1.0.0");
        res.set("LegalCopyright", "\u{00A9} 2024 shirushimori - Open Source");
        res.compile().unwrap();
    }
}
