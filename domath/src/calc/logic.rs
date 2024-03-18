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
If the return is empty -> push char on

If the previous char is a digit, and the current one is a digit -> then add to previous entry
If the previous char is a digit, and the current is not a digit -> then add to new entry
If the previous char is not a digit, and the current one is a digit -> then add to new entry

If the current char is an invalid character -> skip

Assumes valid expression to be tokenized
3+3 -> legal
3+ -> illegal 
3 -> illegal

Theory no negative numbers are real
Cumlative subtraction -- is minus negative number
--- is minus a negative negative number
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
            if(curr == '-') {
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
Maybe convert to postfix form?
*/

/*
The Tokenizer does not believe in negative numbers
For example -11 - -15 becomes - | 11 | -- | 15
This may cause problems not sure how to evaulate unary minus
*/
