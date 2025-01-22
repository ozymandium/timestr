use tabled::{settings::Style, Table, Tabled};

#[derive(Tabled)]
struct Results {
    input: String,
    temporis: String,
    natural_date_parser: String,
    chrono_english: String,
    parse_datetime: String,
    tu: String,
}

fn main() {
    let inputs = vec![
        "tomorrow",
        "today",
        "yesterday",
        "1PM",
        "1300",
        "13:00",
        "1:15",
        "2024",
        "March",
        "last March",
        "next March",
        "March 2024",
        "2024 March",
        "March 10",
        "a year ago",
        "1 year ago",
        "1 year",
        "this week",
        "last week",
        "next week",
        "1PM Pacific Time",
        "1PM Eastern Time",
        "1PM EDT",
        "1PM EST",
    ];
    let mut results = vec![];
    let fail_str = "X".to_string();
    for input in inputs {
        let mut result = Results {
            input: input.to_string(),
            temporis: "".to_string(),
            natural_date_parser: "".to_string(),
            chrono_english: "".to_string(),
            parse_datetime: "".to_string(),
            tu: "".to_string(),
        };
        // temporis
        match temporis::parse_date(&input) {
            Ok(date) => {
                result.temporis = date.to_string();
            }
            Err(_) => {
                result.temporis = fail_str.clone();
            }
        }
        // natural_date_parser
        match natural_date_parser::date_parser::from_string(&input) {
            Ok(date) => {
                result.natural_date_parser = date.to_string();
            }
            Err(_) => {
                result.natural_date_parser = fail_str.clone();
            }
        }
        // chrono-english
        match chrono_english::parse_date_string(
            &input,
            chrono::Local::now(),
            chrono_english::Dialect::Us,
        ) {
            Ok(date) => {
                result.chrono_english = date.to_string();
            }
            Err(_) => {
                result.chrono_english = fail_str.clone();
            }
        }
        // parse_datetime
        match parse_datetime::parse_datetime(&input) {
            Ok(date) => {
                result.parse_datetime = date.to_string();
            }
            Err(_) => {
                result.parse_datetime = fail_str.clone();
            }
        }
        // tu
        match tu::parse_date_args(
            &input
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            chrono::Local::now().into(),
        ) {
            Ok(date) => {
                result.tu = date.to_string();
            }
            Err(_) => {
                result.tu = fail_str.clone();
            }
        }
        results.push(result);
    }
    let mut table = Table::new(&results);
    table.with(Style::sharp());
    println!("{}", table);
}
