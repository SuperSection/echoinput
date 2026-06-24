fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../assets/favicon.ico");
        res.set_manifest_file("resources/manifest.xml");
        res.compile().expect("failed to compile Windows resources");
    }
}
