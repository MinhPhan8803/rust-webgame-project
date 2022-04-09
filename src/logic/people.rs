mod card;
use rand::Rng; // 0.8.0
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
    pub fn new() -> Player {
        Player {
            card_total: 0,
            player_cards: Vec::new(),
        }
    }

    pub fn update_total(&self) {
        self.card_total = 0;
        for card in self.player_cards.iter() {
            if (card.CardName == CardName::Ace) {
                if (self.card_total <= 10) {
                    self.card_total += 11;
                }
            } else {
                self.card_total += card.value;
            }
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

    pub fn update_total(&self) {
        self.card_total = 0;
        for card in self.dealer_cards.iter() {
            if (card.CardName == CardName::Ace) {
                if (self.card_total <= 10) {
                    self.card_total += 11;
                }
            } else {
                self.card_total += card.value;
            }
        }
    }

    pub fn add_card(&mut self, new_card: Card) {
        self.dealer_cards.push(new_card);
        update_total();
    }

    pub fn deal(&all_cards: Deck) -> Card {
        let cards_in_deck: u32 = all_cards.len();
        let num = rand::thread_rng().gen_range(0..cards_in_deck);
        let card_return: Card = all_cards.at(num);
        all_cards.remove(num);
        return card_return;
    }
}