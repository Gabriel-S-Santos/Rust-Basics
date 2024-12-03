fn main() {
    let mut vec: Vec<u32> = vec![1,2,3,4,5];

    for i in vec.iter() {
        println!("{}", i);
    }

    vec[0] = 0;


    println!("after change");
    for i in vec.iter() {
        println!("{}", i);
    }

    vec.push(6); //this creates another memory reference, another vector
    println!("after add");
    for i in vec.iter() {
        println!("{}", i);
    }

    let popped = vec.pop().unwrap();

    println!("O valor excluído: {}", popped);
    for i in vec.iter() {
        println!("{}", i);
    }
    let popped = vec.pop().unwrap();

    println!("O valor excluído: {}", popped);
    for i in vec.iter() {
        println!("{}", i);
    }

    println!("Tamanho do Vetor: {} elementos.", vec.len());
}
