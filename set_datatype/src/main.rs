use std::collections::HashSet;

fn main() {
    let mut set: HashSet<u32> = HashSet::new();

    for i in 1..6 {
        set.insert(i);
    }

    

    let contains = set.contains(&3);
    println!("{}", contains);

    set.remove(&1);
    for i in set{
        println!("{}", i);
    }
    //it doesn't have an order

}
