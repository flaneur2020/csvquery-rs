use crate::csvquery::data_sources::CSVDataSource;
use crate::csvquery::error::CQResult;
use crate::csvquery::plans::{
    AggregatePlan, PlanExpr, PlanNode, PlanNodeRef, ProjectionPlan, ScanPlan, SelectionPlan,
};
use std::sync::Arc;

pub struct PlanBuilder {
    plan: PlanNodeRef,
}

impl PlanBuilder {
    pub fn csv(path: &str) -> CQResult<Self> {
        let data_source = Arc::new(CSVDataSource::try_new(path.clone())?);
        let plan = ScanPlan::new(data_source, vec![]);
        let plan_ref = Arc::new(PlanNode::Scan(plan));
        Ok(Self::new(plan_ref))
    }

    pub fn new(plan: PlanNodeRef) -> Self {
        Self { plan: plan }
    }

    pub fn project(self, exprs: Vec<PlanExpr>) -> CQResult<Self> {
        let plan = ProjectionPlan::new(self.plan.clone(), exprs);
        let plan_ref = Arc::new(PlanNode::Projection(plan));
        Ok(PlanBuilder::new(plan_ref))
    }

    pub fn filter(self, expr: PlanExpr) -> CQResult<Self> {
        let plan = SelectionPlan::new(self.plan.clone(), expr);
        let plan_ref = Arc::new(PlanNode::Selection(plan));
        Ok(PlanBuilder::new(plan_ref))
    }

    pub fn aggregate(
        self,
        group_by: Vec<PlanExpr>,
        aggregate_exprs: Vec<PlanExpr>,
    ) -> CQResult<Self> {
        let plan = AggregatePlan::new(self.plan.clone(), group_by, aggregate_exprs);
        let plan_ref = Arc::new(PlanNode::Aggregate(plan));
        Ok(PlanBuilder::new(plan_ref))
    }

    pub fn build(self) -> PlanNodeRef {
        self.plan.clone()
    }
}
