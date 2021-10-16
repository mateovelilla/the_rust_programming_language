use std::thread;
use std::time::Duration;
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
impl <T>Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generated_workout(intensity: u32, ramdom_number: u32) {
    // The closure definition comes after the = to assign it to the variable expensive_closure.
    // To define a closure, we start with a pair of vertical pipes (|), inside which we specify the parameters to the closure;
    // this syntax was chosen because of its similarity to closure definitions in Smalltalk and Ruby. This closure has one parameter named num:
    // if we had more than one parameter, we would separate them with commas,
    // like |param1, param2|.
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!",expensive_result.value(intensity));
        println!("Next, do {} situps!",expensive_result.value(intensity));
    } else {
        if ramdom_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println! ("Today, run for {} minutes!",expensive_result.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_ramdom_number = 7;
    generated_workout(simulated_user_specified_value, simulated_ramdom_number);
    println!("Hello, world!");
}