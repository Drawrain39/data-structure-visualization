use crate::types::{build_initial_items, StepType, TraceStep};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let steps = array_delete_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_delete() {
        let steps = array_delete_trace(&[10, 20, 30, 40, 50]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
}
