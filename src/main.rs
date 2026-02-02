mod game;

fn main() {
    println!("Tetris");
    let result = game::start_game();

    match result {
        Ok(_) => print!("{esc}c\n", esc = 27 as char),
        Err(code) => println!("ERROR : Code {code}"),
    }
}
