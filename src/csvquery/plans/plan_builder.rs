use crate::csvquery::data_sources::CSVDataSource;
use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::plans::{
    AggregatePlan, PlanExpr, PlanNode, PlanNodeRef, ProjectionPlan, ScanPlan, SelectionPlan,
};
use std::sync::Arc;

pub struct PlanBuilder {
    plan: PlanNodeRef,
}

impl PlanBuilder {
    pub fn csv(path: &str) -> CSVQueryResult<Self> {
        let data_source = Arc::new(CSVDataSource::open(path.clone())?);
        let plan = ScanPlan::new(path.clone(), data_source, vec![]);
        let plan_ref = Arc::new(PlanNode::ScanPlan(plan));
        Ok(Self::new(plan_ref))
    }

    pub fn new(plan: PlanNodeRef) -> Self {
        Self { plan: plan }
    }

    pub fn project(self, exprs: Vec<PlanExpr>) -> CSVQueryResult<Self> {
        let plan = ProjectionPlan::new(self.plan.clone(), exprs);
        let plan_ref = Arc::new(PlanNode::ProjectionPlan(plan));
        Ok(self)
    }

    pub fn filter(self, expr: PlanExpr) -> CSVQueryResult<Self> {
        let plan = SelectionPlan::new(self.plan.clone(), expr);
        let plan_ref = Arc::new(PlanNode::SelectionPlan(plan));
        Ok(self)
    }

    pub fn aggregate(
        self,
        group_by: Vec<PlanExpr>,
        aggregate_exprs: Vec<PlanExpr>,
    ) -> CSVQueryResult<Self> {
        let plan = AggregatePlan::new(self.plan.clone(), group_by, aggregate_exprs);
        let plan_ref = Arc::new(PlanNode::AggregatePlan(plan));
        Ok(self)
    }

    pub fn build(self) -> PlanNodeRef {
        self.plan.clone()
    }
}
