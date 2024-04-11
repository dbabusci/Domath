use crate::calc::calculation;
use crate::calc::logic;

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

/*
Testing the logic functions
*/

//Testing function to determine whether a float can become an int without loss
pub fn test_is_integer() {
    assert_eq!(logic::is_integer(&3.0), true);
    assert_eq!(logic::is_integer(&0.0), true);
    assert_eq!(logic::is_integer(&2.0), true);
    assert_eq!(logic::is_integer(&-2.0), true);
    assert_eq!(logic::is_integer(&-3.0), true);
    assert_eq!(logic::is_integer(&3.3), false);
    assert_eq!(logic::is_integer(&2.7), false);
    assert_eq!(logic::is_integer(&0.1), false);
    assert_eq!(logic::is_integer(&-3.3), false);
    assert_eq!(logic::is_integer(&-2.7), false);
    assert_eq!(logic::is_integer(&-0.1), false);
    println!("Passed test_is_integer!");
}

//testing converting from f64 -> usize
pub fn test_convert_to_int() {
    assert_eq!(logic::convert_to_int(3.0), 3);
    assert_eq!(logic::convert_to_int(3.3), 3);//should never come up by jic
    assert_eq!(logic::convert_to_int(0.0), 0);
    assert_eq!(logic::convert_to_int(-2.0), -2);
    assert_eq!(logic::convert_to_int(-2.4), -2);//should never come up but jic
    println!("Passed test_convert_to_int!");
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
    assert_eq!(logic::is_valid_operator('p'), true);
    assert_eq!(logic::is_valid_operator('d'), true);
    let check: String = String::from("abcefghijklmnoqrstuvwxyz!@#$&");
    for c in check.chars() {
        assert_eq!(logic::is_valid_operator(c), false);
    }
    println!("Passed test_is_valid_operator!");
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
    let test: String = String::from("1+(2*3)");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["1", "+", "(", "2", "*", "3", ")"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("6-4-5-5");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["6", "-", "4", "-", "5", "-", "5"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("(1+2)*3");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["(", "1", "+", "2", ")", "*", "3"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("-1+-2*-3");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["-1", "+", "-2", "*", "-3"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("(41--18.82)-13.34");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["(", "41", "-", "-18.82", ")", "-", "13.34"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("1+1");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["1", "+", "1"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("1--1");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["1", "-", "-1"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("43.2+(5*23.2)^2-2");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["43.2", "+", "(", "5", "*", "23.2", ")", "^", "2", "-", "2"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("-32.76+(54+43.76)^(4.44+-6.3)");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["-32.76", "+", "(", "54", "+", "43.76", ")", "^", "(", "4.44", "+", "-6.3", ")"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("(((((5+5)))))");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["(", "(", "(", "(", "(", "5", "+", "5", ")", ")", ")", ")", ")"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("43.2+p5*23.2d^2-2");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["43.2", "+", "p", "5", "*", "23.2", "d", "^", "2", "-", "2"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("-32.76+p54+43.76d^p4.44+-6.3d");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["-32.76", "+", "p", "54", "+", "43.76", "d", "^", "p", "4.44", "+", "-6.3", "d"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("ppppp5+5ddddd");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["p", "p", "p", "p", "p", "5", "+", "5", "d", "d", "d", "d", "d"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("p41--18.82d-13.34");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["p", "41", "-", "-18.82", "d", "-", "13.34"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    let test: String = String::from("1+p2*3d");
    let check_string: std::vec::Vec<String> = logic::tokenize(test);
    let converted: std::vec::Vec<&str> = Vec::from(["1", "+", "p", "2", "*", "3", "d"]);
    let check_tokens: std::vec::Vec<String> = logic::convert_literals(converted);
    assert_eq!(check_string, check_tokens);

    println!("Passed test_tokenize!")
}

pub fn test_is_token_digit() {
    let check: String = String::from("1");
    assert_eq!(logic::is_token_digit(&check), true);
    let check: String = String::from("-1");
    assert_eq!(logic::is_token_digit(&check), true);
    let check: String = String::from(".98");
    assert_eq!(logic::is_token_digit(&check), true);
    let check: String = String::from("-0.372");
    assert_eq!(logic::is_token_digit(&check), true);
    let check: String = String::from("-.93");
    assert_eq!(logic::is_token_digit(&check), true);
    let check: String = String::from("+");
    assert_eq!(logic::is_token_digit(&check), false);
    println!("Passed test_is_token_digit!");
}

pub fn test_is_token_operator() {
    let check: String = String::from("+");
    assert_eq!(logic::is_token_operator(&check), true);
    let check: String = String::from("-");
    assert_eq!(logic::is_token_operator(&check), true);
    let check: String = String::from("(");
    assert_eq!(logic::is_token_operator(&check), true);
    let check: String = String::from(")");
    assert_eq!(logic::is_token_operator(&check), true);
    let check: String = String::from("*");
    assert_eq!(logic::is_token_operator(&check), true);
    let check: String = String::from("/");
    assert_eq!(logic::is_token_operator(&check), true);
    let check: String = String::from("12");
    assert_eq!(logic::is_token_operator(&check), false);
    let check: String = String::from("-12.5");
    assert_eq!(logic::is_token_operator(&check), false);
    let check: String = String::from("-.04");
    assert_eq!(logic::is_token_operator(&check), false);
    let check: String = String::from("^");
    assert_eq!(logic::is_token_operator(&check), true);
    let check: String = String::from("-1");
    assert_eq!(logic::is_token_operator(&check), false);
    println!("Passed test_is_token_operator!");
}

pub fn test_token_associativity() {
    assert_eq!(true, logic::token_associativity(&"+".to_string()));
    assert_eq!(true, logic::token_associativity(&"-".to_string()));
    assert_eq!(true, logic::token_associativity(&"/".to_string()));
    assert_eq!(true, logic::token_associativity(&"*".to_string()));
    assert_eq!(false, logic::token_associativity(&"^".to_string()));
    assert_eq!(false, logic::token_associativity(&"(".to_string()));
    assert_eq!(false, logic::token_associativity(&")".to_string()));

    println!("Passed test_token_associativity");
}

pub fn test_parser() {
    let tokens: std::vec::Vec<String> = Vec::from(["1".to_string(), "+".to_string(), "2".to_string()]);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let check_tokens: std::vec::Vec<String> = Vec::from(["1".to_string(), "2".to_string(), "+".to_string()]);
    assert_eq!(parsed_tokens, check_tokens);

    let tokens: std::vec::Vec<String> = Vec::from(["-1".to_string(), "+".to_string(), "-2".to_string()]);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let check_tokens: std::vec::Vec<String> = Vec::from(["-1".to_string(), "-2".to_string(), "+".to_string()]);
    assert_eq!(parsed_tokens, check_tokens);

    let expression: String = String::from("1+2-3");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["1","2","+","3","-"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);
    
    let expression: String = String::from("1+2*3");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["1","2","3","*","+"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);

    let expression: String = String::from("-1+-2*-3");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["-1","-2","-3","*","+"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);
    
    let expression: String = String::from("1+(2*3)");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["1","2","3","*","+"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);
    
    let expression: String = String::from("(1+2)*3");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["1","2","+","3","*"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);

    let expression: String = String::from("6-4-5-5");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["6","4","-","5","-","5","-"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);

    let expression: String = String::from("5+2/(3-8)^5^2");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["5", "2", "3", "8", "-", "5", "2", "^", "^", "/", "+"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);

    let expression: String = String::from("5.22+2.12/(3.32-8.76)^5.2^2.7");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["5.22", "2.12", "3.32", "8.76", "-", "5.2", "2.7", "^", "^", "/", "+"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);

    let expression: String = String::from("((((((6+-2))))))");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["6", "-2", "+"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);

    let expression: String = String::from("5+2/p3-8d^5^2");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["5", "2", "3", "8", "-", "5", "2", "^", "^", "/", "+"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);

    let expression: String = String::from("5.22+2.12/p3.32-8.76d^5.2^2.7");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["5.22", "2.12", "3.32", "8.76", "-", "5.2", "2.7", "^", "^", "/", "+"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);

    let expression: String = String::from("pppppp6+-2dddddd");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["6", "-2", "+"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }

    let expression: String = String::from("1+p2*3d");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["1","2","3","*","+"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);
    
    let expression: String = String::from("p1+2d*3");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let literal_check_tokens: std::vec::Vec<&str> = Vec::from(["1","2","+","3","*"]);
    let mut check_tokens: std::vec::Vec<String> = Vec::new();
    for t in literal_check_tokens {
        check_tokens.push(t.to_string());
    }
    assert_eq!(parsed_tokens, check_tokens);

    println!("Passed test_parser!");
}

pub fn test_evaluator() {
    let expression: String = String::from("1+1");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let evaluated: f64 = logic::evaluator(parsed_tokens);
    assert_eq!(evaluated, 2.0);

    let expression: String = String::from("1-1");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let evaluated: f64 = logic::evaluator(parsed_tokens);
    assert_eq!(evaluated, 0.0);

    let expression: String = String::from("1*1");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let evaluated: f64 = logic::evaluator(parsed_tokens);
    assert_eq!(evaluated, 1.0);

    let expression: String = String::from("1/1");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let evaluated: f64 = logic::evaluator(parsed_tokens);
    assert_eq!(evaluated, 1.0);

    let expression: String = String::from("6+4+5+5");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let evaluated: f64 = logic::evaluator(parsed_tokens);
    assert_eq!(evaluated, 20.0);

    let expression: String = String::from("6-4-5-5");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let evaluated: f64 = logic::evaluator(parsed_tokens);
    assert_eq!(evaluated, -8.0);

    let expression: String = String::from("6*4*5*5");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let evaluated: f64 = logic::evaluator(parsed_tokens);
    assert_eq!(evaluated, 600.0);

    let expression: String = String::from("6/4/5/5");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let evaluated: f64 = logic::evaluator(parsed_tokens);
    assert_eq!(evaluated, 0.06);

    let expression: String = String::from("(5+3)*-8");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let evaluated: f64 = logic::evaluator(parsed_tokens);
    assert_eq!(evaluated, -64.0);

    let expression: String = String::from("p5+3d*-8");
    let tokens: std::vec::Vec<String> = logic::tokenize(expression);
    let parsed_tokens: std::vec::Vec<String> = logic::parser(tokens);
    let evaluated: f64 = logic::evaluator(parsed_tokens);
    assert_eq!(evaluated, -64.0);

    println!("Passed test_evaluator!");
}