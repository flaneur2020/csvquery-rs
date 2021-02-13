use crate::csvquery::data_schema::DataSchemaRef;
use crate::csvquery::logical_plans::{AggregatePlan, ProjectionPlan, ScanPlan, SelectionPlan};
use std::sync::Arc;

pub type PlanNodeRef = Arc<PlanNode>;

pub enum PlanNode {
    ScanPlan(ScanPlan),
    ProjectionPlan(ProjectionPlan),
    SelectionPlan(SelectionPlan),
    AggregatePlan(AggregatePlan),
}

impl PlanNode {
    pub fn schema(&self) -> DataSchemaRef {
        match self {
            PlanNode::ScanPlan(plan) => plan.schema(),
            PlanNode::ProjectionPlan(plan) => plan.schema(),
            PlanNode::SelectionPlan(plan) => plan.schema(),
            PlanNode::AggregatePlan(plan) => plan.schema(),
        }
    }
}
