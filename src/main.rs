use std::io::stdin;

fn main() {

    //exercice 1

    let mut string = String::new();
    stdin().read_line(&mut string).expect("invalid input");
    let b = Box::new(string);
    println!("string is: {}",b);

    //exercice 2

    
}
