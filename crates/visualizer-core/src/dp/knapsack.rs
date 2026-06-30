use crate::types::{StepType, TraceStep, VisualItem, build_initial_items};

pub fn knapsack_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);

    let capacity = items.first().map(|it| it.value as usize).unwrap_or(10).max(1);
    let n = items.len().saturating_sub(1);

    steps.push(TraceStep::new(StepType::Start, "knapsack").with_items(items.clone()).with_extra(serde_json::json!({"capacity":capacity,"items":n})).with_note(format!("0/1 背包: 容量={}, 物品数={}", capacity, n)));

    if n == 0 {
        steps.push(TraceStep::new(StepType::Done, "done").with_items(items).with_extra(serde_json::json!({"result":0})).with_note("无物品，最大价值=0"));
        return steps;
    }

    // items from values: weights = items[1..], values = same (for demo, w=v)
    let mut dp = vec![vec![0i32; capacity + 1]; n + 1];

    for i in 1..=n {
        let w = items[i].value;
        let v = w; // For demo, value = weight
        for j in 0..=capacity {
            if w as usize <= j {
                dp[i][j] = dp[i-1][j].max(dp[i-1][j - w as usize] + v);
            } else {
                dp[i][j] = dp[i-1][j];
            }
            if j % (capacity.max(1) / 3).max(1) == 0 {
                steps.push(TraceStep::new(StepType::Select, "dp-update").with_items(items.clone()).with_active(vec![items[i].id]).with_extra(serde_json::json!({"i":i,"j":j,"val":dp[i][j],"capacity":capacity})).with_note(format!("dp[{}][{}] = {}", i, j, dp[i][j])));
            }
        }
    }

    let result = dp[n][capacity];
    steps.push(TraceStep::new(StepType::Done, "done").with_items(items).with_extra(serde_json::json!({"capacity":capacity,"result":result})).with_note(format!("最大价值 = {}", result)));
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_empty() { let steps = knapsack_trace(&[]); assert!(!steps.is_empty()); assert_eq!(steps.last().unwrap().step_type, StepType::Done); }
    #[test] fn test_basic() { let steps = knapsack_trace(&[10, 2, 3, 4, 5]); assert_eq!(steps.last().unwrap().step_type, StepType::Done); }
}
