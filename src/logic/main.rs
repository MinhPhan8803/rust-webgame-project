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
    let game = Blackjack::new();
    game.serve_card_player();
    game.serve_card_dealer();
    game.serve_card_player();
    game.serve_card_dealer();
    while game.check_state().is_none() {
        // while player_value < 21 && still input
        // modify game.player
        // modify game.dealer 
        
    }
    // var = game.check_state.unwrap
    // print var
}
