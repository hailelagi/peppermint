//! Parser

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<String>,
    position: usize,
}

#[derive(Debug, PartialEq)]
pub struct SqlStatement {
    pub table_name: String,
    pub column_name: String,
    pub distinct: bool,
}

impl Parser {
    pub fn new(sql: &str) -> Self {
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

    pub fn parse(&mut self) -> Result<SqlStatement, String> {
        self.expect("select")?;
        self.expect("count")?;
        self.expect("(")?;

        let distinct = match self.peek() {
            Some(token) if token.to_lowercase() == "distinct" => {
                self.consume();
                true
            }
            _ => false,
        };

        let column_name = self
            .consume()
            .ok_or_else(|| "Expected column name".to_string())?;

        self.expect(")")?;

        // FROM
        self.expect("from")?;

        let table_name = self
            .consume()
            .ok_or_else(|| "Expected table name".to_string())?;

        // semicolon
        if let Some(token) = self.peek() {
            if token == ";" {
                self.consume();
            }
        }

        Ok(SqlStatement {
            table_name,
            column_name,
            distinct,
        })
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_count() {
        let sql = "SELECT COUNT(col2) FROM table1;";
        let mut parser = Parser::new(sql);
        let expected = SqlStatement {
            table_name: "table1".to_string(),
            column_name: "col2".to_string(),
            distinct: false,
        };

        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn test_parse_count_distinct() {
        let sql = "SELECT COUNT(DISTINCT col1) FROM table1;";
        let mut parser = Parser::new(sql);

        let expected = SqlStatement {
            table_name: "table1".to_string(),
            column_name: "col1".to_string(),
            distinct: true,
        };

        assert_eq!(parser.parse().unwrap(), expected);
    }

    #[test]
    fn test_parse_count_distinct_lowercase() {
        let sql = "select count(distinct col3) from table1;";
        let mut parser = Parser::new(sql);

        let expected = SqlStatement {
            table_name: "table1".to_string(),
            column_name: "col3".to_string(),
            distinct: true,
        };

        assert_eq!(parser.parse().unwrap(), expected);
    }
}
