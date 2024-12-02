fn main() {

    //ownership model
    let mut word_1: String = String::from("word");

    word_1 = print_ownership(word_1);

    println!("{}", word_1);

    //reference model
    let word_2: String = String::from("word 2");

    print_reference(&word_2);
  
}

fn print_ownership(word: String) -> String {
    println!("{}", word);
    word
}

fn print_reference(word: &String) {
    println!("{}", word);
}