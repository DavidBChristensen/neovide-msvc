// Build for neovide-msvc.
// Sets the icon for the executable.
fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/neovide-msvc.ico");
        res.compile().expect("Failed to compile resource");
    }
}
