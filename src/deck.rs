use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1
mod card;
#[derive(Debug, Clone)]
pub struct Deck {
    pub all_cards: std::vector<Card>();
}

impl Deck {
    fn new() -> Deck {
        temp = Vec::new();
        for cardtype in CardType::iter {
            for cardval in CardName::iter {
                Card c1 = new Card(cardtype, cardval);
                temp.push(c1);
            }
        }
        Deck {
            all_cards = temp;
        }
    }
}