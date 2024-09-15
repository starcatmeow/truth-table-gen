use std::io::{stdin, BufRead};

mod prop;
mod tools;

fn main() {
    let mut buf = String::new();
    print!("Enter a proposition expression: ");
    stdin().lock().read_line(&mut buf).expect("Expected a proposition expression!");
    let prop = prop::parse_prop(buf.as_str()).expect("Failed to parse proposition expression!");
    let result = tools::truth_table_gen::truth_table_gen(&prop);
    tools::truth_table_gen::print_truth_table(result);
}
