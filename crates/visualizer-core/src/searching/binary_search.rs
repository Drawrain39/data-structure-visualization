use crate::types::{StepType, TraceStep, build_initial_items};

pub fn binary_search_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let mut items = build_initial_items(values);
    items.sort_by_key(|it| it.value);
    let target = items.first().map(|it| it.value).unwrap_or(0);

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
