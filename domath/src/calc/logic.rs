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
    let check: String = String::from(")+-/*%(^");
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
    for it in input.chars() {
        if ret.is_empty() {
            if !it.is_digit(10) && !is_valid_operator(it) {
                panic!("Invalid input -> {} <- as the first character!", &it);
            }
            ret.push(it.to_string());
        }
        else{
            ret.push(it.to_string());
        }
    }
    return ret;
}

pub fn print_token_list(tokens: std::vec::Vec<String>) {
    print!("{:?}", tokens);
}
/*
Use Binary tree to evaluate expressions
*/