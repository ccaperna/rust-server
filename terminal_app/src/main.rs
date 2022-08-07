use std::io;

//OWNERSHIP PRINCIPLES
//each value is owned by a variable
//when the owner goes out of scope, the value will be deallocated
//there can be only one owner at a time

fn main() {
    //cargo expand
    println!("Enter your weight in kilograms: ");

    let mut input = String::new();

    //borrow_string(&input);
    //own_string(input);

    io::stdin().read_line(&mut input).unwrap();

    println!("{}", input);

    let weight: f32 = input.trim().parse().unwrap();

    println!("{}", weight);

    let mars_weight = weight_on_mars(weight);
    println!("Your weight on mars: {}kg", mars_weight);
}

fn weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

/* 
fn borrow_string(s: &String) {
    //s.push_str("a")
    println!("{}", s);
}

fn own_string(s: String) {
    println!("{}", s);
}
*/
