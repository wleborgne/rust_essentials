use std::f32::consts;

// Global values!
static MAX_HEALTH: i32 = 100;
static GAME_NAME: &'static str = "Monster Attack";

fn main() {
    // Game start: print a welcome message
    println!("Welcome to {}!", GAME_NAME);
    println!("You start with {} health points.", MAX_HEALTH);
    println!("{}", consts::PI)
}
