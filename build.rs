fn main() {
    // Only compile resources on Windows
    #[cfg(windows)]
    {
        // Embed Windows resources (icon, version info)
        embed_resource::compile("resources/app.rc", embed_resource::NONE);
    }
}
