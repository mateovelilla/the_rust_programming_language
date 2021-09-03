fn largest(list: &[i32]) -> i32{
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34 , 66, 43, 55, 64];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let number_list = vec![10, 20, 30, 40, 50, 60, 70];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
