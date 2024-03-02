pub fn add(first: f64, second: f64) -> f64 {
    return first + second;
}

pub fn subtract(first: f64, second: f64) -> f64 {
    return first - second;
}

pub fn divide(first: f64, second: f64) -> f64 {
    return first / second;
}

pub fn multiply(first: f64, second: f64) -> f64 {
    return first * second;
}

pub fn modulus(first: f64, second: f64) -> f64 {
    //look into mod for floats
    return first % second;
}

pub fn positive_exponent(subject: f64, exponent: usize) -> f64 {
    let mut ret: f64 = 1.0;
    for _ in 0..exponent {ret = ret * subject;}
    return ret;
}

pub fn negitive_exponent(subject: f64, exponent: isize) -> f64 {
    let mut ret: f64 = 1.0;
    for _ in (0..exponent).rev() {ret = ret / subject;} //test
    return ret;
}