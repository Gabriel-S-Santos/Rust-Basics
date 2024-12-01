fn main() {
    //verify is 42 is even and if it is divisible for 3.

    let number: i32 = 42;

    if number % 2 == 0 {
        println!("{} is even", number);
    }
    else {
        println!("{} is odd", number);
    }

    if number % 2 == 0 && number % 3 == 0 {
        println!("{} is even and divisible by 3", number);
    }
}
