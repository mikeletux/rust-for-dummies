use std::io;

fn main() {
    // Fahrenheit to Celsius program
    println!("Please provide the temperature in fahrenheit:");

    let mut temp_f = String::new();

    io::stdin()
            .read_line(&mut temp_f)
            .expect("Failed to read line");

    let temp_f: f64 = temp_f.trim().parse().expect("Please add the F temp in X.X format");

    println!("Temperature in Celsius is: {}", f_to_c(temp_f))
}

fn f_to_c(degrees_f: f64) -> f64 {
    (degrees_f - 32.0)*( 5.0/9.0 )
}