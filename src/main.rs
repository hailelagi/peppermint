mod morris;
mod hll;
mod parser;
mod plan;


fn main() {
    let mut counter = morris::ApprxCounter::new();
    
    for _ in 0..1_000_000 {
        counter.increment();
    }
    
    println!("Estimated count: {}", counter.estimate());

    let relation = plan::Relation {
        column_names: vec!["id".to_string(), "name".to_string(), "age".to_string()],
        columns: todo!(),
        rows: vec![
            vec!["1".to_string(), "Alice".to_string(), "30".to_string()],
            vec!["2".to_string(), "Bob".to_string(), "25".to_string()],
            vec!["3".to_string(), "Alice".to_string(), "22".to_string()],
        ],
    };

    let projected_relation = relation.projection(&[0, 2]);
    let filtered_relation = relation.select(1, "Alice");


    println!("{:?}", projected_relation);
    println!("{:?}", filtered_relation);

    let sql = "SELECT COUNT(DISTINCT col1) FROM table1;";
    let mut parser = parser::Parser::new(sql);
    
    match parser.parse() {
        Ok(ast) => {
            println!("Parsed AST: {:?}", ast);
            

            // let count = count_distinct(&ast.projection.column, &ast.table);
            // println!("Count result: {}", count);
        },
        Err(e) => println!("Error parsing SQL: {}", e),
    }
}
