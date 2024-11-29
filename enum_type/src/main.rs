#[derive(Debug)]

enum Errors {
    InvalidAmount,
    NotEnoughFounds,
}

fn main() {
    let mut balance: u32 = 0;

    println!("{}", balance);
    deposit(&mut balance, 100);
    println!("{}", balance);

    pay(&mut balance, 0);
    pay(&mut balance, 101);
}

fn deposit(balance: &mut u32, amount: u32) {
    *balance += amount;
}

fn pay(balance: &mut u32, amount: u32) {
    if amount == 0 {
        println!("{:?}", Errors::InvalidAmount);
        return;
    }

    if amount > *balance {
        println!("{:?}", Errors::NotEnoughFounds);
        return;
    }

    *balance -= amount;
}