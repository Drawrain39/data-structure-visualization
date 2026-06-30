//! visualizer-cli — command-line interface for the data-structure visualizer.
//!
//! Usage:
//!   cargo run -p visualizer-cli -- list
//!   cargo run -p visualizer-cli -- meta <algorithm>
//!   cargo run -p visualizer-cli -- trace <algorithm> <array>
//!   cargo run -p visualizer-cli -- code <algorithm> <lang>

use std::env;
use visualizer_catalog::{build_catalog, get_code_sample_json, get_line_map, get_meta, get_default_values};

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  visualizer-cli list");
    eprintln!("  visualizer-cli meta <algorithm>");
    eprintln!("  visualizer-cli trace <algorithm> <array>");
    eprintln!("  visualizer-cli code <algorithm> <lang>");
}

fn cmd_list() {
    let catalog = build_catalog();
    for entry in &catalog {
        let cat = format!("{:?}", entry.meta.category).to_lowercase();
        println!("{:30} {:12} {}", entry.algorithm, cat, entry.meta.name);
    }
    println!("\nTotal: {} algorithms", catalog.len());
}

fn cmd_meta(algorithm: &str) {
    match get_meta(algorithm) {
        Some(meta) => {
            let json = serde_json::to_string_pretty(&meta).unwrap_or_default();
            println!("{}", json);
        }
        None => {
            eprintln!("Algorithm not found: {}", algorithm);
            std::process::exit(1);
        }
    }
}

fn cmd_trace(algorithm: &str, array_str: &str) {
    let values: Vec<i32> = array_str
        .split(',')
        .map(|s| s.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|e| {
            eprintln!("Invalid array values: {}", e);
            std::process::exit(1);
        });

    match visualizer_core::generate_trace(algorithm, &values) {
        Ok(steps) => {
            let json = serde_json::to_string_pretty(&steps).unwrap_or_default();
            println!("{}", json);
        }
        Err(e) => {
            eprintln!("Error generating trace: {}", e);
            std::process::exit(1);
        }
    }
}

fn cmd_code(algorithm: &str, lang: &str) {
    let line_map = get_line_map(algorithm);
    let code_json = get_code_sample_json(algorithm);

    match code_json {
        Some(json) => {
            // Parse the code sample JSON and extract the requested language
            let parsed: serde_json::Value =
                serde_json::from_str(&json).unwrap_or_default();
            let lang_key = match lang {
                "cpp" | "c++" | "c" => "cpp",
                "python" | "py" => "python",
                "rust" | "rs" => "rust",
                _ => {
                    eprintln!("Unknown language: {}. Use cpp, python, or rust.", lang);
                    std::process::exit(1);
                }
            };

            if let Some(lang_sample) = parsed.get(lang_key) {
                if let Some(lines) = lang_sample.get("lines").and_then(|l| l.as_array()) {
                    for (i, line) in lines.iter().enumerate() {
                        println!("{:3} | {}", i + 1, line.as_str().unwrap_or(""));
                    }
                }
            }

            // Print line map
            if let Some(lm) = line_map {
                println!("\n--- Line Map ---");
                for (key, line_nums) in &lm {
                    println!(
                        "  {} -> [{}]",
                        key,
                        line_nums
                            .iter()
                            .map(|n| n.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                }
            }
        }
        None => {
            eprintln!("Algorithm not found: {}", algorithm);
            std::process::exit(1);
        }
    }
}

/// Get default values from WASM/catalog and print them
fn cmd_defaults(algorithm: &str) {
    let values = get_default_values(algorithm);
    println!(
        "{}",
        values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    match args[1].as_str() {
        "list" => cmd_list(),
        "meta" => {
            if args.len() < 3 {
                eprintln!("Missing algorithm name");
                std::process::exit(1);
            }
            cmd_meta(&args[2]);
        }
        "trace" => {
            if args.len() < 4 {
                eprintln!("Missing algorithm or array");
                std::process::exit(1);
            }
            cmd_trace(&args[2], &args[3]);
        }
        "code" => {
            if args.len() < 4 {
                eprintln!("Missing algorithm or language");
                std::process::exit(1);
            }
            cmd_code(&args[2], &args[3]);
        }
        "defaults" => {
            if args.len() < 3 {
                eprintln!("Missing algorithm name");
                std::process::exit(1);
            }
            cmd_defaults(&args[2]);
        }
        "--help" | "-h" | "help" => print_usage(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_all_algorithms() {
        let catalog = build_catalog();
        assert_eq!(catalog.len(), 40);
    }

    #[test]
    fn test_meta_quick_sort() {
        let meta = get_meta("quick-sort");
        assert!(meta.is_some());
        let m = meta.unwrap();
        assert_eq!(m.name, "快速排序");
        assert_eq!(m.name_en, "Quick Sort");
    }

    #[test]
    fn test_meta_bubble_sort() {
        let meta = get_meta("bubble-sort");
        assert!(meta.is_some());
        let m = meta.unwrap();
        assert_eq!(m.name, "冒泡排序");
    }

    #[test]
    fn test_code_sample_cpp() {
        let json = get_code_sample_json("binary-search");
        assert!(json.is_some());
        let parsed: serde_json::Value = serde_json::from_str(&json.unwrap()).unwrap();
        assert!(parsed.get("cpp").is_some());
        assert!(parsed.get("python").is_some());
        assert!(parsed.get("rust").is_some());
    }

    #[test]
    fn test_trace_quick_sort() {
        let values = vec![3, 1, 2];
        let steps = visualizer_core::generate_trace("quick-sort", &values);
        assert!(steps.is_ok());
        let steps = steps.unwrap();
        assert!(!steps.is_empty());
        // Last step should be Done
        assert_eq!(steps.last().unwrap().step_type, visualizer_core::StepType::Done);
    }

    #[test]
    fn test_trace_unknown_algorithm() {
        let result = visualizer_core::generate_trace("nonexistent", &[1, 2, 3]);
        assert!(result.is_err());
    }

    #[test]
    fn test_trace_empty_input() {
        let steps = visualizer_core::generate_trace("quick-sort", &[]);
        assert!(steps.is_ok());
    }

    #[test]
    fn test_all_algorithms_can_generate_trace() {
        let catalog = build_catalog();
        for entry in &catalog {
            let values = get_default_values(&entry.algorithm);
            if values.is_empty() {
                continue;
            }
            let result = visualizer_core::generate_trace(&entry.algorithm, &values);
            assert!(
                result.is_ok(),
                "Failed to generate trace for {}",
                entry.algorithm
            );
            let steps = result.unwrap();
            assert!(
                !steps.is_empty(),
                "Empty trace for {}",
                entry.algorithm
            );
        }
    }

    #[test]
    fn test_line_keys_match_catalog_line_map() {
        let catalog = build_catalog();
        for entry in &catalog {
            let values = get_default_values(&entry.algorithm);
            if values.is_empty() {
                continue;
            }
            let steps = visualizer_core::generate_trace(&entry.algorithm, &values).unwrap();
            for step in &steps {
                assert!(
                    entry.line_map.contains_key(&step.line_key),
                    "Algorithm '{}': line_key '{}' not found in catalog lineMap",
                    entry.algorithm,
                    step.line_key
                );
            }
        }
    }

    #[test]
    fn test_sorting_algorithms_final_state_is_sorted() {
        let sorting_algos = [
            "selection-sort", "bubble-sort", "insertion-sort", "merge-sort",
            "quick-sort", "heap-sort", "shell-sort", "counting-sort",
            "bucket-sort", "radix-sort",
        ];
        for algo in &sorting_algos {
            let values = get_default_values(algo);
            if values.is_empty() {
                continue;
            }
            let steps = visualizer_core::generate_trace(algo, &values).unwrap();
            let last = steps.last().unwrap();
            let final_items: Vec<i32> = last.items.iter().map(|it| it.value).collect();
            if !final_items.is_empty() {
                for w in final_items.windows(2) {
                    assert!(
                        w[0] <= w[1],
                        "{} final state not sorted: {:?}",
                        algo,
                        final_items
                    );
                }
            }
        }
    }

    #[test]
    fn test_searching_algorithms_handle_found_and_not_found() {
        let searching_algos = ["linear-search", "binary-search"];
        for algo in &searching_algos {
            // Found case
            let values = get_default_values(algo);
            if values.is_empty() {
                continue;
            }
            let result = visualizer_core::generate_trace(algo, &values);
            assert!(result.is_ok(), "{} failed with default values", algo);

            // Empty check
            let empty_result = visualizer_core::generate_trace(algo, &[]);
            assert!(empty_result.is_ok(), "{} failed with empty input", algo);
        }
    }
}
