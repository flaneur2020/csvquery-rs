use crate::csvquery::data_types::{DataField, DataType};
use crate::csvquery::error::{CQError, CQResult};
use crate::csvquery::logical_plans::PlanNodeRef;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Copy, Clone)]
pub enum BinaryExprOP {
    // compare operators
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,

    // boolean operators
    And,
    Or,

    // math operators
    Add,
    Sub,
    Mult,
    Div,
    Mod,
}

impl std::fmt::Display for BinaryExprOP {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryExprOP::Eq => write!(f, "eq"),
            BinaryExprOP::Neq => write!(f, "neq"),
            BinaryExprOP::Gt => write!(f, "gt"),
            BinaryExprOP::Gte => write!(f, "gte"),
            BinaryExprOP::Lt => write!(f, "lt"),
            BinaryExprOP::Lte => write!(f, "lte"),
            BinaryExprOP::And => write!(f, "and"),
            BinaryExprOP::Or => write!(f, "or"),
            BinaryExprOP::Add => write!(f, "add"),
            BinaryExprOP::Sub => write!(f, "sub"),
            BinaryExprOP::Mult => write!(f, "mult"),
            BinaryExprOP::Div => write!(f, "div"),
            BinaryExprOP::Mod => write!(f, "mod"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PlanExpr {
    ColumnExpr(String),
    LiteralStringExpr(String),
    LiteralLongExpr(String),
    BinaryExpr(BinaryExprOP, Arc<PlanExpr>, Arc<PlanExpr>),
    AggregateExpr { name: String, expr: Arc<PlanExpr> },
}

impl PlanExpr {
    pub fn to_field(&self, input: PlanNodeRef) -> CQResult<DataField> {
        match self {
            PlanExpr::ColumnExpr(ref name) => {
                let schema = input.schema()?.clone();
                let field = schema
                    .field_with_name(&name)
                    .or_else(|_| Err(CQError::FieldNotFound(name.clone())))?;
                Ok(field.clone())
            }

            PlanExpr::LiteralStringExpr(str) => {
                Ok(DataField::new(&(str.clone()), DataType::Utf8, false))
            }

            PlanExpr::LiteralLongExpr(n) => {
                Ok(DataField::new(&(n.clone()), DataType::Int64, false))
            }

            PlanExpr::BinaryExpr(op, left, right) => {
                convert_binary_expr_to_field(input, op.clone(), left, right)
            }

            PlanExpr::AggregateExpr { name, expr } => Ok(DataField::new(
                &name.clone(),
                expr.to_field(input)?.data_type().clone(),
                false,
            )),
        }
    }
}

fn convert_binary_expr_to_field(
    input: PlanNodeRef,
    op: BinaryExprOP,
    left: &PlanExpr,
    right: &PlanExpr,
) -> CQResult<DataField> {
    use BinaryExprOP::*;

    let field_name = format!("({} {} {})", left, op, right).to_string();

    match op {
        Eq | Neq | Gt | Gte | Lt | Lte | And | Or => {
            Ok(DataField::new(&field_name, DataType::Boolean, false))
        }
        Add | Sub | Mult | Div | Mod => {
            let data_type = left.to_field(input)?.data_type().clone();
            Ok(DataField::new(&field_name, data_type, false))
        }
    }
}

impl fmt::Display for PlanExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PlanExpr::ColumnExpr(v) => write!(f, "Column({})", v)?,
            PlanExpr::LiteralStringExpr(v) => write!(f, "LiteralString({})", v)?,
            PlanExpr::LiteralLongExpr(v) => write!(f, "LiteralLong({})", v)?,
            PlanExpr::BinaryExpr(op, left, right) => write!(f, "({} {} {})", left, op, right)?,
            PlanExpr::AggregateExpr { name, expr } => write!(f, "Aggregate({}, {})", name, expr)?,
        }
        Ok(())
    }
}
