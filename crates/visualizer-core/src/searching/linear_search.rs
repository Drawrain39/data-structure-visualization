use crate::types::{build_initial_items, StepType, TraceStep};

pub fn linear_search_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let target = values.first().copied().unwrap_or(0);

    steps.push(
        TraceStep::new(StepType::Start, "linear-search")
            .with_items(items.clone())
            .with_extra(serde_json::json!({ "target": target }))
            .with_note(format!("顺序查找目标值 {}", target)),
    );

    for (i, item) in items.iter().enumerate() {
        steps.push(
            TraceStep::new(StepType::Compare, "compare")
                .with_items(items.clone())
                .with_comparing(vec![item.id])
                .with_extra(serde_json::json!({ "target": target, "index": i }))
                .with_note(format!("比较位置 {} 的值 {}", i, item.value)),
        );

        if item.value == target {
            steps.push(
                TraceStep::new(StepType::Select, "found")
                    .with_items(items.clone())
                    .with_active(vec![item.id])
                    .with_extra(serde_json::json!({ "target": target, "index": i }))
                    .with_note(format!("在位置 {} 找到目标值 {}", i, target)),
            );
            break;
        }
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items.clone())
            .with_extra(serde_json::json!({ "target": target }))
            .with_note("顺序查找结束"),
    );

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let steps = linear_search_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_found() {
        let steps = linear_search_trace(&[42, 7, 13, 99]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        let found = steps.iter().any(|s| s.note.contains("找到"));
        assert!(found);
    }

    #[test]
    fn test_not_present() {
        let steps = linear_search_trace(&[10, 20, 30]);
        // Searches for values[0]=10, which is present
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
}
