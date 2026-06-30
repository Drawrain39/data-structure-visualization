use crate::types::{build_initial_items, StepType, TraceStep, VisualItem};

pub fn lcs_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let items = build_initial_items(values);

    let m = items
        .first()
        .map(|it| it.value as usize)
        .unwrap_or(5)
        .clamp(1, 10);
    let n = items
        .get(1)
        .map(|it| it.value as usize)
        .unwrap_or(5)
        .clamp(1, 10);

    steps.push(
        TraceStep::new(StepType::Start, "lcs")
            .with_items(items.clone())
            .with_extra(serde_json::json!({"m":m,"n":n}))
            .with_note(format!("最长公共子序列: 长度A={}, 长度B={}", m, n)),
    );

    // Generate two sequences from remaining values
    let a: Vec<i32> = if items.len() >= 2 + m {
        items[2..2 + m].iter().map(|it| it.value).collect()
    } else {
        (1..=m as i32).collect()
    };
    let b: Vec<i32> = if items.len() >= 2 + m + n {
        items[2 + m..2 + m + n].iter().map(|it| it.value).collect()
    } else {
        (1..=n as i32).map(|x| x * 2 % 5 + 1).collect()
    };

    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    let display_items: Vec<VisualItem> = dp
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, &v)| VisualItem::new(i * (n + 1) + j, v as i32))
        })
        .collect();

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
            if i % m.max(1) == 0 || j % n.max(1) == 0 {
                steps.push(
                    TraceStep::new(StepType::Select, "dp-cell")
                        .with_items(display_items.clone())
                        .with_active(vec![i * (n + 1) + j])
                        .with_extra(serde_json::json!({"i":i,"j":j,"val":dp[i][j]}))
                        .with_note(format!("dp[{}][{}] = {}", i, j, dp[i][j])),
                );
            }
        }
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(display_items)
            .with_extra(serde_json::json!({"m":m,"n":n,"result":dp[m][n]}))
            .with_note(format!("LCS 长度 = {}", dp[m][n])),
    );
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let steps = lcs_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
    #[test]
    fn test_basic() {
        let steps = lcs_trace(&[5, 5]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
}
