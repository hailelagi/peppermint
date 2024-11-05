use plan::Column;

mod morris;
mod parser;
mod plan;

fn main() {
    let sql = "SELECT COUNT(DISTINCT col1) FROM table1;";
    let mut parser = parser::Parser::new(sql);

    match parser.parse() {
        Ok(ast) => {
            println!("Parsed AST: {:?}", ast);
        }
        Err(e) => println!("Error parsing SQL: {}", e),
    }
    let relation = plan::Relation {
        columns: vec![
            Column{name: "id".to_string(), distinct: false}, 
            Column{name: "name".to_string(), distinct: false}, 
            Column{name: "age".to_string(), distinct: false},
            ],
        rows: vec![
            vec!["1".to_string(), "Alice".to_string(), "30".to_string()],
            vec!["2".to_string(), "Bob".to_string(), "25".to_string()],
            vec!["3".to_string(), "Alice".to_string(), "22".to_string()],
        ],
    };

    let rel = relation.select(1, "Alice").projection(&[0, 2]);

    println!("{:?}", rel);
}
