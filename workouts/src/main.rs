use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculation slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generated_workout(intensity: u32, ramdom_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!",expensive_result);
        println!("Next, do {} situps!",expensive_result);
    } else {
        if ramdom_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println! ("Today, run for {} minutes!",expensive_result);
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_ramdom_number = 7;
    generated_workout(simulated_user_specified_value, simulated_ramdom_number);
    println!("Hello, world!");
}