use cgrep::{walk_directory, parse_file, search};
use tree_sitter::{Parser, Language, Tree};
use tree_sitter_rust::language as tree_sitter_rust;
use std::collections::HashMap;

fn main() {
    // Example usage of the API (currently placeholders)
    println!("Hello, world!");
    let exts: Vec<String> = vec!{"rs".to_string()};
    let files = walk_directory(
        "/workspaces/ubuntu/.devcontainer/cgrep/src",
        &exts
    );
    println!("{:?}", files); 
    let mut trees: HashMap<String, Tree> = HashMap::new();
    
    for file in &files {
        let tree = parse_file(file);
        trees.insert(file.clone(), tree);
    }
    let keyword = "let mut cursor = QueryCursor::new();"; // Replace with actual query
    let language = tree_sitter_rust();
    let results = search(trees, keyword, language);
    for result in results {
        println!("{:?}", result);
    }
}
