use crate::card;
use crate::deck;
use rand::Rng; // 0.8.0
#[derive(Debug, Clone)]
pub struct Player {
    pub card_total: u32,
    pub player_cards: Vec<card::Card>,
}

#[derive(Debug, Clone)]
pub struct Dealer {
    pub card_total: u32,
    pub dealer_cards: Vec<card::Card>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            card_total: 0,
            player_cards: Vec::new(),
        }
    }

    pub fn update_total(&mut self) {
        self.card_total = 0;
        for card in self.player_cards.iter() {
            if card.name == card::CardName::Ace {
                if self.card_total <= 10 {
                    self.card_total += 11;
                }
            } else {
                self.card_total += card.value;
            }
        }
    }


    //add card
    pub fn add_card(&mut self, new_card: card::Card) {
        self.player_cards.push(new_card);
        self.update_total();
    }
    
    //getters
    pub fn get_total(&self) -> u32 {
        self.card_total
    }

    pub fn get_cards(&self) -> &Vec<card::Card> {
        &self.player_cards
    }
}

impl Dealer {
    pub fn new() -> Dealer {
        Dealer {
            card_total: 0,
            dealer_cards: Vec::new(),
        }
    }

    pub fn update_total(&mut self) {
        self.card_total = 0;
        for card in self.dealer_cards.iter() {
            if card.name == card::CardName::Ace {
                if self.card_total <= 10 {
                    self.card_total += 11;
                }
            } else {
                self.card_total += card.value;
            }
        }
    }

    pub fn add_card(&mut self, new_card: card::Card) {
        self.dealer_cards.push(new_card);
        self.update_total();
    }

    pub fn deal(&mut self, deck: &mut deck::Deck) -> card::Card {
        let num_cards_in_deck = deck.all_cards.len();
        let num = rand::thread_rng().gen_range(0, num_cards_in_deck);
        let card_return: card::Card = deck.all_cards[num].clone();
        deck.all_cards.remove(num);
        return card_return;
    }

    pub fn get_total(&self) -> u32 {
        self.card_total
    }

    pub fn get_cards_dealer(&self) -> &Vec<card::Card> {
        return &self.dealer_cards;
    }
}