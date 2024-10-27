use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Aggregation {
    Count,
}

#[derive(Debug, PartialEq)]
struct Column {
    name: String,
    distinct: bool,
}

#[derive(Debug, PartialEq)]
struct AggregateExpression {
    function: Aggregation,
    column: Column,
}

#[derive(Debug, PartialEq)]
struct SelectStatement {
    projection: AggregateExpression,
    table: String,
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<String>,
    position: usize,
}

impl Parser {
    fn new(sql: &str) -> Self {
        let tokens: Vec<String> = sql
            .replace("(", " ( ")
            .replace(")", " ) ")
            .replace(";", " ;")
            .split_whitespace()
            .map(String::from)
            .collect();
        
        Parser {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&String> {
        self.tokens.get(self.position)
    }

    fn consume(&mut self) -> Option<String> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect(&mut self, expected: &str) -> Result<(), String> {
        match self.consume() {
            Some(token) if token.to_lowercase() == expected.to_lowercase() => Ok(()),
            Some(token) => Err(format!("Expected {}, found {}", expected, token)),
            None => Err(format!("Expected {}, found end of input", expected)),
        }
    }

    fn parse(&mut self) -> Result<SelectStatement, String> {
        self.expect("select")?;
        self.expect("count")?;
        self.expect("(")?;
        
        let distinct = match self.peek() {
            Some(token) if token.to_lowercase() == "distinct" => {
                self.consume();
                true
            },
            _ => false,
        };

        let column_name = self.consume()
            .ok_or_else(|| "Expected column name".to_string())?;
        
        self.expect(")")?;

        // FROM
        self.expect("from")?;
        
        let table_name = self.consume()
            .ok_or_else(|| "Expected table name".to_string())?;

        // semicolon
        if let Some(token) = self.peek() {
            if token == ";" {
                self.consume();
            }
        }

        Ok(SelectStatement {
            projection: AggregateExpression {
                function: Aggregation::Count,
                column: Column {
                    name: column_name,
                    distinct,
                },
            },
            table: table_name,
        })
    }
}

// TODO(hyperloglog): FIXME
fn count_distinct(_column: &Column, _table: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let sql = "SELECT COUNT(DISTINCT col1) FROM table1;";
        let mut parser = Parser::new(sql);
        
        let expected = SelectStatement {
            projection: AggregateExpression {
                function: Aggregation::Count,
                column: Column {
                    name: "col1".to_string(),
                    distinct: true,
                },
            },
            table: "table1".to_string(),
        };
        
        assert_eq!(parser.parse().unwrap(), expected);
    }
}