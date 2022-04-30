use std::io::stdin;
mod game;
mod people;
mod deck;
mod card;
extern crate rand;
fn main() {
    println!("Hello, world!");

    //gameplay
    /*
    --> call deal 4 times, 2 cards go to player, 2 to dealer
    --> check card value --> whoever has 21 wins, tie if both have 21, else keep playing
    --> player chooses whether to keep their card deck as is, or to add more cards one by one
    --> if card value is exactly 21, player wins. If card value goes over, player loses. 
    --> player decides when to stop picking up cards. 
    --> dealer draws cards till their card value is 17 or higher
    --> if dealer hits 21 exactly, dealer wins, if dealer goes over, dealer loses. 
    --> whoever has card value under 21, but closest to 21 wins. 
    */
    let mut test = String::new();
    stdin().read_line(&mut test).unwrap();
    println!("{}", test);
    let game = game::Blackjack::new();
    game.serve_card_player();
    game.serve_card_dealer();
    game.serve_card_player();
    game.serve_card_dealer();
    println!("Please take a card:");
    while game.check_state().is_none() {
        let input = String::new();
        //prompt user to input yes or no
        //take input
        if input.as_str().to_ascii_lowercase() == "no" {
            break;
        }
        game.serve_card_player();
    }
    game.toggle_check();
    while game.dealer.get_total() < 17 {
        game.serve_card_dealer();
    }
    // while player_value < 21 && still input
    // modify game.player
    // modify game.dealer 
    let var = game.check_state().unwrap();
}
