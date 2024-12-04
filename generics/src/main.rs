fn main() {
    fn max<T: PartialOrd>(x: T, y: T) -> T {
        if x >= y {
            x
        } else {
            y
        }
    }

    let x: i32 = 10;
    let y: i32 = 2;
    let result = max(x, y);

    println!("The maximum value is: {}", result);
}
