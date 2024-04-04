/*
Global constants
*/

use crate::calc::calculation;

//For the parser
//If it works replace the things inside the valid op checks for tokenizer
//pub const operator_string: String = String::from("+-/*^)(");

/*
Determines whether a float could be converted to int without loss
*/
pub fn is_integer(answer: f64) -> bool {
    if answer.fract() == 0.0 {return true;}
    else {return false;}
}
/*
Converts float to int for readability
When the result of an operation yields an int
5.3 + 4.7 = 10 ! 10.0
*/
pub fn convert_to_int(target: f64) -> isize {
    return target as isize; //legal?
}
/*
Check to see if the non-numeric character is valid for math
*/
pub fn is_valid_operator(input: char) -> bool {
    let check: String = String::from(")+-/*%.(^");
    for c in check.chars() {
        if c == input {return true;}
    }
    return false;
}
/*
Check to see if the char is a special operator
*/
pub fn is_special_operator(input: char) -> bool {
    let check: String = String::from(".(-)");
    for c in check.chars() {
        if c == input {return true;}
    }
    return false;
}
/*
checks for instances where we can put a negative after
*/
pub fn neg_operator_check(input: char) -> bool {
    let check: String = String::from("+-/*^");
    for c in check.chars() {
        if c == input {return true;}
    }
    return false; 
}
/*
Take the different arguments, and returns one string
*/
pub fn combine_strings(arg_list: std::vec::Vec<String>) -> String {
    let mut ret: String = String::new();
    //Skip over first arg bc it is not relevent
    for i in 1..arg_list.len() {
        ret = ret + &arg_list[i];
    }
    return ret;
}

/*
Converts from a vector of string literals to a vector of Strings
C++ would never make me do this, the most strickly typed bullshit I ever used
*/
pub fn convert_literals(convert: std::vec::Vec<&str>) -> std::vec::Vec<String> {
    let mut ret: std::vec::Vec<String> = Vec::new();
    for it in convert {
        ret.push((&it).to_string());
    }
    return ret;
}

pub fn flip_boolean(to_flip: bool) -> bool {
    if to_flip == true {return false;}
    return true;
}

/*
Initializes a hashmap for the values of all of the operators for parser
*/
pub fn init_operator_hash_map() -> std::collections::HashMap<String, usize> {
    let mut ret: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
    ret.insert("^".to_string(), 3);
    ret.insert("*".to_string(), 2);
    ret.insert("/".to_string(), 2);
    ret.insert("+".to_string(), 1);
    ret.insert("-".to_string(), 1);
    return ret;
}

/*
Checks to see if String token is a number
Iterates through the string as chars and does an is_digit(10) compare
*/
pub fn is_token_digit(token: String) -> bool {
    for i in token.chars() {
        if i.is_digit(10) {
            return true;
        }
    }
    return false;
}

/*
Checks to see if token is an opertor
*/
pub fn is_token_operator(token: String) -> bool {
    let check: String = String::from("+-/*^)(");
    if token.len() == 1 {
        for c in check.chars() {
            if c == token.chars().nth(0).unwrap() {
                return true;
            }
        }
    }

    return false;
}

/*
Hashmap acting foolish
Dumb thing but what ever
*/
pub fn token_value(token: String) -> usize {
    if token == "+".to_string() || token == "-".to_string() {
        return 1;
    }
    else if token == "*".to_string() || token == "/".to_string() {
        return 2;
    }
    else if token == "^".to_string() {
        return 3;
    }
    return 0;
}

/*
Adding function to determine left associtivty
Without cannot handle exponents
*/
pub fn token_associativity(token: String) -> bool {
    if token == "+".to_string() ||
       token == "-".to_string() ||
       token == "*".to_string() ||
       token == "/".to_string() 
    {
        return true;
    }
    return false;
}

pub fn handle_math(token: String, a: f64, b: f64) -> f64 {
    if token == "+".to_string() {
        return calculation::add(a, b);
    }
    else if token == "-".to_string() {
        return calculation::subtract(a, b);
    }
    else if token == "*".to_string() {
        return calculation::multiply(a, b);
    }
    else if token == "/".to_string() {
        return calculation::divide(a, b);
    }
    else if token == "^".to_string() {
        return f64::powf(a, b);
    }
    else {
        panic!("Evaluator: Not a valid operator -> {} <- to do math on", token);
    }
}


//I miss oop
//The hashmap that corresponds to the values of the operators
//pub const operator_precedence: std::collections::HashMap<String, usize> = init_operator_hash_map();

/*---------------------------------------------------------------------------------------------
|                                                                                             |
|                                                                                             |
|                                       Big Three                                             |
|                                                                                             |
|                                                                                             |
---------------------------------------------------------------------------------------------*/

/*
If the return is empty -> push char on

If the previous char is a digit, and the current one is a digit -> then add to previous entry
If the previous char is a digit, and the current is not a digit -> then add to new entry
If the previous char is not a digit, and the current one is a digit -> then add to new entry

If the current char is an invalid character -> skip

Assumes valid expression to be tokenized
3+3 -> legal
3+ -> illegal 
3 -> illegal
*/

pub fn tokenize(input: String) -> std::vec::Vec<String> {
    let mut ret: std::vec::Vec<String> = Vec::new();
    let mut prev: char = '|';
    let mut is_negative_number: bool = false;
    
    for curr in input.chars() {
        if ret.is_empty() {
            if !curr.is_digit(10) && !is_valid_operator(curr) {
                panic!("Tokenizer: inavlid -> {} <- as first character", curr);
            }
            if curr == '-' {
                is_negative_number = true;
            }
            ret.push(curr.to_string());
        }
        else {
            if prev.is_digit(10) || prev == '.' || is_negative_number {
                if curr.is_digit(10) || curr == '.' {
                    ret.last_mut().unwrap().push(curr);
                }
                else if is_valid_operator(curr) {
                    ret.push(curr.to_string());
                    is_negative_number = false;
                }
                else {
                    panic!("Tokenizer: Invalid -> {} <- after -> {} <-", curr, prev);
                }
            }
            else if prev == '-' {
                if curr == '-' {
                    ret.push(curr.to_string());
                    is_negative_number = true;
                }
                else if curr.is_digit(10) {
                    ret.push(curr.to_string());
                    is_negative_number = true;
                }
                else {
                    panic!("Tokenizer: Invalid -> {} <- after -> {} <-", curr, prev);
                }
            }
            else if is_special_operator(prev) {
                if curr.is_digit(10) {
                    ret.push(curr.to_string());
                    is_negative_number = false;
                }
                else if is_valid_operator(prev) {
                    ret.push(curr.to_string());
                    is_negative_number = false;
                }
            }
            else if is_valid_operator(prev) {
                if curr.is_digit(10) {
                    ret.push(curr.to_string());
                    is_negative_number = false;
                }
                else if curr == '-' {
                    ret.push(curr.to_string());
                    is_negative_number = true;
                }
                else if is_special_operator(curr) {
                    ret.push(curr.to_string());
                    is_negative_number = false;
                }
                else {
                    panic!("Tokenizer: Inavlid -> {} <- after -> {} <-", curr, prev);
                }
            }
            else {
                panic!("Tokenizer: Not a valid operator -> {} <- after -> {} <-", curr, prev);
            }
        }
        prev = curr;
    }

    return ret;
}

/*
Implementation of Shunting Yard algorithmn
Converts the token vector into reverse polish notation
Stacks dont exist for some reason so we use a Vec
Yeah yeah I know they are built into Vec
*/
pub fn parser(tokens: std::vec::Vec<String>) -> std::vec::Vec<String> {
    let mut ret: std::vec::Vec<String> = Vec::new();
    let mut operator_stack: std::vec::Vec<String> = Vec::new();
    //Only run once per program execution so its ok ig
    let mut operator_value: std::collections::HashMap<String, usize> = init_operator_hash_map();

    for t in tokens {
        if is_token_digit(t.clone()){ //maybe make borrow
            ret.push(t);
        }
        else if t == "(".to_string() {
            operator_stack.push(t);
        }
        else if t == ")".to_string() {
            while !operator_stack.is_empty() && operator_stack.last().unwrap().to_string() != "(".to_string() { //example uses peek
                ret.push(operator_stack.pop().expect("Popping to return while in paren"));
            }
            let _ = operator_stack.pop();
        }
        else {
            /*
            while !operator_stack.is_empty() && 
                    operator_value[&t] <= operator_value[operator_stack.last().unwrap()]
            {
                ret.push(operator_stack.pop().expect("Popping inside the comparison"));
            }
            operator_stack.push(t); 
            */
            while !operator_stack.is_empty() && 
                    token_value(t.clone()) <= token_value(operator_stack.last().unwrap().to_string()) &&
                    token_associativity(t.clone()) 
                {
                ret.push(operator_stack.pop().expect("Popping inside the comparison"));
            }
            operator_stack.push(t);
        }
    }

    while !operator_stack.is_empty() {
        ret.push(operator_stack.pop().expect("Popping stack while stack is not empty"));
    }

    return ret;
}

/*
Converts reverse polish notation of tokens into a numerical value
*/
pub fn evaluator(reverse_polish: std::vec::Vec<String>) -> f64 {
    let mut stack: std::vec::Vec<f64> = Vec::new(); //will have to convert String -> f64
    for t in reverse_polish {
        if !is_token_operator(t.clone()) { //may be trouble since ( and ) eval to true also optimize so no clones
            let converted_float: f64 = t.parse::<f64>().unwrap();
            stack.push(converted_float);
        }
        else {
            let first: f64 = *stack.last().unwrap();
            let _ = stack.pop();
            let last: f64 = *stack.last().unwrap();
            let _ = stack.pop();
            stack.push(handle_math(t, last, first));
        }
    }
    return *stack.last().unwrap();
}