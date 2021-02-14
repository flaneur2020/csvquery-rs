use crate::csvquery::data_schema::DataSchemaRef;
use crate::csvquery::error::{CSVQueryError, CSVQueryResult};
use crate::csvquery::plans::{AggregatePlan, ProjectionPlan, ScanPlan, SelectionPlan};
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

    pub fn list_until_bottom(&self) -> CSVQueryResult<(Vec<PlanNode>, PlanNode)> {
        let max_depth = 128;
        let list: Vec<PlanNode> = vec![];
        let mut plan = self.clone();

        loop {
            if list.len() > max_depth {
                return Err(CSVQueryError::Internal(format!(
                    "PlanNode depth exceed {}",
                    max_depth
                )));
            }

            match plan {
                PlanNode::ScanPlan(v) => {
                    list.push(*plan.clone());
                    break;
                }
                PlanNode::ProjectionPlan(v) => {
                    list.push(*plan.clone());
                    plan = v.input.as_ref().clone();
                }
                PlanNode::SelectionPlan(v) => {
                    list.push(*plan.clone());
                    plan = v.input.as_ref().clone();
                }
                PlanNode::AggregatePlan(v) => {
                    list.push(*plan.clone());
                    plan = v.input.as_ref().clone();
                }
            }
        }

        list.reverse();
        Ok((list, *plan.clone()))
    }
}
