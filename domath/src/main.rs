mod calc;

fn test_everything() {
    println!("<----------Testing the calculation functions:---------->");
    calc::tests::test_add();
    calc::tests::test_subtract();
    calc::tests::test_divide();
    calc::tests::test_multiply();
    calc::tests::test_positive_exponent();
    println!("<----------Testing the logic operation functions:---------->");
    calc::tests::test_is_integer();
    calc::tests::test_convert_to_int();
    calc::tests::test_is_valid_operator();
    calc::tests::test_combine_strings();
    calc::tests::test_is_token_digit();
    calc::tests::test_is_token_operator();
    calc::tests::test_operator_value();
    println!("<----------Testing the Tokenizer, Parser and Evaluator:---------->");
    calc::tests::test_tokenize();
    calc::tests::test_parser();
}

fn main() {
    /*
    let command_line_input: std::vec::Vec<String> = std::env::args().collect();
    let test: String = calc::logic::combine_strings(command_line_input);
    println!("{}", test);
    */
    test_everything();
}
