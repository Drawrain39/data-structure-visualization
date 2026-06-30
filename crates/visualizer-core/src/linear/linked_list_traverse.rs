use crate::types::{StepType, TraceStep, build_initial_items};

pub fn linked_list_traverse_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);

    steps.push(
        TraceStep::new(StepType::Start, "traverse")
            .with_items(items.clone())
            .with_note("遍历链表"),
    );

    for (i, item) in items.iter().enumerate() {
        steps.push(
            TraceStep::new(StepType::Visit, "visit")
                .with_items(items.clone())
                .with_active(vec![item.id])
                .with_extra(serde_json::json!({ "index": i }))
                .with_note(format!("访问第 {} 个节点, 值 {}", i, item.value)),
        );
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_note("遍历完成"),
    );

    steps
}
