use std::io::*;
pub fn input() -> String {
    let mut arguments = String::new();
    println! ("what wimm you like to fetch?: ");
    stdin().read_line(&mut arguments).expect("unvalide input");
    return arguments;
}
