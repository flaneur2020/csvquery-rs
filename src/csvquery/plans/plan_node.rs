use crate::csvquery::data_types::DataSchemaRef;
use crate::csvquery::error::{CSVQueryError, CSVQueryResult};
use crate::csvquery::plans::{
    AggregatePlan, IndentVisitor, PlanVisitor, ProjectionPlan, ScanPlan, SelectionPlan,
};
use std::fmt;
use std::sync::Arc;

pub type PlanNodeRef = Arc<PlanNode>;

#[derive(Clone)]
pub enum PlanNode {
    Scan(ScanPlan),
    Projection(ProjectionPlan),
    Selection(SelectionPlan),
    Aggregate(AggregatePlan),
}

impl PlanNode {
    pub fn schema(&self) -> CSVQueryResult<DataSchemaRef> {
        match self {
            PlanNode::Scan(plan) => plan.schema(),
            PlanNode::Projection(plan) => plan.schema(),
            PlanNode::Selection(plan) => plan.schema(),
            PlanNode::Aggregate(plan) => plan.schema(),
        }
    }

    pub fn visit<V: PlanVisitor>(&self, visitor: &mut V) -> std::result::Result<(), V::Error> {
        visitor.pre_visit(self)?;

        match self {
            PlanNode::Projection(plan) => plan.input.visit(visitor)?,
            PlanNode::Selection(plan) => plan.input.visit(visitor)?,
            PlanNode::Aggregate(plan) => plan.input.visit(visitor)?,
            PlanNode::Scan(_) => (),
        };

        visitor.post_visit(self)?;
        Ok(())
    }

    pub fn display_indent(&self) -> impl fmt::Display + '_ {
        struct Wrapper<'a>(&'a PlanNode);
        impl<'a> fmt::Display for Wrapper<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut visitor = IndentVisitor::new(f);
                self.0.visit(&mut visitor)?;
                Ok(())
            }
        }
        Wrapper(self)
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
                PlanNode::Scan(v) => {
                    list.push(plan.clone());
                    break;
                }
                PlanNode::Projection(v) => {
                    list.push(plan.clone());
                    plan = v.input.as_ref().clone();
                }
                PlanNode::Selection(v) => {
                    list.push(plan.clone());
                    plan = v.input.as_ref().clone();
                }
                PlanNode::Aggregate(v) => {
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
        match (self) {
            PlanNode::Scan(plan) => write!(f, "{}", plan)?,
            PlanNode::Projection(plan) => write!(f, "{}", plan)?,
            PlanNode::Selection(plan) => write!(f, "{}", plan)?,
            PlanNode::Aggregate(plan) => write!(f, "{}", plan)?,
        }

        Ok(())
    }
}
