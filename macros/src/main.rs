macro_rules! my_vec {
    ($ ($x:expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);) * temp_vec
            
        }
    };
}

fn main() {
    let vector = my_vec![1,2,3,4,5];

    println!("{:?}", vector);
}
