use crate::types::{StepType, TraceStep, build_initial_items};

pub fn fibonacci_dp_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let n = items.first().map(|it| it.value).unwrap_or(8).clamp(0, 20) as usize;

    steps.push(TraceStep::new(StepType::Start, "fib-dp").with_items(items.clone()).with_extra(serde_json::json!({"n":n})).with_note(format!("DP 计算 fibonacci({})", n)));

    if n <= 1 {
        steps.push(TraceStep::new(StepType::Done, "done").with_items(items).with_extra(serde_json::json!({"n":n,"result":n})).with_note(format!("F({}) = {}", n, n)));
        return steps;
    }

    let mut dp = vec![0usize; n + 1];
    dp[1] = 1;

    steps.push(TraceStep::new(StepType::Select, "init").with_items(items.clone()).with_extra(serde_json::json!({"dp":[0,1]})).with_note("dp[0]=0, dp[1]=1"));

    let display_items: Vec<_> = dp.iter().enumerate().map(|(i,&v)| crate::types::VisualItem::new(i, v as i32)).collect();

    for i in 2..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
        steps.push(TraceStep::new(StepType::Select, "compute").with_items(display_items.clone()).with_active(vec![i as usize]).with_extra(serde_json::json!({"i":i,"val":dp[i],"dp":dp[..=i].to_vec()})).with_note(format!("dp[{}] = dp[{}] + dp[{}] = {}", i, i-1, i-2, dp[i])));
    }

    let final_items: Vec<_> = dp.iter().enumerate().map(|(i,&v)| crate::types::VisualItem::new(i, v as i32)).collect();
    steps.push(TraceStep::new(StepType::Done, "done").with_items(final_items).with_sorted((0..=n).collect()).with_extra(serde_json::json!({"n":n,"result":dp[n]})).with_note(format!("F({}) = {}", n, dp[n])));
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_zero() { let steps = fibonacci_dp_trace(&[0]); assert_eq!(steps.last().unwrap().step_type, StepType::Done); }
    #[test] fn test_one() { let steps = fibonacci_dp_trace(&[1]); assert_eq!(steps.last().unwrap().step_type, StepType::Done); }
    #[test] fn test_eight() { let steps = fibonacci_dp_trace(&[8]); assert_eq!(steps.last().unwrap().step_type, StepType::Done); let extra = &steps.last().unwrap().extra; assert_eq!(extra["result"], 21); }
}
