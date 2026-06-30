use crate::types::{StepType, TraceStep, build_initial_items};

pub fn array_insert_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let insert_value = values.last().copied().unwrap_or(0);

    steps.push(
        TraceStep::new(StepType::Start, "insert")
            .with_items(items.clone())
            .with_extra(serde_json::json!({ "value": insert_value }))
            .with_note(format!("在数组末尾插入 {}", insert_value)),
    );

    steps.push(
        TraceStep::new(StepType::Overwrite, "append")
            .with_items(items.clone())
            .with_extra(serde_json::json!({ "value": insert_value }))
            .with_note(format!("将 {} 追加到数组末尾", insert_value)),
    );

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_note("插入完成"),
    );

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let steps = array_insert_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_insert() {
        let steps = array_insert_trace(&[1, 2, 3, 99]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
}
