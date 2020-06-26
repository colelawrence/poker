#[derive(Debug, PartialEq, Clone)]
pub enum CardSuit {
    Diamond,
    Heart,
    Club,
    Spade,
}

impl CardSuit {
    pub fn variants() -> impl Iterator<Item = Self> {
        use CardSuit::*;
        vec![Diamond, Heart, Club, Spade].into_iter()
    }
    pub fn variant_count() -> usize {
        4
    }
}

impl std::fmt::Display for CardSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CardSuit::*;
        match *self {
            Heart => write!(f, "♥"),
            Diamond => write!(f, "♦"),
            Spade => write!(f, "♠"),
            Club => write!(f, "♣"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum CardRank {
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
    King,
}
impl CardRank {
    pub fn all_variants() -> impl Iterator<Item = Self> {
        use CardRank::*;
        vec![
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
        ]
        .into_iter()
    }
    pub fn variant_count() -> usize {
        13
    }
}

impl std::fmt::Display for CardRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use CardRank::*;
        write!(
            f,
            "{}",
            match self {
                Ace => "Ace",
                Two => "Two",
                Three => "Three",
                Four => "Four",
                Five => "Five",
                Six => "Six",
                Seven => "Seven",
                Eight => "Eight",
                Nine => "Nine",
                Ten => "Ten",
                Jack => "Jack",
                Queen => "Queen",
                King => "King",
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Card(pub CardSuit, pub CardRank);

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.1, self.0)
    }
}
