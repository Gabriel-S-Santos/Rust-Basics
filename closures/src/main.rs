fn main() {
    let add = |a: i32, b: i32| -> i32 {a+b};
    let result = add(2,3);
    println!("{}", result);

    let base: u32 = 10;
    let power = |exp: u32| -> u32 {base.pow(exp)} ;  
    println!("{}", power(2));

    let array: [u32; 5] = [1,2,3,4,5];
    array.iter().for_each(|&num| {
        println!("{}", num);
    })
}
