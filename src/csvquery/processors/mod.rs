mod pipeline;
mod processor;
mod processor_merge;

pub use pipeline::Pipeline;
pub use processor::{IProcessor, ProcessorRef};
pub use processor_merge::{MergeProcessor};
