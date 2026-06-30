use crate::types::{build_initial_items, StepType, TraceStep};

pub fn interpolation_search_trace(values: &[i32]) -> Vec<TraceStep> {
    let mut steps = Vec::new();
    let mut items = build_initial_items(values);
    items.sort_by_key(|it| it.value);
    let target = items.first().map(|it| it.value).unwrap_or(0);

    if items.is_empty() {
        steps.push(TraceStep::new(StepType::Start, "interpolation-search").with_note("数组为空"));
        steps.push(TraceStep::new(StepType::Done, "done").with_note("插值查找结束"));
        return steps;
    }

    steps.push(
        TraceStep::new(StepType::Start, "interpolation-search")
            .with_items(items.clone())
            .with_extra(serde_json::json!({"target":target}))
            .with_note(format!("插值查找 {}", target)),
    );

    let mut lo = 0usize;
    let mut hi = items.len() - 1;

    while lo <= hi && target >= items[lo].value && target <= items[hi].value {
        if lo == hi {
            if items[lo].value == target {
                steps.push(
                    TraceStep::new(StepType::Select, "found")
                        .with_items(items.clone())
                        .with_active(vec![items[lo].id])
                        .with_note(format!("在位置 {} 找到 {}", lo, target)),
                );
                break;
            }
            break;
        }

        let pos = lo
            + (((target - items[lo].value) as usize * (hi - lo))
                / ((items[hi].value - items[lo].value) as usize).max(1));

        steps.push(
            TraceStep::new(StepType::Compare, "probe")
                .with_items(items.clone())
                .with_active(vec![items[pos].id])
                .with_range((lo, hi))
                .with_extra(serde_json::json!({"target":target,"pos":pos}))
                .with_note(format!("探测位置 {}, 值 {}", pos, items[pos].value)),
        );

        if items[pos].value == target {
            steps.push(
                TraceStep::new(StepType::Select, "found")
                    .with_items(items.clone())
                    .with_active(vec![items[pos].id])
                    .with_note(format!("在位置 {} 找到 {}", pos, target)),
            );
            break;
        } else if items[pos].value < target {
            lo = pos + 1;
        } else {
            hi = pos.saturating_sub(1);
        }
    }

    steps.push(
        TraceStep::new(StepType::Done, "done")
            .with_items(items)
            .with_extra(serde_json::json!({"target":target}))
            .with_note("插值查找结束"),
    );
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty() {
        let steps = interpolation_search_trace(&[]);
        assert!(!steps.is_empty());
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
    #[test]
    fn test_found() {
        let steps = interpolation_search_trace(&[10, 20, 30, 40, 50]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
        assert!(steps.iter().any(|s| s.note.contains("找到")));
    }
    #[test]
    fn test_single() {
        let steps = interpolation_search_trace(&[42]);
        assert_eq!(steps.last().unwrap().step_type, StepType::Done);
    }
}
