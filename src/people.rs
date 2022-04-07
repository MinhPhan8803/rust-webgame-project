mod card;

#[derive(Debug, Clone)]
pub struct Player {
    pub card_total: u32,
    pub player_cards: std::vector<Card>(),
}

#[derive(Debug, Clone)]
pub struct Dealer {
    pub card_total: u32,
    pub dealer_cards: std::vector<Card>(),
}

impl Player {
    pub fn new(player_cards: std::vector<Card>()) -> Player {
        Player {
            card_total: 0,
            player_cards,
        }
    }

    pub fn update_total(&self) {
        self.card_total = 0;
        for card in self.player_cards.iter() {
            self.card_total += card.value;
        }
    }

    // check state
    /*
    *   true for good
    *   false for lose
    */
    pub fn check_state(&self) -> bool {
       update_total(); 
       if self.card_total <= WINNING_POINT {
          return true; 
       } else {
           return false;
       }
    }

    //add card
    pub fn add_card(&mut self, new_card: Card) {
        self.player_cards.push(new_card);
        update_total();
    }
    
    //getters
    pub fn get_total(&self) -> u32 {
        self.card_total
    }

    pub fn get_cards(&self) -> std::vector<Card>() {
        self.player_cards
    }
}

impl Dealer {
    pub fn new() -> Dealer {
        Dealer {
            card_total: 0,
            dealer_cards: Vec::new(),
        }
    }
}