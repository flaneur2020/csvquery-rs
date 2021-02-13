use crate::csvquery::data_schema::DataField;
use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::logical_plans::{PlanExpr, PlanNodeRef};

pub(crate) fn exprs_to_fields(
    input: PlanNodeRef,
    exprs: &Vec<PlanExpr>,
) -> CSVQueryResult<Vec<DataField>> {
    exprs
        .iter()
        .map(|e| e.to_field(input.clone()))
        .collect::<CSVQueryResult<Vec<DataField>>>()
}
