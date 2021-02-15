use crate::csvquery::data_types::DataSchemaRef;
use crate::csvquery::error::{CSVQueryError, CSVQueryResult};
use crate::csvquery::plans::{AggregatePlan, ProjectionPlan, ScanPlan, SelectionPlan, PlanVisitor};
use std::sync::Arc;
use std::fmt;

pub type PlanNodeRef = Arc<PlanNode>;

#[derive(Clone)]
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

    pub fn visit<V: PlanVisitor>(
        &self,
        visitor: &mut V
    ) -> std::result::Result<bool, V::Error> {
        if !visitor.pre_visit(self)? {
            return Ok(false);
        }

        let recurse = match self {
            PlanNode::ProjectionPlan(plan) => plan.input.visit(visitor)?,
            PlanNode::SelectionPlan(plan) => plan.input.visit(visitor)?,
            PlanNode::AggregatePlan(plan) => plan.input.visit(visitor)?,
            PlanNode::ScanPlan(_) => true,
        };
        if !recurse {
            return Ok(false);
        }

        if !visitor.post_visit(self)? {
            return Ok(false);
        }

        Ok(true)
    }

    pub fn list_until_bottom(&self) -> CSVQueryResult<(Vec<PlanNode>, PlanNode)> {
        let max_depth = 128;
        let mut list: Vec<PlanNode> = vec![];
        let mut plan = self.clone();

        loop {
            if list.len() > max_depth {
                return Err(CSVQueryError::Internal(format!(
                    "PlanNode depth exceed {}",
                    max_depth
                )));
            }

            match &plan {
                PlanNode::ScanPlan(v) => {
                    list.push(plan.clone());
                    break;
                }
                PlanNode::ProjectionPlan(v) => {
                    list.push(plan.clone());
                    plan = v.input.as_ref().clone();
                }
                PlanNode::SelectionPlan(v) => {
                    list.push(plan.clone());
                    plan = v.input.as_ref().clone();
                }
                PlanNode::AggregatePlan(v) => {
                    list.push(plan.clone());
                    plan = v.input.as_ref().clone();
                }
            }
        }

        list.reverse();
        Ok((list, plan.clone()))
    }
}

impl fmt::Display for PlanNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match(self) {
            PlanNode::ScanPlan(plan) => write!(f, "{}", plan)?,
            PlanNode::ProjectionPlan(plan) => write!(f, "{}", plan)?,
            PlanNode::SelectionPlan(plan) => write!(f, "{}", plan)?,
            PlanNode::AggregatePlan(plan) => write!(f, "{}", plan)?,
        }
        
        Ok(())
    }
}