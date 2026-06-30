// ── Macros shared across all catalog sub-modules ───────────────────────────

#[macro_export]
macro_rules! cpp {
    ($($l:expr),* $(,)?) => {
        CodeSample { language: "cpp".into(), label: "C++".into(), lines: vec![$($l.into()),*] }
    };
}

#[macro_export]
macro_rules! py {
    ($($l:expr),* $(,)?) => {
        CodeSample { language: "python".into(), label: "Python".into(), lines: vec![$($l.into()),*] }
    };
}

#[macro_export]
macro_rules! rs {
    ($($l:expr),* $(,)?) => {
        CodeSample { language: "rust".into(), label: "Rust".into(), lines: vec![$($l.into()),*] }
    };
}

#[macro_export]
macro_rules! lm {
    ($($k:expr => [$($v:literal),*]),* $(,)?) => {{
        let mut m = std::collections::BTreeMap::new();
        $( m.insert($k.into(), vec![$($v),*]); )*
        m
    }};
}

/// Full variant: time, best, avg, worst, space, stable, use (sorting)
#[macro_export]
macro_rules! meta_full {
    ($key:literal, $name:literal, $en:literal, $cat:ident, $desc:literal,
     time=$t:literal, best=$b:literal, avg=$a:literal, worst=$w:literal, space=$s:literal,
     stable=$st:literal, use=[$($u:literal),*]) => {
        AlgorithmMeta {
            key: $key.into(), name: $name.into(), name_en: $en.into(),
            category: AlgorithmCategory::$cat,
            description: $desc.into(),
            complexity: Complexity {
                time: $t.into(),
                best: Some($b.into()), average: Some($a.into()), worst: Some($w.into()),
                space: $s.into(),
            },
            stable: Some($st.into()),
            use_cases: vec![$($u.into()),*],
        }
    };
}

/// Variant with best/avg/worst but no stable (searching)
#[macro_export]
macro_rules! meta_no_stable {
    ($key:literal, $name:literal, $en:literal, $cat:ident, $desc:literal,
     time=$t:literal, best=$b:literal, avg=$a:literal, worst=$w:literal, space=$s:literal,
     use=[$($u:literal),*]) => {
        AlgorithmMeta {
            key: $key.into(), name: $name.into(), name_en: $en.into(),
            category: AlgorithmCategory::$cat,
            description: $desc.into(),
            complexity: Complexity {
                time: $t.into(),
                best: Some($b.into()), average: Some($a.into()), worst: Some($w.into()),
                space: $s.into(),
            },
            stable: None,
            use_cases: vec![$($u.into()),*],
        }
    };
}

/// Simple variant: time, space, use (most categories)
#[macro_export]
macro_rules! meta_simple {
    ($key:literal, $name:literal, $en:literal, $cat:ident, $desc:literal,
     time=$t:literal, space=$s:literal, use=[$($u:literal),*]) => {
        AlgorithmMeta {
            key: $key.into(), name: $name.into(), name_en: $en.into(),
            category: AlgorithmCategory::$cat,
            description: $desc.into(),
            complexity: Complexity {
                time: $t.into(), best: None, average: None, worst: None, space: $s.into(),
            },
            stable: None,
            use_cases: vec![$($u.into()),*],
        }
    };
}

// ── Sub-modules ────────────────────────────────────────────────────────────

pub mod dp;
pub mod graph;
pub mod linear;
pub mod recursion;
pub mod searching;
pub mod sorting;
pub mod stack_queue;
pub mod tree;

// ── Imports for query helpers ──────────────────────────────────────────────

use crate::types::*;

// ── Category labels ────────────────────────────────────────────────────────

pub fn category_labels() -> Vec<CategoryInfo> {
    vec![
        CategoryInfo {
            key: "sorting".into(),
            label: "排序".into(),
        },
        CategoryInfo {
            key: "searching".into(),
            label: "查找".into(),
        },
        CategoryInfo {
            key: "linear".into(),
            label: "线性表".into(),
        },
        CategoryInfo {
            key: "stack-queue".into(),
            label: "栈与队列".into(),
        },
        CategoryInfo {
            key: "recursive".into(),
            label: "递归".into(),
        },
        CategoryInfo {
            key: "tree".into(),
            label: "树".into(),
        },
        CategoryInfo {
            key: "graph".into(),
            label: "图".into(),
        },
        CategoryInfo {
            key: "dp".into(),
            label: "动态规划".into(),
        },
    ]
}

// ── Build catalog ──────────────────────────────────────────────────────────

pub fn build_catalog() -> Vec<AlgorithmEntry> {
    let mut entries = Vec::new();
    entries.extend(sorting::sorting_entries());
    entries.extend(searching::searching_entries());
    entries.extend(linear::linear_entries());
    entries.extend(stack_queue::stack_queue_entries());
    entries.extend(recursion::recursion_entries());
    entries.extend(tree::tree_entries());
    entries.extend(graph::graph_entries());
    entries.extend(dp::dp_entries());
    entries
}

// ── Query helpers ──────────────────────────────────────────────────────────

pub fn get_catalog_json() -> serde_json::Value {
    let catalog = build_catalog();
    serde_json::json!({
        "categories": category_labels(),
        "algorithms": catalog,
    })
}

pub fn get_algorithm_meta_json() -> serde_json::Value {
    let catalog = build_catalog();
    let metas: Vec<&AlgorithmMeta> = catalog.iter().map(|e| &e.meta).collect();
    serde_json::to_value(metas).unwrap_or_default()
}

pub fn get_code_sample_json(algorithm: &str) -> Option<String> {
    build_catalog()
        .iter()
        .find(|e| e.algorithm == algorithm)
        .map(|e| serde_json::to_string(&e.samples).unwrap_or_default())
}

pub fn get_single_code_sample(algorithm: &str, lang: &str) -> Option<String> {
    let catalog = build_catalog();
    let entry = catalog.iter().find(|e| e.algorithm == algorithm)?;
    let lines = match lang {
        "cpp" => &entry.samples.cpp.lines,
        "python" => &entry.samples.python.lines,
        "rust" => &entry.samples.rust.lines,
        _ => return None,
    };
    Some(
        serde_json::to_string(&serde_json::json!({
            "algorithm": algorithm,
            "language": lang,
            "lines": lines,
            "lineMap": entry.line_map,
        }))
        .unwrap_or_default(),
    )
}

pub fn get_line_map(algorithm: &str) -> Option<LineMap> {
    build_catalog()
        .iter()
        .find(|e| e.algorithm == algorithm)
        .map(|e| e.line_map.clone())
}

pub fn get_meta(algorithm: &str) -> Option<AlgorithmMeta> {
    build_catalog()
        .iter()
        .find(|e| e.algorithm == algorithm)
        .map(|e| e.meta.clone())
}

pub fn get_default_values(algorithm: &str) -> Vec<i32> {
    match algorithm {
        // Sorting
        "selection-sort" | "bubble-sort" | "insertion-sort" => vec![64, 25, 12, 22, 11, 89, 34, 55],
        "merge-sort" => vec![38, 27, 43, 3, 9, 82, 10],
        "quick-sort" => vec![10, 7, 8, 9, 1, 5],
        "heap-sort" => vec![12, 11, 13, 5, 6, 7],
        "shell-sort" => vec![12, 34, 54, 2, 3, 7, 8, 99],
        "counting-sort" => vec![3, 1, 4, 1, 5, 9, 2, 6],
        "bucket-sort" => vec![78, 17, 39, 26, 72, 94, 21, 12],
        "radix-sort" => vec![170, 45, 75, 90, 802, 24, 2, 66],

        // Searching
        "linear-search" => vec![3, 1, 4, 1, 5, 9, 2, 6],
        "binary-search" | "interpolation-search" => vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        "hash-search" => vec![10, 20, 30, 40, 50],

        // Linear
        "array-insert" => vec![1, 2, 3, 4, 5, 99],
        "array-delete" | "linked-list-traverse" => vec![10, 20, 30, 40, 50],

        // Stack/Queue
        "stack-push-pop" | "queue-enqueue-dequeue" => vec![1, 2, 3, 4, 5],

        // Recursive
        "factorial" => vec![5],
        "fibonacci" => vec![8],
        "tower-of-hanoi" => vec![3],

        // Tree
        "bst-insert" | "bst-search" => vec![50, 30, 70, 20, 40, 60, 80],
        "heap-insert" => vec![10, 20, 5, 30, 15],
        "avl-insert" => vec![50, 30, 70, 20, 40, 60, 80],
        "bst-preorder" | "bst-inorder" | "bst-postorder" | "bst-levelorder" => {
            vec![1, 2, 3, 4, 5, 6, 7]
        }

        // Graph
        "bfs" | "dfs" => vec![1, 2, 3, 4, 5, 6],
        "dijkstra" => vec![0, 4, 2, 7, 1, 5],
        "topological-sort" => vec![1, 2, 3, 4, 5],
        "kruskal" | "prim" => vec![4, 8, 1, 3, 7],

        // DP
        "fibonacci-dp" => vec![8],
        "knapsack" => vec![10, 2, 3, 4, 5],
        "lcs" => vec![5, 5],
        "lis" => vec![10, 9, 2, 5, 3, 7, 101, 18],

        _ => vec![1, 2, 3, 4, 5],
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_algorithms_have_meta() {
        let catalog = build_catalog();
        assert_eq!(catalog.len(), 40, "catalog should have 40 algorithms");
        for entry in &catalog {
            assert!(
                !entry.meta.name.is_empty(),
                "{} missing name",
                entry.algorithm
            );
            assert!(
                !entry.meta.description.is_empty(),
                "{} missing description",
                entry.algorithm
            );
        }
    }

    #[test]
    fn test_all_algorithms_have_three_language_samples() {
        for entry in build_catalog() {
            assert!(
                !entry.samples.cpp.lines.is_empty(),
                "{} missing C++ code",
                entry.algorithm
            );
            assert!(
                !entry.samples.python.lines.is_empty(),
                "{} missing Python code",
                entry.algorithm
            );
            assert!(
                !entry.samples.rust.lines.is_empty(),
                "{} missing Rust code",
                entry.algorithm
            );
        }
    }

    #[test]
    fn test_all_algorithms_have_line_maps() {
        for entry in build_catalog() {
            assert!(
                !entry.line_map.is_empty(),
                "{} missing lineMap",
                entry.algorithm
            );
            let done = entry.line_map.get("done");
            assert!(
                done.is_some(),
                "{} lineMap missing 'done' key",
                entry.algorithm
            );
            let lines = done.unwrap();
            assert!(
                !lines.is_empty(),
                "{} 'done' has empty line list",
                entry.algorithm
            );
        }
    }

    #[test]
    fn test_algorithms_match_visualizer_core() {
        let catalog = build_catalog();
        for entry in &catalog {
            let id = visualizer_core::AlgorithmId::from_name(&entry.algorithm);
            assert!(
                id.is_some(),
                "algorithm '{}' not found in visualizer-core AlgorithmId",
                entry.algorithm
            );
        }
    }
}
