use crate::csvquery::error::CQResult;
use crate::csvquery::logical_plans::{BinaryExprOP, PlanBuilder, PlanExpr};
use crate::csvquery::streams::CsvReadOptions;
use std::sync::Arc;

#[test]
fn test_display_indent() -> CQResult<()> {
    use PlanExpr::*;

    let csv_options = CsvReadOptions::new();
    let plan = PlanBuilder::csv("./sample.csv", &csv_options)?
        .filter(BinaryExpr(
            BinaryExprOP::Eq,
            Arc::new(ColumnExpr("city".to_string())),
            Arc::new(LiteralStringExpr("beijing".to_string())),
        ))?
        .project(vec![
            ColumnExpr("name".to_string()),
            ColumnExpr("age".to_string()),
        ])?
        .build();

    let want = "Projection: Column(name), Column(age)\
              \n  Selection: (Column(city) eq LiteralString(beijing))\
              \n    Scan CSV: ./sample.csv";
    let got = format!("{}", plan.display_indent());
    assert_eq!(want, got);
    Ok(())
}
