use std::{collections::HashMap, env};
use tree_sitter::{Tree, Language};
use tree_sitter_rust::language as tree_sitter_rust;
use cgrep::{walk_directory, parse_file, search};
use std::time::Instant;
use futures::future::join_all;
use std::thread;


fn parallel_search(
    trees: HashMap<String, Tree>,
    keyword: &str,
    language: Language,
) -> Vec<String> {
    let num_threads = 4;
    let mut results = Vec::new();

    // Convert to Vec to chunk
    let entries: Vec<_> = trees.into_iter().collect();
    let chunk_size = (entries.len() + num_threads - 1) / num_threads;

    let chunks: Vec<_> = entries.chunks(chunk_size).map(|c| c.to_vec()).collect();

    let mut handles = Vec::new();

    for chunk in chunks {
        // Move owned copies into the thread
        let keyword = keyword.to_string();
        let language = language;
        let handle = thread::spawn(move || {
            // Rebuild HashMap inside thread from chunk
            let map: HashMap<String, Tree> = chunk.into_iter().collect();

            // Run the search
            search(map, &keyword, language)
        });
        handles.push(handle);
    }

    // Join threads and collect all results
    for handle in handles {
        results.extend(handle.join().expect("Thread panicked"));
    }

    results
}

async fn parse_files_async(files: Vec<String>) -> HashMap<String, Tree> {
    let futures = files.into_iter().map(|file| parse_file(file));

    let results = join_all(futures).await;

    results.into_iter().collect()
}
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

    let files = walk_directory(
        &directory,
        &exts,
    );
    
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
