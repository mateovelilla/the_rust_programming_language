fn main() {

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.pop();

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let mut vector = vec![100, 32, 57];
    for i in &mut vector { 
        *i += 50;
    }
    println!("{:?}",vector); // [150, 82, 107]
    
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    
    
}
