enum CustomEnum {
    Enum1,
    Enum2(u32),
    Enum3(String),
    
}

fn main() {
    let en = CustomEnum::Enum2(12);

    match en {
        CustomEnum::Enum1 => println!("A type of Enum"),
        CustomEnum::Enum2(number) => println!("A type of Enum2({})", number),
        CustomEnum::Enum3(word) => println!("A type of Enum3: {}", word),
    }

    //matching tuples
    let tuple = (10, "hello");
    match tuple {
        (x, "hello") => println!("A tuple with the word hello"),
        (0, _) => println!("A tuple with value 0"),
        _ => println!("Not covered match case"),
    }

    //matching slices

    let array = [1, 2, 3, 4, 5];
    match &array[..] {
        [first, second, rest @ ..] => {
            println!("First: {}. Second: {}", first, second);
        },
        _ => println!("Not covered match case"),
    }

    //matching multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("One or Two"),
        3..=5 => println!("Between 3 and 5"),
        _ => println!("Not covered match case"),
    }

    //matching guards
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, boooom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }

}
