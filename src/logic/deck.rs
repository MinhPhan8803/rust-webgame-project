use strum::IntoEnumIterator; // 0.17.1
//use strum_macros::EnumIter; // 0.17.1
use super::card;
#[derive(Debug, Clone)]
pub struct Deck {
    pub all_cards: Vec<card::Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut temp = Vec::new();
        for cardtype in card::CardType::iter() {
            for cardval in card::CardName::iter() {
                let c1 = card::Card::new(cardtype.clone(), cardval);
                temp.push(c1);
            }
        }
        Deck {
            all_cards: temp,
        }
    }
}