use crate::card::*;

#[derive(Debug, PartialEq)]
pub struct Deck(Vec<Card>);

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut it = self.0.iter().peekable();
        if let Some(first) = it.next() {
            if it.peek().is_none() {
                return write!(f, "Deck has just 1 card, the {}.", first);
            }

            write!(f, "Deck has {} cards consisting of ", self.0.len())?;
            write!(f, "{}", first)?;
            while it.peek().is_some() {
                let card = it.next().unwrap();
                if it.peek().is_none() {
                    write!(f, ", and {}.", card)?;
                } else {
                    write!(f, ", {}", card)?;
                }
            }

            Ok(())
        } else {
            write!(f, "Deck is empty.")
        }
    }
}

#[test]
fn test_deck_empty_display() {
    assert_eq!(
        format!("{}", Deck(Vec::new())),
        "Deck is empty.".to_string(),
    )
}

#[test]
fn test_deck_full_display() {
    assert_eq!(
        format!(
            "{}",
            Deck(vec![
                Card(CardSuit::Club, CardRank::Ace),
                Card(CardSuit::Club, CardRank::Eight),
                Card(CardSuit::Diamond, CardRank::Two),
                Card(CardSuit::Diamond, CardRank::Jack),
                Card(CardSuit::Spade, CardRank::Jack),
            ])
        ),
        "Deck has 5 cards consisting of Ace ♣, Eight ♣, Two ♦, Jack ♦, and Jack ♠.".to_string(),
    )
}

#[test]
fn test_deck_one_display() {
    assert_eq!(
        format!("{}", Deck(vec![Card(CardSuit::Diamond, CardRank::Two),])),
        "Deck has just 1 card, the Two ♦.".to_string(),
    )
}

impl Deck {
    pub fn new(cards: Vec<Card>) -> Self {
        Deck(cards)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Deck(Vec::with_capacity(capacity))
    }

    pub fn peek_top(&self, number: usize) -> Option<Vec<&Card>> {
        if self.0.len() >= number {
            Some((0..number).map(|i| self.0.get(i).unwrap()).collect())
        } else {
            None
        }
    }
    pub fn draw_top(&mut self) -> Option<Card> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }

    pub fn add_to_bottom(&mut self, card: Card) {
        self.0.push(card)
    }

    pub fn add_to_top(&mut self, card: Card) {
        self.0.insert(0, card)
    }
}

impl From<Vec<Card>> for Deck {
    fn from(cards: Vec<Card>) -> Self {
        Deck(cards)
    }
}
