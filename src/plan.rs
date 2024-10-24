//! Logical Planner
//! 

#[derive(Debug, Clone)]
pub struct Relation {
    pub col_names: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Relation {
        pub fn select(&self, idx: usize, expr: &str) -> Relation {
        let result: Vec<Vec<String>> = self.rows
            .iter()
            .filter(|row| row[idx] == expr) 
            .cloned()
            .collect();

        Relation {
            col_names: self.col_names.clone(), // Clone the column names
            rows: result,
        }
    }

    pub fn projection(&self, columns: &[usize]) -> Relation {
        let result: Vec<Vec<String>> = self.rows
            .iter()
            .map(|row| {
                columns.iter().map(|&col_idx| row[col_idx].clone()).collect()
            })
            .collect();

        let col_names: Vec<String> = columns
            .iter()
            .map(|&col_idx| self.col_names[col_idx].clone())
            .collect();

        Relation {
            col_names,
            rows: result,
        }
    }
}
