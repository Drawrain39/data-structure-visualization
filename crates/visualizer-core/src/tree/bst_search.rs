use crate::types::{StepType, TraceStep, VisualItem, build_initial_items};

pub fn bst_search_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let target = items.first().map(|it| it.value).unwrap_or(0);

    if items.is_empty() {
        steps.push(
            TraceStep::new(StepType::Start, "bst-search")
                .with_extra(serde_json::json!({ "target": target }))
                .with_note("树为空"),
        );
        steps.push(
            TraceStep::new(StepType::Done, "done")
                .with_extra(serde_json::json!({ "target": target }))
                .with_note("查找结束"),
        );
        return steps;
    }

    steps.push(
        TraceStep::new(StepType::Start, "bst-search")
            .with_items(items.clone())
            .with_extra(serde_json::json!({ "target": target }))
            .with_note(format!("在二叉搜索树中查找 {}", target)),
    );

    // Use sorted values to mimic BST traversal
    let sorted: Vec<&VisualItem> = items.iter().collect();
    let mut left = 0usize;
    let mut right = sorted.len().saturating_sub(1);

    while left <= right {
        let mid = left + (right - left) / 2;
        let mid_id = sorted[mid].id;

        steps.push(
            TraceStep::new(StepType::Compare, "compare")
                .with_items(items.clone())
                .with_comparing(vec![mid_id])
                .with_range((left, right))
                .with_extra(serde_json::json!({ "target": target }))
                .with_note(format!("比较节点 {} 和 {}", sorted[mid].value, target)),
        );

        if sorted[mid].value == target {
            steps.push(
                TraceStep::new(StepType::Select, "found")
                    .with_items(items.clone())
                    .with_active(vec![mid_id])
                    .with_extra(serde_json::json!({ "target": target }))
                    .with_note(format!("找到 {}", target)),
            );
            break;
        } else if sorted[mid].value < target {
            left = mid + 1;
        } else {
            right = mid.saturating_sub(1);
        }
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_extra(serde_json::json!({ "target": target }))
            .with_note("查找结束"),
    );

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let steps = bst_search_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_search() {
        let steps = bst_search_trace(&[50, 30, 70, 20, 40, 60, 80]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        assert!(steps.iter().any(|s| s.step_type == StepType::Compare));
    }
}
