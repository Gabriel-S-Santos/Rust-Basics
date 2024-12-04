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

}
