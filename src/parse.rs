use futures::future::join_all;
use std::collections::HashMap;
use tokio::task;
use tree_sitter::{Language, Parser, Tree};

unsafe extern "C" {
    unsafe fn tree_sitter_rust() -> Language;
}

pub async fn parse_file(path: String) -> (String, Tree) {
    let source_code = tokio::fs::read_to_string(&path)
        .await
        .expect("Failed to read file");

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

pub async fn parse_files_async(files: Vec<String>) -> HashMap<String, Tree> {
    let futures = files.into_iter().map(|file| parse_file(file));

    let results = join_all(futures).await;

    results.into_iter().collect()
}
