mod game;

fn main() {
    println!("Tetris");
    let result = game::start_game();

    match result {
        Ok(_) => println!("\n\nBye bye !"),
        Err(code) => println!("ERROR : Code {code}"),
    }
}
