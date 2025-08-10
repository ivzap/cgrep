use tree_sitter::{Parser, Language, Tree};
use tokio::task;

unsafe extern "C" { unsafe fn tree_sitter_rust() -> Language; }

pub async fn parse_file(path: String) -> (String, Tree) {
    let source_code = tokio::fs::read_to_string(&path)
        .await
        .expect("Failed to read file");

    // Move both path and source_code into spawn_blocking closure
    let path_clone: String = path.clone();
    let tree = task::spawn_blocking(move || {
        let mut parser = Parser::new();
        unsafe {
            parser
                .set_language(tree_sitter_rust())
                .expect("Error loading Rust grammar");
        }
        parser.parse(&source_code, None).expect("Failed to parse")
    })
    .await
    .expect("Parsing task panicked");

    (path_clone, tree)
}
