use crate::csvquery::plans::PlanNode;
use std::fmt;

/// Trait that implements the Visitor pattern for a depth
/// first walk of `PlanNode` nodes. `pre_visit` is called
/// before any children are visited, and then `post_visit`
/// is called after all children have been visited.
pub trait PlanVisitor {
    type Error;

    fn pre_visit(&mut self, plan: &PlanNode) -> std::result::Result<bool, Self::Error>;

    fn post_visit(&mut self, plan: &PlanNode) -> std::result::Result<bool, Self::Error> {
        Ok(true)
    }
}

/// Formats plans with single line per node. For example:
///
/// Projection: #id
///   Filter: #state Eq 'ACTIVE'
///     Scan: employee.csv projection=[]
pub struct IndentVisitor<'a, 'b> {
    f: &'a mut fmt::Formatter<'b>,
    indent: u32,
}

impl<'a, 'b> IndentVisitor<'a, 'b> {
    pub fn new(f: &'a mut fmt::Formatter<'b>) -> Self {
        Self { f: f, indent: 0 }
    }

    fn write_indent(&mut self) -> fmt::Result {
        for _ in 0..self.indent {
            write!(self.f, "  ")?;
        }
        Ok(())
    }
}

impl<'a, 'b> PlanVisitor for IndentVisitor<'a, 'b> {
    type Error = fmt::Error;

    fn pre_visit(&mut self, plan: &PlanNode) -> std::result::Result<bool, Self::Error> {
        if self.indent > 0 {
            writeln!(self.f)?;
        }
        self.write_indent()?;

        write!(self.f, "{}", plan)?;

        self.indent += 1;
        Ok(true)
    }

    fn post_visit(&mut self, plan: &PlanNode) -> std::result::Result<bool, Self::Error> {
        self.indent -= 1;
        Ok(true)
    }
}
