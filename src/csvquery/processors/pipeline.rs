use crate::csvquery::processors::ProcessorRef;

pub type Pipe = Vec<ProcessorRef>;

pub struct Pipeline {
    processors: Vec<Pipe>,
}

impl Pipeline {
    pub fn new() -> Self {
        Pipeline { processors: vec![] }
    }

    pub fn add_source(&mut self, processor: ProcessorRef) {
    }

    pub fn add_simple_transform(&mut self, processor: ProcessorRef) {
    }
}