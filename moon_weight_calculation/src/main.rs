use std::io;
fn main() {
    println!("Enter your weight (kg):");
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    
    let weight: f32 = input.trim().parse().unwrap(); 
    let mars_weight= calculate_weight_on_moon(weight);
    println!("Weight on to the Moon is {} kg",mars_weight);
}

fn calculate_weight_on_moon(weight: f32) -> f32 {
    (weight/9.81) * 1.622
}
