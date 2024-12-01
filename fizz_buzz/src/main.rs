fn main() {
    /*Print out the number of 1 to 15
    for each multiple of 3, print fizz
    for each multiple of 5, print buzz
    for each number that is multiple of 3 and 5, print FizzBuzz
     */

    let mut number: u32 = 1;

    while number < 16{
        if number % 3 == 0 && number % 5 == 0 {
            println!("FizzBuzz");
        }
        else if number % 3 == 0 {
            println!("Fizz");
        }
        else if number % 5 ==0 {
            println!("Buzz");   
        } else {
            println!("{}", number);
        }
        number += 1;
    }

    //using For loop

    println!("\nFOR LOOP:\n");
    let n: u32 = 15;

    for i in 1..n + 1{
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 ==0 {
            println!("Buzz");   
        } else {
            println!("{}", i);
        }
    }
}
