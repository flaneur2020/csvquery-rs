use crate::csvquery::plans::{PlanNodeRef};

pub struct PipelineBuilder {
    plan: PlanNodeRef,
}

impl PipelineBuilder {
    fn new(plan: PlanNodeRef) -> Self {
        Self { plan }
    }
}