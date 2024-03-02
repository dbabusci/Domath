pub fn is_integer(answer: f64) -> bool {
    if answer.fract() == 0.0 {return true;}
    else {return false;}
}

pub fn convert_to_int(target: f64) -> usize {
    return target as usize; //legal?
}

pub fn remove_white_space(input: String) -> String {
    for i in inputs.chars() {
        let ret: String = ret + i;
    }
    return ret;
}

pub fn tokenize(input: String) -> std::vec::Vec<String> {
    
}

/*
Use Binary tree to evaluate expressions
*/

