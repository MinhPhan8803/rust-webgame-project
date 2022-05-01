//use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

pub const WINNING_POINT: u32 = 21;

#[derive(Debug, Clone)]
pub struct Card {
    pub value: u32,
    pub name: CardName,
    pub card_type: CardType
}

#[derive(Debug, Clone, EnumIter, Eq, PartialEq)]
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

#[derive(Debug, Clone, EnumIter, Eq, PartialEq)]
pub enum CardType {
    Club,
    Diamond,
    Heart,
    Spade
}

impl Card {
    pub fn new(card_type: CardType, name: CardName) -> Card {
        Card {
            card_type: card_type,
            name: name.clone(),
            value: match name {
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
    pub fn card_print(&mut self) -> String {
        let mut name = String::new();
        name.push_str(
            match self.name {
                CardName::Ace => "Ace", 
               CardName::Two => "Two",
               CardName::Three => "Three",
               CardName::Four => "Four",
               CardName::Five => "Five",
               CardName::Six => "Six",
               CardName::Seven => "Seven",
               CardName::Eight => "Eight",
               CardName::Nine => "Nine",
               CardName::Ten => "Ten",
               CardName::Jack => "Jack",
               CardName::Queen => "Queen",
               CardName::King => "King"
            }
        );
        name.push_str(
            match self.card_type {
               CardType::Club => " of Club",
               CardType::Diamond => " of Diamond",
               CardType::Heart => " of Heart",
               CardType::Spade => " of Spade",
            }
        );
        name
    }
}