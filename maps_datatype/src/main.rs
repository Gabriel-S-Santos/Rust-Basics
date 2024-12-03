use std::collections::HashMap;

fn main() {
    let mut map: HashMap<u32, String> = HashMap::new();
    let keys: [u32; 5] = [1,2,3,4,5];
    let values: [String; 5] = [
        String::from("Gabriel"), 
        String::from("Júlia"), 
        String::from("Lívia"),     
        String::from("Luiza"), 
        String::from("Dinho")];

    for i in 0..keys.len(){
        map.insert(keys[i], values[i].clone());
    }

    for i in map.iter() {
        println!("{}: {}", i.0, i.1);
    }


}
