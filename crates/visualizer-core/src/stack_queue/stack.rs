use crate::types::{StepType, TraceStep, build_initial_items};

pub fn stack_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);

    steps.push(
        TraceStep::new(StepType::Start, "stack")
            .with_items(items.clone())
            .with_note("栈操作演示"),
    );

    let mut stack: Vec<usize> = Vec::new();
    for (i, item) in items.iter().enumerate() {
        stack.push(item.id);
        steps.push(
            TraceStep::new(StepType::Push, "push")
                .with_items(items.clone())
                .with_active(stack.clone())
                .with_extra(serde_json::json!({ "top": item.id }))
                .with_note(format!("入栈: {}", item.value)),
        );

        if i == items.len() - 1 {
            while let Some(top) = stack.pop() {
                steps.push(
                    TraceStep::new(StepType::Pop, "pop")
                        .with_items(items.clone())
                        .with_active(vec![top])
                        .with_extra(serde_json::json!({ "top": top }))
                        .with_note(format!("出栈: {}", items[top].value)),
                );
            }
        }
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_note("栈操作完成"),
    );

    steps
}
