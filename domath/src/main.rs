mod calc;
use calc::calculation_definitions;

fn main() {
    let command_line_input: std::vec::Vec<_> = std::env::args().collect();
    let test = calculation_definitions::add(1.0, 2.0);
    println!("{:?}", command_line_input);
    println!("{}",  test);
}
