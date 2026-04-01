// EXERCISE 5: The deliverable — a CSV parser that never panics.
// Fix the three marked bugs so errors propagate correctly all the way to main.
// Run: cargo test --bin ex05_final -p module_08_robust

#[derive(Debug, PartialEq)]
struct Row {
    name: String,
    age: u32,
    score: f64,
}

#[derive(Debug, PartialEq)]
enum CsvError {
    WrongColumnCount { line: usize, expected: usize, got: usize },
    InvalidAge { line: usize, value: String },
    InvalidScore { line: usize, value: String },
}

impl std::fmt::Display for CsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CsvError::WrongColumnCount { line, expected, got } =>
                write!(f, "line {}: expected {} columns, got {}", line, expected, got),
            CsvError::InvalidAge { line, value } =>
                write!(f, "line {}: invalid age '{}'", line, value),
            CsvError::InvalidScore { line, value } =>
                write!(f, "line {}: invalid score '{}'", line, value),
        }
    }
}

fn parse_row(line_num: usize, line: &str) -> Result<Row, CsvError> {
    let cols: Vec<&str> = line.split(',').map(str::trim).collect();

    if cols.len() != 3 {
        return Err(CsvError::WrongColumnCount {
            line: line_num,
            expected: 3,
            got: cols.len(),
        });
    }

    let name = cols[0].to_string();

    let age: u32 = cols[1].parse().map_err(|_| CsvError::InvalidAge {
        line: line_num,
        value: cols[1].to_string(),
    })?;

    let score: f64 = cols[2].parse().map_err(|_| CsvError::InvalidScore {
        line: line_num,
        value: cols[2].to_string(),
    })?;

    Ok(Row { name, age, score })
}

fn parse_csv(input: &str) -> Result<Vec<Row>, CsvError> {
    let mut rows = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if line.trim().is_empty() { continue; }
        let row = parse_row(i + 1, line);
        rows.push(row);
    }
    Ok(rows)
}

fn summary(rows: &[Row]) -> String {
    if rows.is_empty() {
        return "No data".to_string();
    }
    let avg_score = rows.iter().map(|r| r.score).sum::<f64>() / rows.len() as f64;
    let oldest = rows.iter().max_by_key(|r| r.age).unwrap();
    format!("rows={} avg_score={:.1} oldest={}", rows.len(), avg_score, oldest.name)
}

fn main() {
    let good = "Alice, 30, 95.0\nBob, 25, 82.5\nCarol, 35, 88.0";
    match parse_csv(good) {
        Ok(rows) => println!("{}", summary(&rows)),
        Err(e)   => println!("Error: {}", e),
    }

    let bad = "Alice, 30, 95.0\nBob, ???, 82.5";
    match parse_csv(bad) {
        Ok(rows) => println!("{}", summary(&rows)),
        Err(e)   => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_good() {
        let input = "Alice, 30, 95.0\nBob, 25, 82.5";
        let rows = parse_csv(input).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].name, "Alice");
        assert_eq!(rows[1].age, 25);
    }

    #[test]
    fn test_parse_bad_age() {
        let input = "Alice, 30, 95.0\nBob, oops, 82.5";
        let err = parse_csv(input).unwrap_err();
        assert_eq!(err, CsvError::InvalidAge { line: 2, value: "oops".into() });
    }

    #[test]
    fn test_wrong_columns() {
        let input = "Alice, 30";
        let err = parse_csv(input).unwrap_err();
        assert_eq!(err, CsvError::WrongColumnCount { line: 1, expected: 3, got: 2 });
    }

    #[test]
    fn test_summary() {
        let rows = vec![
            Row { name: "Alice".into(), age: 30, score: 90.0 },
            Row { name: "Bob".into(), age: 25, score: 80.0 },
        ];
        assert_eq!(summary(&rows), "rows=2 avg_score=85.0 oldest=Alice");
    }
}
