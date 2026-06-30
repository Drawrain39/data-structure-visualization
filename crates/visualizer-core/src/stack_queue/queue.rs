use crate::types::{StepType, TraceStep, build_initial_items};

pub fn queue_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);

    steps.push(
        TraceStep::new(StepType::Start, "queue")
            .with_items(items.clone())
            .with_note("队列操作演示"),
    );

    let mut queue: Vec<usize> = Vec::new();
    for (i, item) in items.iter().enumerate() {
        queue.push(item.id);
        steps.push(
            TraceStep::new(StepType::Enqueue, "enqueue")
                .with_items(items.clone())
                .with_active(queue.clone())
                .with_extra(serde_json::json!({ "rear": item.id }))
                .with_note(format!("入队: {}", item.value)),
        );

        if i == items.len() - 1 {
            while let Some(front) = queue.first().copied() {
                queue.remove(0);
                steps.push(
                    TraceStep::new(StepType::Dequeue, "dequeue")
                        .with_items(items.clone())
                        .with_active(vec![front])
                        .with_extra(serde_json::json!({ "front": front }))
                        .with_note(format!("出队: {}", items[front].value)),
                );
            }
        }
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_note("队列操作完成"),
    );

    steps
}
