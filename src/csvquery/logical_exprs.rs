use std::fmt;
use crate::csvquery::error::{CSVQueryResult, CSVQueryError};
use crate::csvquery::data_schema::{DataField, DataType};
use crate::csvquery::logical_plans::LogicalPlanRef;

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

#[derive(Debug)]
pub enum LogicalExpr {
    ColumnExpr { name: String },
    LiteralStringExpr { str: String },
    LiteralLongExpr { n: String },
    BinaryExpr {
        name: String,
        op: BinaryExprOP,
        left: Box<LogicalExpr>,
        right: Box<LogicalExpr>
    },
    AggregateExpr {
        name: String,
        expr: Box<LogicalExpr>,
    }
}

impl LogicalExpr {
    pub fn to_field(&self, input: LogicalPlanRef) -> CSVQueryResult<DataField> {
        match self {
            LogicalExpr::ColumnExpr{ ref name } => {
                let schema = input.schema().clone();
                let field = schema
                    .find_field(&name)
                    .ok_or_else(|| CSVQueryError::FieldNotFound(name.clone()))?;
                Ok(field.clone())
            },

            LogicalExpr::LiteralStringExpr{ ref str } => {
                Ok(DataField::new(str.clone(), DataType::String))
            },

            LogicalExpr::LiteralLongExpr{ ref n } => {
                Ok(DataField::new(n.clone(), DataType::Int64))
            },

            LogicalExpr::BinaryExpr{ name, op, left, right } => {
                Self::convert_binary_expr_to_field(input, name.clone(), op.clone(), left, right)
            },

            LogicalExpr::AggregateExpr{ name, expr } => {
                Ok(DataField::new(name.clone(), expr.to_field(input)?.data_type))
            },
        }
    }

    fn convert_binary_expr_to_field(input: LogicalPlanRef, name: String, op: BinaryExprOP, left: &LogicalExpr, right: &LogicalExpr) -> CSVQueryResult<DataField> {
        use BinaryExprOP::*;

        match op {
            Eq | Neq | Gt | Gte | Lt | Lte => {
                Ok(DataField::new(op.to_string(), DataType::Boolean))
            },
            And | Or => {
                Ok(DataField::new(op.to_string(), DataType::Boolean))
            },
            Add | Sub | Mult | Div | Mod  => {
                Ok(DataField::new(op.to_string(), left.to_field(input)?.data_type))
            },
        }
    }
}