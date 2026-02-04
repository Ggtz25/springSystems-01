
const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{

        (f - 32.0) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64{

        (c * 9.0 / 5.0) + 32.0
}


fn main() {
    
    let mut temp_fahrenheit: f64 = FREEZING_POINT_F;

    let temp_celsius = fahrenheit_to_celsius(temp_fahrenheit);
    println!("{temp_fahrenheit}F = {:.2}C", temp_celsius);

    for _ in 0..5 {
        temp_fahrenheit += 1.0;
        let temp_celsius = fahrenheit_to_celsius(temp_fahrenheit);
        println!("{temp_fahrenheit}F = {:.2}C", temp_celsius);
    }
}
