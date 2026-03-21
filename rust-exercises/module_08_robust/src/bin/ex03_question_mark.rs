// EXERCISE 3: Replace manual match chains with the ? operator.
// `expr?` on a Result: if Ok, unwraps the value. If Err, returns early with the error.
// It's shorthand for: `match expr { Ok(v) => v, Err(e) => return Err(e) }`
// Rewrite `parse_record` to use `?` instead of the verbose match blocks.
// Run: cargo test --bin ex03_question_mark -p module_08_robust

#[derive(Debug, PartialEq)]
struct Record {
    name: String,
    age: u32,
    score: f64,
}

#[derive(Debug, PartialEq)]
enum ParseError {
    MissingField(String),
    InvalidNumber(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::MissingField(s) => write!(f, "missing field: {}", s),
            ParseError::InvalidNumber(s) => write!(f, "invalid number: '{}'", s),
        }
    }
}

fn get_field<'a>(fields: &[&'a str], index: usize, name: &str) -> Result<&'a str, ParseError> {
    fields.get(index).copied().ok_or_else(|| ParseError::MissingField(name.to_string()))
}

fn parse_record(line: &str) -> Result<Record, ParseError> {
    let fields: Vec<&str> = line.split(',').map(str::trim).collect();

    // BUG: rewrite each of these using `?` instead of match
    let name = match get_field(&fields, 0, "name") {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let age_str = match get_field(&fields, 1, "age") {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let score_str = match get_field(&fields, 2, "score") {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let age: u32 = match age_str.parse() {
        Ok(v) => v,
        Err(_) => return Err(ParseError::InvalidNumber(age_str.to_string())),
    };
    let score: f64 = match score_str.parse() {
        Ok(v) => v,
        Err(_) => 0.0,  // BUG: silently swallows score parse errors instead of returning Err
    };

    Ok(Record { name: name.to_string(), age, score })
}

fn main() {
    println!("{:?}", parse_record("Alice, 30, 95.5"));
    println!("{:?}", parse_record("Bob, ???, 80"));
    println!("{:?}", parse_record("Carol"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(
            parse_record("Alice, 30, 95.5"),
            Ok(Record { name: "Alice".into(), age: 30, score: 95.5 })
        );
    }

    #[test]
    fn test_missing_field() {
        assert_eq!(
            parse_record("Alice"),
            Err(ParseError::MissingField("age".into()))
        );
    }

    #[test]
    fn test_invalid_number() {
        assert_eq!(
            parse_record("Bob, ???, 80"),
            Err(ParseError::InvalidNumber("???".into()))
        );
    }
}
