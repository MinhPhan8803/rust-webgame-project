use std::io::{stdin, stdout, Write};
mod game;
mod people;
mod deck;
mod card;
extern crate rand;
fn main() {
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
    let mut play = String::new();
    println!("Welcome to a black jack game:");
    println!("Type anything to continue");
    stdin().read_line(&mut play).unwrap();
    while play.as_str().to_ascii_lowercase() != "no" {

        //Game bootup
        println!("{}",play);
        play.clear();
        println!("new game started");
        let mut game = game::Blackjack::new();

        // Serve initial cards to players and dealers
        game.serve_card_player();
        println!("Player's current value is: {:?}", game.get_player().get_total());
        game.serve_card_dealer();
        game.serve_card_player();
        println!("Player's current value is: {:?}", game.get_player().get_total());
        game.serve_card_dealer();    

        println!("Please take a card:");
        let mut input = String::new();
        //prompt user to input yes or no
        //take input
        while input.as_str().to_ascii_lowercase() != "no" && game.check_state().is_none() {
            game.serve_card_player();
            input.clear();
            println!("Player's current value is: {:?}", game.get_player().get_total());
            println!("Say anything to keep dealing, or no to stop your turn");
            stdin().read_line(&mut input).unwrap();
        }

        //Announce Winner
        println!("The winner is:");
        let winner = match game.check_state() {
            Some(game::WinnerType::Player) => "The player".to_string(),
            Some(game::WinnerType::Dealer) => "The dealer".to_string(),
           Some(game::WinnerType::Tie) => "Tie".to_string(),
           None => "Nobody".to_string()
        };
        println!("{}", winner);
        
        // End
        println!("Say anything to keep playing, or no to stop the game");
        stdin().read_line(&mut play).unwrap();
    }
    println!("Come back to play next time!");
}

    // game.toggle_check();
    // while game.get_dealer().get_total() < 17 {
    //     game.serve_card_dealer();
    // }
    // // while player_value < 21 && still input
    // // modify game.player
    // // modify game.dealer 
    // let var = game.check_state().unwrap();
