fn main() {
    let celsius = convert_fahrenheit_to_celsius(3.0);
    println!("2.0 fahrenheit is {} celsius", celsius);
}
fn convert_fahrenheit_to_celsius(fahrenheit:f64) -> f64 {
    (fahrenheit - 32.0) / 1.8000
}
