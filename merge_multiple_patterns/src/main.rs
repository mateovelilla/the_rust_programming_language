fn main() {
    let x = 1;
    let y = 5;
    let z = 'c';
    match x {
        1 | 2 => println!("one or two"), // the " | "" symbols is like or
        3 => println!("three"),
        _ => println!("anithing"), // the " _ " is like else in conditionals
    }

    match y {
        1..=5 => println!("one through five"), // this conditional include 5 for example 1 | 2 | 3 | 4 | 5
        _ => println!("something else"),
    }
    match z {
        'a'..='j' => println!("Early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else")
    }

}
