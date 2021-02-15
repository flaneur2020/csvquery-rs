use crate::csvquery::data_types::{DataField, DataType};
use crate::csvquery::error::{CSVQueryError, CSVQueryResult};
use crate::csvquery::plans::PlanNodeRef;
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
    ColumnExpr {
        name: String,
    },
    LiteralStringExpr {
        str: String,
    },
    LiteralLongExpr {
        n: String,
    },
    BinaryExpr {
        name: String,
        op: BinaryExprOP,
        left: Arc<PlanExpr>,
        right: Arc<PlanExpr>,
    },
    AggregateExpr {
        name: String,
        expr: Arc<PlanExpr>,
    },
}

impl PlanExpr {
    pub fn to_field(&self, input: PlanNodeRef) -> CSVQueryResult<DataField> {
        match self {
            PlanExpr::ColumnExpr { ref name } => {
                let schema = input.schema().clone();
                let field = schema
                    .field_with_name(&name)
                    .or_else(|_| Err(CSVQueryError::FieldNotFound(name.clone())))?;
                Ok(field.clone())
            }

            PlanExpr::LiteralStringExpr { str } => {
                Ok(DataField::new(&(str.clone()), DataType::Utf8, false))
            }

            PlanExpr::LiteralLongExpr { n } => {
                Ok(DataField::new(&(n.clone()), DataType::Int64, false))
            }

            PlanExpr::BinaryExpr {
                name,
                op,
                left,
                right,
            } => convert_binary_expr_to_field(input, name.clone(), op.clone(), left, right),

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
    name: String,
    op: BinaryExprOP,
    left: &PlanExpr,
    right: &PlanExpr,
) -> CSVQueryResult<DataField> {
    use BinaryExprOP::*;

    match op {
        Eq | Neq | Gt | Gte | Lt | Lte => {
            Ok(DataField::new(&op.to_string(), DataType::Boolean, false))
        }
        And | Or => Ok(DataField::new(&op.to_string(), DataType::Boolean, false)),
        Add | Sub | Mult | Div | Mod => {
            let data_type = left.to_field(input)?.data_type().clone();
            Ok(DataField::new(&op.to_string(), data_type, false))
        }
    }
}
