fn main() {
    let mut num: u32 = 0;
    let mut word: String = String::from("word");

    increase(&mut num);
    println!("{}", num);

    increase(&mut num);
    println!("{}", num);

    change_word(&mut word);
    println!("{}", word);

}

fn increase(num: &mut u32) {
    *num += 1;
}

fn change_word (word: &mut String) {
    *word = String::from("New Word");
}
