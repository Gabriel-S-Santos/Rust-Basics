struct Surfer{
    pub height: u32,
    pub weight: u32,
    pub max_wave_height: u32,
    pub board_name: String
}

impl Surfer {
    fn say_aloha(&self) {
        println!("Alohaa");
    }

    fn ride_wave(&mut self, wave_height: u32){
        if wave_height > self.max_wave_height {
            self.max_wave_height = wave_height;
            println!("Wow! That's a new record! {} mts!", wave_height);
        } else {
            println!("That's Awesome!");
        }
    }

    fn chage_board_name(&mut self, new_boardname: String) {
        self.board_name = new_boardname;
    }


}

fn main() {
    let mut surfer: Surfer = Surfer{
        height: 180,
        weight: 70,
        board_name: String::from("Blocky"),
        max_wave_height: 0,
    };

    println!("Altura: {}, Peso: {}, Nome da Prancha:{}, Onda mais alta {}", 
    surfer.height, surfer.weight, surfer.board_name, surfer.max_wave_height);

    surfer.say_aloha();

    surfer.ride_wave(32);
    let old_board: String = surfer.board_name.clone();

    surfer.chage_board_name(String::from("novo nome"));
    print!("Nome antigo: {}\nNovo nome: {}\n\n", old_board, surfer.board_name);
    
}
