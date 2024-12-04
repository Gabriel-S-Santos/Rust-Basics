fn find_element(array: &[i32], target: i32) -> Option<usize> {
    for (index, &value) in array.iter().enumerate() {
        if value == target{
            return Some(index);
        }
    }

    None
}

fn main() {
    let array: [i32; 5] = [1,2,3,4,5];

    match find_element(&array, 2) {
        Some(index) => println!("The number is in the index {}", index),
        None => println!("The number does not exist within the array"),
    }

    match find_element(&array, 10) {
        Some(index) => println!("The number is in the index {}", index),
        None => println!("The number does not exist within the array"),
    }
}
