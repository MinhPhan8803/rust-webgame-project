mod card;
mod deck;
mod people;


#[derive(Debug)]
struct Blackjack {
    player: Player,
    dealer: Dealer,
    is_over: bool,
}

impl Blackjack {
    // initialize player and dealer
    pub fn new() {
        Blackjack {
            player: player::new(),
            dealer: dealer::new(),
            check: false,
            is_over: false
        }
    }

    // check game state
    enum WinnerType<A, B> {
        Player(A),
        Dealer(B),
        Tie(None)
    }
    fn check_state(&self) -> Option<WinnerType<Player, Dealer, Tie>> {
        let player_total: u32 = self.player.get_total();
        let dealer_total: u32 = self.player.get_total();
        if !is_over {
            is_over = true;
            if player_total == WINNING_POINT && dealer_total == WINNING_POINT {
                return Some(WinnerType::Tie);
            }
           if player_total == WINNING_POINT {
               return Some(WinnerType::Player);
           } 
           if player_total > WINNING_POINT {
               return Some(WinnerType::Dealer);
           }
           if dealer_total == WINNING_POINT {
                return Some(WinnerType::Dealer);
           } 
           if dealer_total > WINNING_POINT {
               return Some(WinnerType::Player);
           }
           if check {
               if dealer_total == player_total {
                   return Some(WinnerType::Tie);
               }
               if dealer_total > player_total {
                   return Some(WinnerType::Dealer);
               }
               return Some(WinnerType::Player);
           }
           is_over = false;
        }
       None
    }
    // get player
    fn get_player(&self) -> &Player {
        self.player
    }
    // get dealer 
    fn get_dealer(&self) -> &Dealer {
        self.dealer
    }
    // serve card
    fn serve_card_player(&self) {
        self.player.add_card(self.dealer.deal());
    }

    fn serve_card_dealer(&self) {
        self.dealer.add_card(self.dealer.deal());
    }
}