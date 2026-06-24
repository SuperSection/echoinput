fn main() {
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rerun-if-changed=resources/echoinput.rc");
        println!("cargo:rerun-if-changed=resources/manifest.xml");
        println!("cargo:rerun-if-changed=../../assets/favicon.ico");

        embed_resource::compile("resources/echoinput.rc", [] as [&str; 0]);
    }
}
