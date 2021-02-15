use crate::csvquery::error::CSVQueryResult;
use crate::csvquery::plans::{BinaryExprOP, PlanBuilder, PlanExpr};
use std::sync::Arc;

#[test]
fn test_display_indent() -> CSVQueryResult<()> {
    use PlanExpr::*;

    let plan = PlanBuilder::csv("./sample.csv")?
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
              \n    Scan: ./sample.csv";
    let got = format!("{}", plan.display_indent());
    assert_eq!(want, got);
    Ok(())
}
