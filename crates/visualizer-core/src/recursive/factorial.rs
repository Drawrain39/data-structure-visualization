use crate::types::{build_initial_items, StepType, TraceStep};

pub fn factorial_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let n = items.first().map(|it| it.value).unwrap_or(5).clamp(0, 10) as usize;

    steps.push(
        TraceStep::new(StepType::Start, "factorial")
            .with_items(items.clone())
            .with_extra(serde_json::json!({ "n": n }))
            .with_note(format!("计算 {}!", n)),
    );

    let mut result = 1usize;
    for i in 1..=n {
        result *= i;
        steps.push(
            TraceStep::new(StepType::Select, "multiply")
                .with_items(items.clone())
                .with_extra(serde_json::json!({ "i": i, "result": result }))
                .with_note(format!("{}! = {}", i, result)),
        );
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_extra(serde_json::json!({ "n": n, "result": result }))
            .with_note(format!("{}! = {}", n, result)),
    );

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero() {
        let steps = factorial_trace(&[0]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_five() {
        let steps = factorial_trace(&[5]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }

    #[test]
    fn test_empty() {
        let steps = factorial_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
}
