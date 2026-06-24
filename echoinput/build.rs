fn main() {
    #[cfg(target_os = "windows")]
    {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let mut res = winresource::WindowsResource::new();
        res.set_icon(format!("{}/../../assets/favicon.ico", manifest_dir));
        res.set_manifest_file(format!("{}/resources/manifest.xml", manifest_dir));
        res.compile().expect("failed to compile Windows resources");
    }
}
