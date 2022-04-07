pub const WINNING_POINT: u32 = 21;

#[derive(Debug, Clone)]
pub struct Card {
    pub value: u32,
    pub name: CardName,
    pub type: CardType
}

pub enum CardName {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King
}

pub enum CardType {
    Club,
    Diamond,
    Heart,
    Spade
}

impl Card {
    pub fn new(type: CardType, name: CardName) -> Card {
        Card {
            type: type,
            name: name,
            value = match name {
               CardName::Ace => 1, 
               CardName::Two => 2,
               CardName::Three => 3,
               CardName::Four => 4,
               CardName::Five => 5,
               CardName::Six => 6,
               CardName::Seven => 7,
               CardName::Eight => 8,
               CardName::Nine => 9,
               CardName::Ten | CardName::Jack | CardName::Queen | CardName::King => 10
            }
        }
    }
}