enum Money {
    Coin,
    Bill,
}

fn define_cash(cash:Money)->String {
    match cash {
        Money::Coin => String::from("It's a Coin"),
        Money::Bill => String::from("It's a Bill"),
    }
}

fn main() {
    let cash_one = Money::Coin;
    let cash_two = Money::Bill;

    let what_is_first:String = define_cash(cash_one);
    let what_is_second:String = define_cash(cash_two);
    println!("{}",what_is_first);
    println!("{}",what_is_second);

}
