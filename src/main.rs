use cgrep::{parallel_search, parse_files_async, walk_directory};

use std::time::Instant;
use std::{collections::HashMap, env};
use tree_sitter::Tree;
use tree_sitter_rust::language as tree_sitter_rust;

#[tokio::main]
async fn main() {
    // Get the search pattern from command line args
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <dir> <search_pattern>", args[0]);
        std::process::exit(1);
    }

    let keyword = &args[2];

    let exts: Vec<String> = vec!["rs".to_string()];
    let directory = &args[1];

    let start: Instant = Instant::now();

    let files = walk_directory(&directory, &exts);

    let duration = start.elapsed();
    println!("Directory Walk Elapsed time: {:.2?}", duration);

    println!("╔═════════════════════════════════════════════════════════╗");
    println!("║                      Searching Paths                    ║");
    println!("╚═════════════════════════════════════════════════════════╝");
    for file in &files {
        println!("  • {}", file);
    }
    let start: Instant = Instant::now();

    let trees: HashMap<String, Tree> = parse_files_async(files).await;

    let duration = start.elapsed();
    println!("Tree generation Elapsed time: {:.2?}", duration);

    let language = tree_sitter_rust();

    let start = Instant::now();

    let results = parallel_search(trees, keyword, language);

    let duration = start.elapsed();
    println!("Search Elapsed time: {:.2?}", duration);

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
