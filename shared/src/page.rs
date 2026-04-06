pub trait Page {
    // Render the page as plain text for the REPL/CLI
    fn render(&self) -> String;
}
