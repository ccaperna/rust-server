use std::io;

fn main() {

    //cargo expand
    //println!("Hello, world!");

    let mut input = String::new();
    io::stdin().read_line(&mut input);

    let mars_weight = weight_on_mars(40.0);
    println!("Your weight on mars: {}kg", mars_weight);
}

fn weight_on_mars(weight: f32) -> f32 {

   (weight / 9.81) * 3.711

}