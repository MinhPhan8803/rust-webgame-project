use std::io::{stdin, stdout, Write};
use std::{thread,time};
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
    let one_second = time::Duration::from_secs(1);
    let two_second = time::Duration::from_secs(2);
    println!("Welcome to a black jack game:");
    thread::sleep(one_second);
    println!("Type anything to continue");
    stdin().read_line(&mut play).unwrap();
    while play.as_str().to_ascii_lowercase() != "no\n" {

        //Game bootup
        play.clear();
        println!("New game started");
        println!("\n");
        let mut game = game::Blackjack::new();
        thread::sleep(one_second);
        println!("Dealing initial cards");
        thread::sleep(two_second);

        // Serve initial cards to player and dealer
        game.serve_card_player();
        println!("Player's current value is: {:?}", game.get_player().get_total());
        thread::sleep(one_second);
        game.serve_card_dealer_open_card();
        thread::sleep(one_second);
        game.serve_card_player();
        println!("Player's current value is: {:?}", game.get_player().get_total());
        thread::sleep(one_second);
        game.serve_card_dealer();
        println!("\n");

        if game.check_state().is_none() {

        //prompt user to enter input to decide whether or not to get dealt another card
            println!("Please take a card, or say no to stop");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            thread::sleep(two_second);
            while input.as_str().to_ascii_lowercase() != "no\n" {
                input.clear();
                game.serve_card_player();
                println!("Player's current value is: {:?}", game.get_player().get_total());
                if !game.check_state().is_none() {
                    println!("Your value is over or equal 21, the game now stops");
                    break;
                }
                thread::sleep(one_second);
                println!("Say anything to keep dealing, or no to stop your turn.");
                println!("The game will automatically stop if your total card value is over 21.");
                println!("\n");
                thread::sleep(one_second);
                stdin().read_line(&mut input).unwrap();
            }
            println!("\n");
            thread::sleep(two_second);
            println!("Dealer picking up his card");
            thread::sleep(two_second);

            //Dealer picks up cards till his total value hits a minimum of 17
            while game.get_dealer().get_total() < 17 {
                game.serve_card_dealer();
            }
            println!("Dealer done picking cards");
            thread::sleep(two_second);
            println!("\n");
        }

            //Announce Winner
            let winner = match game.check_state() {
                Some(game::WinnerType::Player) => "The player".to_string(),
                Some(game::WinnerType::Dealer) => "The dealer".to_string(),
                Some(game::WinnerType::Tie) => "Tie".to_string(),
                None => game.get_state_no_winner_yet()
            };
            println!("The player's end value is {}", game.get_player().get_total());
            println!("The dealer's end value is {}", game.get_dealer().get_total());
            print!("The dealer's cards are: ");
            let cards_dealers = game.get_dealer().get_cards_dealer();
            for card in cards_dealers.iter() {
                print!("{:?}, ", card.clone().card_print());
            }
            println!("\n");
            println!("The winner is:");
            println!("{}", winner);
            println!("\n");
        
            // End
            println!("Say anything to start another game, or no to stop playing for now");
            stdin().read_line(&mut play).unwrap();
        }
        println!("Come back to play next time!");
    }
