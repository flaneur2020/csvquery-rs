use crate::csvquery::data_types::DataField;
use crate::csvquery::error::CQResult;
use crate::csvquery::plans::{PlanExpr, PlanNodeRef};

pub(crate) fn exprs_to_fields(
    input: PlanNodeRef,
    exprs: &Vec<PlanExpr>,
) -> CQResult<Vec<DataField>> {
    exprs
        .iter()
        .map(|e| e.to_field(input.clone()))
        .collect::<CQResult<Vec<DataField>>>()
}
