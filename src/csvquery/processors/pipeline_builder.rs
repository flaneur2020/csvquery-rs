use crate::csvquery::plans::PlanNode;
use crate::csvquery::processors::Pipeline;
use crate::csvquery::error::CSVQueryResult;

pub struct PipelineBuilder {
    plan: PlanNode,
    pipeline: Pipeline,
}

impl PipelineBuilder {
    fn new(plan: PlanNode) -> Self {
        Self {
            plan: plan,
            pipeline: Pipeline::new(),
        }
    }

    fn build(&self) -> CSVQueryResult<Pipeline> {
        let (plan_items, _) = self.plan.list_until_bottom()?;
        for plan in plan_items {
            match plan {
                PlanNode::ScanPlan(v) => {
                }
            }
        }
        Ok(self.pipeline)
    }
}
