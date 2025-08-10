use tree_sitter::{Language, Query, QueryCursor, Tree};

use std::{fs, thread};
use tree_sitter::Parser;

use std::collections::HashMap;
use tree_sitter::Node;

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

fn to_sexp_with_fields(
    node: &Node,
    source: &[u8],
    mut field_start: u32,
) -> (String, u32) {
    let kind = node.kind();

    if node.child_count() == 0 {
        return (format!("({})", kind), field_start);
    }

    let mut parts = Vec::new();

    for i in 0..node.child_count() {
        let child = node.child(i).unwrap();

        if !child.is_named() {
            continue;
        }

        if let Some(field_name) = node.field_name_for_child(i as u32) {
            let field_id = format!("{}{}", field_name, field_start);

            let node_source_text = get_node_text(&child, &source);
            let eq_expr = if node_source_text.contains('\n') {
                String::new()
            } else {
                format!(" (#eq? @{} \"{}\")", field_id, node_source_text)
            };

            field_start += 1;

            let (child_sexp, new_field_start) =
                to_sexp_with_fields(&child, &source, field_start);
            field_start = new_field_start;

            parts.push(format!(
                "{}: {} @{}{}",
                field_name, child_sexp, field_id, eq_expr
            ));
        } else {
            let (child_sexp, new_field_start) =
                to_sexp_with_fields(&child, &source, field_start);
            field_start = new_field_start;
            parts.push(format!("{}", child_sexp));
        }
    }

    if parts.is_empty() {
        (format!("({})", kind), field_start)
    } else {
        (format!("({} {})", kind, parts.join(" ")), field_start)
    }
}

pub fn search(
    trees: HashMap<String, Tree>,
    keyword: &str,
    language: Language,
) -> Vec<String> {
    let mut results = Vec::new();

    // Parse the keyword into a tree
    let mut parser = Parser::new();
    parser
        .set_language(language)
        .expect("Error loading language");
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

    let query_pattern = format!(
        "({} @match)",
        to_sexp_with_fields(&child, &keyword.as_bytes(), 0).0
    );
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

pub fn parallel_search(
    trees: HashMap<String, Tree>,
    keyword: &str,
    language: Language,
) -> Vec<String> {
    let num_threads = 4;
    let mut results = Vec::new();

    let entries: Vec<_> = trees.into_iter().collect();
    let chunk_size = (entries.len() + num_threads - 1) / num_threads;

    let chunks: Vec<_> =
        entries.chunks(chunk_size).map(|c| c.to_vec()).collect();

    let mut handles = Vec::new();

    for chunk in chunks {
        let keyword = keyword.to_string();
        let language = language;
        let handle = thread::spawn(move || {
            let map: HashMap<String, Tree> = chunk.into_iter().collect();

            search(map, &keyword, language)
        });
        handles.push(handle);
    }

    for handle in handles {
        results.extend(handle.join().expect("Thread panicked"));
    }

    results
}
