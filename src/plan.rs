//! Logical Planner
//!

use crate::parser::SqlStatement;

#[derive(Debug, Clone)]
pub struct Relation {
    pub columns: Vec<Column>,
    pub rows: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub name: String,
    pub distinct: bool,
}

#[derive(Debug, PartialEq)]
pub struct AggregateExpression {
    pub function: Aggregation,
    pub column: Column,
}

#[derive(Debug, PartialEq)]
pub struct SelectStatement {
    pub projection: AggregateExpression,
    pub table: String,
}

#[derive(Debug, PartialEq)]
pub enum Aggregation {
    Count,
}

#[derive(Debug, PartialEq)]
pub struct LogicalPlan {
    pub statement: SelectStatement,
}

impl LogicalPlan {
    pub fn new(statement: SqlStatement) -> LogicalPlan {
        LogicalPlan {
            statement: SelectStatement {
                projection: AggregateExpression {
                    function: Aggregation::Count,
                    column: Column {
                        name: statement.column_name,
                        distinct: statement.distinct,
                    },
                },
                table: statement.table_name,
            },
        }
    }
}

impl Relation {
    pub fn select(&self, idx: usize, expr: &str) -> Relation {
        let result: Vec<Vec<String>> = self
            .rows
            .iter()
            .filter(|row| row[idx] == expr)
            .cloned()
            .collect();


        Relation {
            columns: self.columns.clone(),
            rows: result,
        }
    }

    pub fn projection(&self, columns: &[usize]) -> Relation {
        let result: Vec<Vec<String>> = self
            .rows
            .iter()
            .map(|row| {
                columns
                    .iter()
                    .map(|&col_idx| row[col_idx].clone())
                    .collect()
            })
            .collect();

        // dispatch to count

        Relation {
            columns: self.columns.clone(),
            rows: result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::Parser;

    #[test]
    fn test_plan_count() {
        let sql = "SELECT COUNT(col) FROM table;";
        let mut parser = Parser::new(sql);
        let plan = LogicalPlan::new(parser.parse().unwrap());

        let expected = LogicalPlan {
            statement: SelectStatement {
                projection: AggregateExpression {
                    function: Aggregation::Count,
                    column: Column {
                        name: "col".to_string(),
                        distinct: false,
                    },
                },
                table: "table".to_string(),
            },
        };

        assert_eq!(plan, expected);
    }

    #[test]
    fn test_plan_count_distinct() {
        let sql = "SELECT COUNT(DISTINCT col1) FROM table1";
        let mut parser = Parser::new(sql);
        let plan = LogicalPlan::new(parser.parse().unwrap());

        let expected = LogicalPlan {
            statement: SelectStatement {
                projection: AggregateExpression {
                    function: Aggregation::Count,
                    column: Column {
                        name: "col1".to_string(),
                        distinct: true,
                    },
                },
                table: "table1".to_string(),
            },
        };

        assert_eq!(plan, expected);
    }
}
