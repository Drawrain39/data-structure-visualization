use crate::types::{StepType, TraceStep, build_initial_items};

pub fn fibonacci_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let n = items.first().map(|it| it.value).unwrap_or(6).clamp(0, 15) as usize;

    steps.push(
        TraceStep::new(StepType::Start, "fibonacci")
            .with_items(items.clone())
            .with_extra(serde_json::json!({ "n": n }))
            .with_note(format!("计算斐波那契数列第 {} 项", n)),
    );

    let mut memo = vec![0usize; n + 1];
    if n >= 1 {
        memo[1] = 1;
    }

    for i in 2..=n {
        memo[i] = memo[i - 1] + memo[i - 2];
        steps.push(
            TraceStep::new(StepType::Select, "compute")
                .with_items(items.clone())
                .with_extra(serde_json::json!({ "i": i, "value": memo[i] }))
                .with_note(format!("F({}) = F({}) + F({}) = {}", i, i - 1, i - 2, memo[i])),
        );
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_extra(serde_json::json!({ "n": n, "result": memo[n] }))
            .with_note(format!("F({}) = {}", n, memo[n])),
    );

    steps
}
