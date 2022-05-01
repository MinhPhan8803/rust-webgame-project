use crate::card;
use crate::deck;
use crate::people;


pub enum WinnerType {
    Player,
    Dealer,
    Tie
}

#[derive(Debug)]
pub struct Blackjack {
    player: people::Player,
    dealer: people::Dealer,
    deck: deck::Deck,
    check: bool,
    is_over: bool,
}

impl Blackjack {
    // initialize player and dealer
    pub fn new() -> Blackjack {
        Blackjack {
            player: people::Player::new(),
            dealer: people::Dealer::new(),
            deck: deck::Deck::new(),
            check: false,
            is_over: false,
        }
    }

    // check game state
    pub fn toggle_check(&mut self) {
        self.check = !self.check;  
    }

    pub fn check_state(&mut self) -> Option<WinnerType> {
        let player_total: u32 = self.player.get_total();
        let dealer_total: u32 = self.player.get_total();
        if !self.is_over {
            self.is_over = true;
            if player_total == card::WINNING_POINT && dealer_total == card::WINNING_POINT {
                return Some(WinnerType::Tie);
            }
           if player_total == card::WINNING_POINT {
               return Some(WinnerType::Player);
           } 
           if player_total > card::WINNING_POINT {
               return Some(WinnerType::Dealer);
           }
           if dealer_total == card::WINNING_POINT {
                return Some(WinnerType::Dealer);
           } 
           if dealer_total > card::WINNING_POINT {
               return Some(WinnerType::Player);
           }
           if self.check {
               if dealer_total == player_total {
                   return Some(WinnerType::Tie);
               }
               if dealer_total > player_total {
                   return Some(WinnerType::Dealer);
               }
               return Some(WinnerType::Player);
           }
           self.is_over = false;
        }
       None
    }
    // get player
    pub fn get_player(&self) -> &people::Player {
        &self.player
    }
    // get dealer 
    pub fn get_dealer(&self) -> &people::Dealer {
        &self.dealer
    }
    // serve card
    pub fn serve_card_player(&mut self) {
        self.player.add_card(self.dealer.deal(&mut self.deck));
    }

    pub fn serve_card_dealer(&mut self) {
        let cards = self.dealer.deal(&mut self.deck);
        self.dealer.add_card(cards);
    }
}