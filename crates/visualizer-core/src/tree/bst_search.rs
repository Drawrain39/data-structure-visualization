use crate::types::{StepType, TraceStep, VisualItem, build_initial_items};

pub fn bst_search_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let target = items.first().map(|it| it.value).unwrap_or(0);

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
