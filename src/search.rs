use tree_sitter::{Query, QueryCursor, Language, Tree};

use tree_sitter::{Parser};
use std::fs;

use tree_sitter::{Node};
use std::collections::HashMap;

fn get_node_text(node: &Node, source: &[u8]) -> String {
    let byte_range = node.byte_range();

    if byte_range.end > source.len() {
        panic!(
            "Node byte range {:?} is out of bounds for source length {}",
            byte_range,
            source.len()
        );
    }

    std::str::from_utf8(&source[byte_range])
        .unwrap_or_else(|e| panic!("Invalid UTF-8 in node text: {}", e))
        .to_string()
}

fn to_sexp_with_fields(node: &Node, source: &[u8]) -> String {
    let kind = node.kind();

    // Leaf node: just print kind (or you can include text if you want)
    if node.child_count() == 0 {
        return format!("({})", kind);
    }

    let mut parts = Vec::new();

    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();
        
        if !child.is_named() {
            continue;
        }
        
        let child_sexp = to_sexp_with_fields(&child, &source);
        
        if let Some(field_name) = node.field_name_for_child(i as u32) {
            parts.push(format!("{}: {} @{} (#eq? @{} \"{}\")", field_name, child_sexp, field_name, field_name, get_node_text(&child, &source)));
        } else {
            parts.push(format!("{}", child_sexp));
        }
    }
    if parts.is_empty() {
        return format!("({})", kind);
    } else {
        return format!("({} {})", kind, parts.join(" "))
    }
}

pub fn search(trees: HashMap<String, Tree>, keyword: &str, language: Language) -> Vec<String> {
    let mut results = Vec::new();

    // Parse the keyword into a tree
    let mut parser = Parser::new();
    parser.set_language(language).expect("Error loading language");
    let keyword_tree = match parser.parse(&keyword, None) {
        Some(tree) => tree,
        None => {
            println!("Failed to parse keyword");
            return results;
        }
    };

    let keyword_root = keyword_tree.root_node();
    if keyword_root.is_missing() {
        println!("keyword_tree is invalid");
        return results;
    }

    let child = match keyword_root.child(0) {
        Some(c) => c,
        None => {
            return results;
        }
    };
    
    let query_pattern = format!("({} @match)", to_sexp_with_fields(&child, &keyword.as_bytes()));
    let query = Query::new(language, &query_pattern).expect("Invalid query");
    let mut cursor = QueryCursor::new();

    for (filename, tree) in trees {
        let root_node = tree.root_node();
        let bytes = fs::read(&filename)
        .unwrap_or_else(|e| panic!("Failed to read {}: {}", filename, e));

        let bytes_slice: &[u8] = &bytes;
        let matches = cursor.matches(&query, root_node, bytes_slice);
        for m in matches {
            for cap in m.captures {
                let node = cap.node;
                let start = node.start_position();
                // Tree-sitter positions are 0-based; add 1 for human-readable format
                let row = start.row + 1;
                let col = start.column + 1;
                results.push(format!("{}:{}:{}", filename, row, col));
            }
        }
    }

    results
}