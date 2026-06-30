pub mod queue;
pub mod stack;

use crate::types::{AlgorithmId, TraceStep};

pub fn generate_trace(algorithm: &str, values: &[i32]) -> Result<Vec<TraceStep>, String> {
    let id = AlgorithmId::from_name(algorithm)
        .ok_or_else(|| format!("unknown algorithm: {}", algorithm))?;

    Ok(match id {
        AlgorithmId::StackPushPop => stack::stack_trace(values),
        AlgorithmId::QueueEnqueueDequeue => queue::queue_trace(values),
        _ => {
            return Err(format!(
                "algorithm {} is not a stack/queue algorithm",
                algorithm
            ))
        }
    })
}
