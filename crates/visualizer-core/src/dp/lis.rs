use crate::types::{build_initial_items, StepType, TraceStep, VisualItem};

pub fn lis_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);
    let n = items.len();

    steps.push(
        TraceStep::new(StepType::Start, "lis")
            .with_items(items.clone())
            .with_note("最长递增子序列 (DP)"),
    );

    if n == 0 {
        steps.push(TraceStep::new(StepType::Done, "done").with_note("数组为空"));
        return steps;
    }

    let mut dp = vec![1usize; n];
    steps.push(
        TraceStep::new(StepType::Select, "init")
            .with_items(items.clone())
            .with_extra(serde_json::json!({"dp":dp.clone()}))
            .with_note("dp[i] 初始化为 1"),
    );

    for i in 0..n {
        for j in 0..i {
            steps.push(
                TraceStep::new(StepType::Compare, "compare")
                    .with_items(items.clone())
                    .with_comparing(vec![items[i].id, items[j].id])
                    .with_extra(serde_json::json!({"i":i,"j":j,"dp_i":dp[i],"dp_j":dp[j]}))
                    .with_note(format!(
                        "比较 {} 和 {}: dp[{}]={}, dp[{}]={}",
                        items[i].value, items[j].value, i, dp[i], j, dp[j]
                    )),
            );

            if items[j].value < items[i].value && dp[j] + 1 > dp[i] {
                dp[i] = dp[j] + 1;
                steps.push(
                    TraceStep::new(StepType::Select, "update")
                        .with_items(items.clone())
                        .with_active(vec![items[i].id])
                        .with_extra(serde_json::json!({"i":i,"dp_i":dp[i],"from_j":j}))
                        .with_note(format!("更新 dp[{}] = {}", i, dp[i])),
                );
            }
        }
    }

    let max_len = dp.iter().copied().max().unwrap_or(0);
    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_extra(serde_json::json!({"dp":dp,"result":max_len}))
            .with_note(format!("LIS 长度 = {}", max_len)),
    );
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let steps = lis_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
    #[test]
    fn test_single() {
        let steps = lis_trace(&[42]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
    #[test]
    fn test_basic() {
        let steps = lis_trace(&[10, 9, 2, 5, 3, 7, 101, 18]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        assert_eq!(steps.last().unwrap().extra["result"], 4);
    }
}
