use crate::types::{StepType, TraceStep, VisualItem, build_initial_items};

pub fn tower_of_hanoi_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let n = items.first().map(|it| it.value).unwrap_or(3).clamp(1, 6) as usize;

    steps.push(
        TraceStep::new(StepType::Start, "hanoi")
            .with_items(items.clone())
            .with_extra(serde_json::json!({ "n": n }))
            .with_note(format!("汉诺塔：将 {} 个盘子从 A 移动到 C", n)),
    );

    hanoi(n, 'A', 'C', 'B', &items, &mut steps);

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_extra(serde_json::json!({ "n": n }))
            .with_note("汉诺塔完成"),
    );

    steps
}

fn hanoi(
    n: usize,
    from: char,
    to: char,
    aux: char,
    items: &[VisualItem],
    steps: &mut Vec<TraceStep>,
) {
    if n == 0 {
        return;
    }
    hanoi(n - 1, from, aux, to, items, steps);
    steps.push(
        TraceStep::new(StepType::Select, "move")
            .with_items(items.to_vec())
            .with_extra(serde_json::json!({
                "disk": n,
                "from": from.to_string(),
                "to": to.to_string()
            }))
            .with_note(format!("将盘子 {} 从 {} 移动到 {}", n, from, to)),
    );
    hanoi(n - 1, aux, to, from, items, steps);
}
