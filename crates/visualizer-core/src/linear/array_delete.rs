use crate::types::{StepType, TraceStep, build_initial_items};

pub fn array_delete_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let delete_index = values.first().copied().unwrap_or(0) as usize % items.len().max(1);

    steps.push(
        TraceStep::new(StepType::Start, "delete")
            .with_items(items.clone())
            .with_extra(serde_json::json!({ "index": delete_index }))
            .with_note(format!("删除数组位置 {} 的元素", delete_index)),
    );

    if delete_index < items.len() {
        steps.push(
            TraceStep::new(StepType::Select, "select")
                .with_items(items.clone())
                .with_active(vec![items[delete_index].id])
                .with_extra(serde_json::json!({ "index": delete_index }))
                .with_note(format!("选中待删除元素 {}", items[delete_index].value)),
        );
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_note("删除完成"),
    );

    steps
}
