use crate::types::{build_initial_items, StepType, TraceStep};
use std::collections::HashMap;

pub fn hash_search_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let target = items.first().map(|it| it.value).unwrap_or(0);

    steps.push(
        TraceStep::new(StepType::Start, "hash-search")
            .with_items(items.clone())
            .with_extra(serde_json::json!({"target":target}))
            .with_note(format!("哈希查找 {}", target)),
    );

    let mut map: HashMap<i32, usize> = HashMap::new();
    for (i, item) in items.iter().enumerate() {
        map.insert(item.value, i);
        steps.push(
            TraceStep::new(StepType::Push, "build")
                .with_items(items.clone())
                .with_active(vec![item.id])
                .with_extra(serde_json::json!({"hash":item.value,"index":i}))
                .with_note(format!("hash[{}] = {}", item.value, i)),
        );
    }

    steps.push(
        TraceStep::new(StepType::Compare, "query")
            .with_items(items.clone())
            .with_extra(serde_json::json!({"target":target}))
            .with_note(format!("查询 hash[{}]", target)),
    );

    if let Some(&idx) = map.get(&target) {
        steps.push(
            TraceStep::new(StepType::Select, "found")
                .with_items(items.clone())
                .with_active(vec![items[idx].id])
                .with_extra(serde_json::json!({"target":target,"index":idx}))
                .with_note(format!("在位置 {} 找到 {}", idx, target)),
        );
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_extra(serde_json::json!({"target":target}))
            .with_note("哈希查找结束"),
    );
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let steps = hash_search_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
    #[test]
    fn test_found() {
        let steps = hash_search_trace(&[10, 20, 30, 40, 50]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        assert!(steps.iter().any(|s| s.note.contains("找到")));
    }
}
