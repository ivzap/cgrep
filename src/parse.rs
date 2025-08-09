use tree_sitter::{Parser, Language, Tree};
unsafe extern "C" { unsafe fn tree_sitter_rust() -> Language; }

pub fn parse_file(path: &str) -> Tree {
    let source_code = std::fs::read_to_string(path).expect("Failed to read file");
    let mut parser = Parser::new();
    parser.set_language(unsafe { tree_sitter_rust() }).expect("Error loading Rust grammar");
    let tree = parser.parse(&source_code, None).expect("Failed to parse");
    // You can now use `tree` for further analysis
    return tree;
}