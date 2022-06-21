use std::io;

fn main() {
    let mut temp = String::new();
    println!("Enter a temp: ");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    
    let temp: f32 = temp.trim().parse()
        .expect("Cannot parse number");

    let c = f_to_c(temp);
    println!("The temp in c is: {}", c);
}

fn f_to_c(temp: f32) -> f32 {
    ((temp - 32.0)*5.0)/9.0
}
