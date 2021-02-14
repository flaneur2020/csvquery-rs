use crate::csvquery::plans::PlanNodeRef;
use crate::csvquery::processors::Pipeline;

pub struct PipelineBuilder {
    plan: PlanNodeRef,
    pipeline: Pipeline,
}

impl PipelineBuilder {
    fn new(plan: PlanNodeRef) -> Self {
        Self {
            plan: plan,
            pipeline: Pipeline::new(),
        }
    }

    fn build(&self) -> Pipeline {
        self.pipeline
    }
}
