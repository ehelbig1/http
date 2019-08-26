pub mod startLine {
    pub struct startLine {
        method: String,
        path: String,
        version: String,
    }

    impl startLine {
        pub fn from_string(startLine: String) {
            for section in startLine.split_whitespace() {
                println!("Section: {}", section);
            }
        }
    }
}
