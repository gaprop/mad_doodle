extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::fs;

use pest::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn main() {
    let file = fs::read_to_string("mad.csv").expect("cannot read file");

    let file = CSVParser::parse(Rule::file, &file)
        .expect("unsuccessfull parse")
        .next().unwrap();

    println!("{:?}", file);
}
