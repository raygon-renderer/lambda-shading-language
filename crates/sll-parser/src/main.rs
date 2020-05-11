extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "../grammar.pest"]
pub struct GrammarParser;

fn main() {
    let test =
r#"
struct Test {
    a: i32,
    b: [f32; 4],
    c: Vector3
}

fn other() {

}

fn add(x: i32, y: i32) -> i32 {
    let z: i32 = x + y;

    other();

    z + 1
}
"#;

    let parsed = GrammarParser::parse(Rule::main, test);

    println!("{:#?}", parsed);
}