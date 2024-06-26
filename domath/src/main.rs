mod calc;

fn test_everything() {
    println!("<----------Testing the calculation functions:---------->");
    calc::tests::test_add();
    calc::tests::test_subtract();
    calc::tests::test_divide();
    calc::tests::test_multiply();
    println!("<----------Testing the logic operation functions:---------->");
    calc::tests::test_is_integer();
    calc::tests::test_convert_to_int();
    calc::tests::test_is_valid_operator();
    calc::tests::test_combine_strings();
    calc::tests::test_is_token_digit();
    calc::tests::test_is_token_operator();
    calc::tests::test_token_associativity();
    println!("<----------Testing the Tokenizer, Parser and Evaluator:---------->");
    calc::tests::test_tokenize();
    calc::tests::test_parser();
    calc::tests::test_evaluator();
}

fn main() {
    let command_line_input: std::vec::Vec<String> = std::env::args().collect();
    let expression: String = calc::logic::combine_strings(command_line_input);
    let tokens: std::vec::Vec<String> = calc::logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = calc::logic::parser(tokens);
    let result: f64 = calc::logic::evaluator(parsed_tokens);
    
    if calc::logic::is_integer(&result) {
        println!("{}", calc::logic::convert_to_int(result));
    }
    else {
        println!("{}", result);
    }
    //temp add method for handling command
    //test_everything();
}
