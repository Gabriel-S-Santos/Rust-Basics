fn devide(a: i32, b: i32) -> Result<i32, String>{
    if b == 0 {
        return Err("Error: Division by zero is not allowed".to_string());
    }
    else{
        return Ok(a / b);
    }
}

fn main() {
    match devide(10, 2) {
        Ok(result) => println!("Result of division: {}", result),
        Err(error) => println!("Error: {}", error),
    }

    match devide(10, 0) {
        Ok(result) => println!("Result of division: {}", result),
        Err(error) => println!("{}", error),
    }
}
