use crate::calculation;
use crate::logic;

/*
Testing the math operations
*/

//testing the addition function
pub fn test_add() {
    assert_eq!(calculation::add(0.0, 0.0), 0.0);
    assert_eq!(calculation::add(0.0, 1.0), 1.0);
    assert_eq!(calculation::add(1.0, 0.0), 1.0);
    assert_eq!(calculation::add(0.0, -1.0), -1.0);
    assert_eq!(calculation::add(-1.0, 0.0), -1.0);
    assert_eq!(calculation::add(0.0, -0.5), -0.5);
    assert_eq!(calculation::add(-0.5, 0.0), -0.5);
    assert_eq!(calculation::add(-0.5, -0.5), -1.0);
    assert_eq!(calculation::add(27.5, -27.5), 0.0);
    assert_eq!(calculation::add(27.5, -29.5), -2.0);
    assert_eq!(calculation::add(27.5, -22.5), 5.0);
    assert_eq!(calculation::add(10.0, 15.0), 25.0);
    assert_eq!(calculation::add(0.0, -0.0005), -0.0005);
    assert_eq!(calculation::add(-0.0005, 0.0), -0.0005);
    assert_eq!(calculation::add(-0.0005, -0.0005), -0.001);
    println!("Passed test_add!");
}

//testing the subtraction function
pub fn test_subtract() {
    assert_eq!(calculation::subtract(0.0, 0.0), 0.0);
    assert_eq!(calculation::subtract(1.0, 0.0), 1.0);
    assert_eq!(calculation::subtract(0.0, 1.0), -1.0);
    assert_eq!(calculation::subtract(1.0, 1.0), 0.0);
    assert_eq!(calculation::subtract(-1.0, 0.0), -1.0);
    assert_eq!(calculation::subtract(0.0, -1.0), 1.0);
    assert_eq!(calculation::subtract(-1.0, -1.0), 0.0); 
    assert_eq!(calculation::subtract(-0.1, 0.0), -0.1);
    assert_eq!(calculation::subtract(0.0, -0.1), 0.1);
    assert_eq!(calculation::subtract(-0.1, -0.1), 0.0);
    assert_eq!(calculation::subtract(27.0, 13.5), 13.5);
    assert_eq!(calculation::subtract(27.0, -13.5), 40.5);
    assert_eq!(calculation::subtract(-27.0, 13.5), -40.5);
    assert_eq!(calculation::subtract(-27.0, -13.5), -13.5);
    println!("Passed test_subtract!");
}

//testing the divide function
pub fn test_divide() {
    assert_eq!(calculation::divide(0.0, 1.0), 0.0);
    assert_eq!(calculation::divide(0.0, -1.0), 0.0);
    assert_eq!(calculation::divide(0.0, 0.5), 0.0);
    assert_eq!(calculation::divide(0.0, -0.5), 0.0);
    assert_eq!(calculation::divide(0.0, 26.7), 0.0);
    assert_eq!(calculation::divide(0.0, -26.7), 0.0);
    assert_eq!(calculation::divide(1.0, 1.0), 1.0);
    assert_eq!(calculation::divide(1.0, -1.0), -1.0);
    assert_eq!(calculation::divide(-1.0, -1.0), 1.0);
    assert_eq!(calculation::divide(-1.0, -1.0), 1.0);
    assert_eq!(calculation::divide(10.0, 2.0), 5.0);
    assert_eq!(calculation::divide(10.0, -2.0), -5.0);
    assert_eq!(calculation::divide(-10.0, 2.0), -5.0);
    assert_eq!(calculation::divide(-10.0, -2.0), 5.0);
    assert_eq!(calculation::divide(10.5, 2.0), 5.25);
    assert_eq!(calculation::divide(-10.5, 2.0), -5.25);
    assert_eq!(calculation::divide(10.5, -2.0), -5.25);
    assert_eq!(calculation::divide(-10.5, -2.0), 5.25);
    println!("Passed test_divide!");
}

//testing the multiply function
pub fn test_multiply() {
    assert_eq!(calculation::multiply(1.0, 1.0), 1.0);
    assert_eq!(calculation::multiply(-1.0, 1.0), -1.0);
    assert_eq!(calculation::multiply(1.0, -1.0), -1.0);
    assert_eq!(calculation::multiply(-1.0, -1.0), 1.0);
    assert_eq!(calculation::multiply(0.5, 1.0), 0.5);
    assert_eq!(calculation::multiply(-0.5, 1.0), -0.5);
    assert_eq!(calculation::multiply(0.5, -1.0), -0.5);
    assert_eq!(calculation::multiply(-0.5, -1.0), 0.5);
    assert_eq!(calculation::multiply(0.0, 10.0), 0.0);
    assert_eq!(calculation::multiply(0.0, -10.0), 0.0);
    assert_eq!(calculation::multiply(0.0, 0.0), 0.0);
    assert_eq!(calculation::multiply(10.0, 0.0), 0.0);
    assert_eq!(calculation::multiply(-10.0, 0.0), 0.0);
    assert_eq!(calculation::multiply(2.5, 2.5), 6.25);
    assert_eq!(calculation::multiply(-2.5, 2.5), -6.25);
    assert_eq!(calculation::multiply(2.5, -2.5), -6.25);
    assert_eq!(calculation::multiply(-2.5, -2.5), 6.25);
    println!("Passed test_multiply!");
}

//testing positive exponents
pub fn test_positive_exponent() {
    assert_eq!(calculation::positive_exponent(0.0, 0), 1.0);
    assert_eq!(calculation::positive_exponent(-0.5, 0), 1.0);
    assert_eq!(calculation::positive_exponent(0.5, 0), 1.0);
    assert_eq!(calculation::positive_exponent(2.0, 0), 1.0);
    assert_eq!(calculation::positive_exponent(-2.0, 0), 1.0);
    assert_eq!(calculation::positive_exponent(0.0, 1), 0.0);
    assert_eq!(calculation::positive_exponent(3.0, 1), 3.0);
    assert_eq!(calculation::positive_exponent(-3.0, 1), -3.0);
    assert_eq!(calculation::positive_exponent(-0.5, 1), -0.5);
    assert_eq!(calculation::positive_exponent(0.5, 1), 0.5);
    assert_eq!(calculation::positive_exponent(3.0, 3), 27.0);
    assert_eq!(calculation::positive_exponent(-3.0, 3), -27.0);
    assert_eq!(calculation::positive_exponent(0.5, 3), 0.125);
    assert_eq!(calculation::positive_exponent(-0.5, 3), -0.125);
    assert_eq!(calculation::positive_exponent(3.0, 4), 81.0);
    assert_eq!(calculation::positive_exponent(-3.0, 4), 81.0);
    assert_eq!(calculation::positive_exponent(0.5, 4), 0.0625);
    assert_eq!(calculation::positive_exponent(-0.5, 4), 0.0625);
    println!("Passed test_positive_exponent!");
}

/*
Testing the logic functions
*/

//Testing function to determine whether a float can become an int without loss
pub fn test_is_integer() {
    assert_eq!(logic::is_integer(3.0), true);
    assert_eq!(logic::is_integer(0.0), true);
    assert_eq!(logic::is_integer(2.0), true);
    assert_eq!(logic::is_integer(-2.0), true);
    assert_eq!(logic::is_integer(-3.0), true);
    assert_eq!(logic::is_integer(3.3), false);
    assert_eq!(logic::is_integer(2.7), false);
    assert_eq!(logic::is_integer(0.1), false);
    assert_eq!(logic::is_integer(-3.3), false);
    assert_eq!(logic::is_integer(-2.7), false);
    assert_eq!(logic::is_integer(-0.1), false);
    println!("Passed test_is_integer!");
}

//testing converting from f64 -> usize
pub fn test_convert_to_int() {
    assert_eq!(logic::convert_to_int(3.0), 3);
    assert_eq!(logic::convert_to_int(3.3), 3);//should never come up by jic
    assert_eq!(logic::convert_to_int(0.0), 0);
    assert_eq!(logic::convert_to_int(-2.0), -2);
    assert_eq!(logic::convert_to_int(-2.4), -2);//should never come up but jic
    println!("Passed test_convert_to_int");
}

//testing if we can determine a valid operator
pub fn test_is_valid_operator() {
    assert_eq!(logic::is_valid_operator('+'), true);
    assert_eq!(logic::is_valid_operator('-'), true);
    assert_eq!(logic::is_valid_operator('/'), true);
    assert_eq!(logic::is_valid_operator('*'), true);
    assert_eq!(logic::is_valid_operator('('), true);
    assert_eq!(logic::is_valid_operator(')'), true);
    assert_eq!(logic::is_valid_operator('^'), true);
    assert_eq!(logic::is_valid_operator('%'), true);
    let check: String = String::from("abcdefghijklmnopqrstuvwxyz!@#$&");
    for c in check.chars() {
        assert_eq!(logic::is_valid_operator(c), false);
    }
    println!("Passed test_is_valid_operator");
}

//testing turning a vector of strings into a string
pub fn test_combine_strings() {
    let test: std::vec::Vec<String> = Vec::from(["These".to_string(), "Should".to_string(), "Be".to_string(), "Together".to_string()]);
    assert_eq!(logic::combine_strings(test), "ShouldBeTogether");
    let test: std::vec::Vec<String> = Vec::from(["1+2".to_string(), "-".to_string(), "(4-2)".to_string()]);
    assert_eq!(logic::combine_strings(test), "-(4-2)");
    let test: std::vec::Vec<String> = Vec::from(["1+2".to_string()]);
    let s: String = String::new();
    assert_eq!(logic::combine_strings(test), s);
    println!("Passed test_combine_strings!");
}

pub fn test_tokenize() {
    let test: String = String::from("(41--18.82)-13.34");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["(", "41", "--", "18.82", ")", "-", "13.34"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("1+1");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["1", "+", "1"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("1--1");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["1", "--", "1"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("43.2+(5*23.2)^2-2");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["43.2", "+", "(", "5", "*", "23.2", ")", "^", "2", "-", "2"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("-32.76+-(54+43.76)^(4.44+-6.3)");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["-", "32.76", "+-", "(", "54", "+", "43.76", ")", "^", "(", "4.44", "+-", "6.3", ")"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    println!("Passed test_tokenize!")
}