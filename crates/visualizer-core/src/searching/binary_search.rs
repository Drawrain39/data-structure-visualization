use crate::types::{StepType, TraceStep, build_initial_items};

pub fn binary_search_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let mut items = build_initial_items(values);
    items.sort_by_key(|it| it.value);
    let target = items.first().map(|it| it.value).unwrap_or(0);

    if items.is_empty() {
        steps.push(
            TraceStep::new(StepType::Start, "binary-search")
                .with_extra(serde_json::json!({ "target": target }))
                .with_note("数组为空"),
        );
        steps.push(
            TraceStep::new(StepType::Done, "done")
                .with_extra(serde_json::json!({ "target": target }))
                .with_note("二分查找结束"),
        );
        return steps;
    }

    steps.push(
        TraceStep::new(StepType::Start, "binary-search")
            .with_items(items.clone())
            .with_extra(serde_json::json!({ "target": target }))
            .with_note(format!("在有序数组中二分查找 {}", target)),
    );

    let mut left = 0usize;
    let mut right = items.len().saturating_sub(1);

    while left <= right {
        let mid = left + (right - left) / 2;
        steps.push(
            TraceStep::new(StepType::Select, "mid")
                .with_items(items.clone())
                .with_active(vec![items[mid].id])
                .with_range((left, right))
                .with_extra(serde_json::json!({ "target": target, "mid": mid }))
                .with_note(format!("中间位置 {}, 值 {}", mid, items[mid].value)),
        );

        steps.push(
            TraceStep::new(StepType::Compare, "compare")
                .with_items(items.clone())
                .with_comparing(vec![items[mid].id])
                .with_range((left, right))
                .with_extra(serde_json::json!({ "target": target, "mid": mid }))
                .with_note(format!("比较 {} 和 {}", items[mid].value, target)),
        );

        if items[mid].value == target {
            steps.push(
                TraceStep::new(StepType::Select, "found")
                    .with_items(items.clone())
                    .with_active(vec![items[mid].id])
                    .with_extra(serde_json::json!({ "target": target, "index": mid }))
                    .with_note(format!("在位置 {} 找到 {}", mid, target)),
            );
            break;
        } else if items[mid].value < target {
            left = mid + 1;
        } else {
            right = mid.saturating_sub(1);
        }
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_extra(serde_json::json!({ "target": target }))
            .with_note("二分查找结束"),
    );

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let steps = binary_search_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_found() {
        let steps = binary_search_trace(&[1, 3, 5, 7, 9]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        let found = steps.iter().any(|s| s.note.contains("找到"));
        assert!(found);
    }

    #[test]
    fn test_single() {
        let steps = binary_search_trace(&[42]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
}
