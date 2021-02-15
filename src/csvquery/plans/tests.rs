use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::plans::{BinaryExprOP, PlanBuilder, PlanExpr};
use std::sync::Arc;

#[test]
fn test_display_indent() -> CSVQueryResult<()> {
    use PlanExpr::*;

    let plan = PlanBuilder::csv("./sample.csv")?
        .filter(BinaryExpr {
            op: BinaryExprOP::Eq,
            left: Arc::new(ColumnExpr("city".to_string())),
            right: Arc::new(LiteralStringExpr("beijing".to_string())),
        })?
        .project(vec![
            ColumnExpr("name".to_string()),
            ColumnExpr("age".to_string()),
        ])?
        .build();

    Ok(())
}
