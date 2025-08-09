use std::{collections::HashMap, env};
use tree_sitter::{Tree};
use tree_sitter_rust::language as tree_sitter_rust;
use cgrep::{walk_directory, parse_file, search};

fn main() {
    // Get the search pattern from command line args
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <dir> <search_pattern>", args[0]);
        std::process::exit(1);
    }

    let keyword = &args[2];

    let exts: Vec<String> = vec!["rs".to_string()];
    let directory = &args[1];
    let files = walk_directory(
        &directory,
        &exts,
    );

        println!("╔═════════════════════════════════════════════════════════╗");
        println!("║                      Searching Paths                    ║");
        println!("╚═════════════════════════════════════════════════════════╝");
    for file in &files {
        println!("  • {}", file);
    }
    let mut trees: HashMap<String, Tree> = HashMap::new();
    for file in &files {
        let tree = parse_file(file);
        trees.insert(file.clone(), tree);
    }

    let language = tree_sitter_rust();
    let results = search(trees, keyword, language);
    
    if results.is_empty() {
        println!(
            "❌ Pattern [{}] not found in directory: {}",
            keyword, directory
        );
    } else {
        println!("╔═════════════════════════════════════════════════════════╗");
        println!("║        ✅ Found pattern in the following locations      ║");
        println!("╚═════════════════════════════════════════════════════════╝");
        for result in &results {
            println!("  - {}", result);
        }
    }
}
