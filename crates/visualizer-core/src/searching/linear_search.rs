use crate::types::{StepType, TraceStep, build_initial_items};

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
