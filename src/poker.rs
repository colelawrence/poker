pub use crate::card::{Card, CardRank, CardSuit};
pub use crate::cashier::{Cashier, Chips};

pub use crate::deck::Deck;

// Game Related Structures

#[derive(Debug, PartialEq, Hash, Clone, Copy)]
#[repr(transparent)]
pub struct PlayerID(usize);

#[derive(Debug, PartialEq)]
pub struct Player {
    // id: PlayerID,
    name: String,
    chips: Chips,
}

#[derive(Debug, PartialEq)]
pub enum PlayerMove {
    Fold,
    Check,
    Bet(Chips),
    Raise(Chips),
}

#[derive(Debug, PartialEq, Clone)]
pub enum PlayerState {
    WaitingToBeDealt,
    Active(Card, Card),
    Folded,
}

impl PlayerState {
    pub fn is_active(&self) -> bool {
        match self {
            PlayerState::Active(_, _) => true,
            _ => false,
        }
    }
}

/// A hand is one round/turn in the game.
pub struct Hand {
    active: usize,
    players: Vec<(Player, PlayerState)>,
    deck: Deck,
    pot: Chips,
    // community_cards: Vec<Card>,
}

pub enum PlayResult<'a> {
    Winner(&'a Player),
    Continued,
}

impl Hand {
    pub fn new(players: Vec<Player>, deck: Deck) -> Self {
        Hand {
            active: 0,
            players: players
                .into_iter()
                .map(|player| (player, PlayerState::WaitingToBeDealt))
                .collect(),
            pot: Chips::new(),
            deck,
        }
    }

    pub fn play<'a>(&'a mut self, next_player_move: PlayerMove) -> Result<PlayResult<'a>, String> {
        {
            let active_player_and_state = self
                .players
                .get_mut(self.active)
                .expect("next player is active");

            match next_player_move {
                PlayerMove::Bet(chips) => {
                    // TODO: ensure amount of chips matches current contribution
                    self.pot.add(chips)
                }
                PlayerMove::Raise(chips) => {
                    // TODO: ensure amount of chips raises current contribution
                    self.pot.add(chips)
                },
                PlayerMove::Check => {
                    // TODO handle check
                }
                PlayerMove::Fold => {
                    // mem replace so we don't drop or clone cards
                    match std::mem::replace(&mut active_player_and_state.1, PlayerState::Folded) {
                        PlayerState::Active(first, second) => {
                            self.deck.add_to_bottom(first);
                            self.deck.add_to_bottom(second);
                            Ok(())
                        }
                        PlayerState::Folded => Err("Player already folded"),
                        PlayerState::WaitingToBeDealt => Err("Player was waiting to be dealt"),
                    }?;
                }
            }
        }

        let active_player = self
            .players
            .get(self.active)
            .expect("next player is active");

        let mut next_players = self
            .players
            .iter()
            .enumerate()
            .skip(self.active + 1)
            .chain(self.players.iter().enumerate().take(self.active));

        self.active = match next_players
            .find_map(|(idx, (_, state))| if state.is_active() { Some(idx) } else { None })
        {
            Some(next_player) => next_player,
            None => return Ok(PlayResult::Winner(&active_player.0)),
        };

        Ok(PlayResult::Continued)
    }

    pub fn deal(&mut self) {
        match self {
            Hand {
                ref mut deck,
                ref mut players,
                ..
            } => {
                for (player, state) in players.iter_mut() {
                    if let PlayerState::WaitingToBeDealt = state {
                        let first = deck.draw_top().expect("deck has enough cards");
                        let second = deck.draw_top().expect("deck has enough cards");
                        // replace state
                        *state = PlayerState::Active(first, second);
                    } else {
                        panic!(
                            "Found player not waiting to be dealt. {:?} in state {:?}",
                            player, state
                        )
                    }
                }
            }
        }
    }
}

// Unused but potentially interesting follow-on structures

pub enum PlayerPosition {
    Button,
    SmallBlind,
    BigBlind,
}

pub enum Deal {
    Hole,
    Flop,
    River,
    Turn,
}

#[derive(Debug)]
pub enum HandValues<'a> {
    HighCard(&'a Card),
    Pair(&'a Card, &'a Card),
    TwoPairs((&'a Card, &'a Card), (&'a Card, &'a Card)),
    ThreeOfAKind(&'a Card, &'a Card, &'a Card),
    Straight(&'a Card, &'a Card, &'a Card, &'a Card, &'a Card),
    Flush(&'a Card, &'a Card, &'a Card, &'a Card, &'a Card),
    FullHouse(&'a Card, &'a Card, &'a Card, &'a Card, &'a Card),
    FourOfAKind(&'a Card, &'a Card, &'a Card, &'a Card),
    StraightFlush(&'a Card, &'a Card, &'a Card, &'a Card, &'a Card),
}

trait CheckHand<'a> {
    fn check_hand(self) -> Vec<HandValues<'a>>;
}

impl<'a, IntoCardIter> CheckHand<'a> for IntoCardIter
where
    IntoCardIter: IntoIterator<Item = &'a Card> + Sized,
{
    fn check_hand(self) -> Vec<HandValues<'a>> {
        let mut it = self.into_iter();
        let ex = HandValues::Pair(it.next().unwrap(), it.next().unwrap());
        vec![ex]
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn test_chips(count: usize) -> Chips {
        Cashier::new(count).buy_chips(count).unwrap()
    }

    fn destroy_chips(chips: Chips) {
        Cashier::new(0).exchange_chips(chips);
    }

    fn clear_hand_chips(mut hand: Hand) {
        for (player, state) in hand.players.iter_mut() {
            destroy_chips(player.chips.take_all())
        }
        destroy_chips(hand.pot.take_all())
    }

    #[test]
    fn suits_display_as_icons_and_ranks_as_text() {
        // Hint: The format! macro makes use of the Display Trait.
        // How can we display different values for different enumerations?
        assert_eq!("Ace ♥", format!("{}", Card(CardSuit::Heart, CardRank::Ace)));
        assert_eq!(
            "Ten ♦",
            format!("{}", Card(CardSuit::Diamond, CardRank::Ten))
        );
        assert_eq!("Ace ♣", format!("{}", Card(CardSuit::Club, CardRank::Ace)));
        assert_eq!(
            "Jack ♠",
            format!("{}", Card(CardSuit::Spade, CardRank::Jack))
        );
    }

    #[test]
    fn new_hand_results_in_all_players_waiting_and_pot_of_zero() {
        let hand = Hand::new(
            vec![
                Player {
                    name: s("Will"),
                    chips: test_chips(10),
                },
                Player {
                    name: s("Jean"),
                    chips: test_chips(2),
                },
            ],
            simple_deck(),
        );

        assert_eq!(hand.pot.count(), 0);
        assert_eq!(hand.deck, simple_deck());
        assert_eq!(hand.players.len(), 2);
        for (_, state) in hand.players.iter() {
            assert_eq!(state, &PlayerState::WaitingToBeDealt);
        }

        clear_hand_chips(hand);
    }

    #[test]
    fn deal_provides_cards_from_deck_and_sets_first_player_active() {
        let will = Player {
            name: s("Will"),
            chips: test_chips(10),
        };
        let mut hand = Hand::new(
            vec![
                will,
                Player {
                    name: s("Jean"),
                    chips: test_chips(2),
                },
            ],
            // Note: simple_deck just has some aces
            simple_deck(),
        );

        let simple_deck = simple_deck();
        let top_4 = simple_deck
            .peek_top(4)
            .expect("simple deck has more than 2");

        hand.deal();
        assert_eq!(
            hand.players.first().unwrap().1,
            PlayerState::Active(top_4[0].clone(), top_4[1].clone()),
        );
        assert_eq!(
            hand.players.last().unwrap().1,
            PlayerState::Active(top_4[2].clone(), top_4[3].clone()),
        );

        clear_hand_chips(hand);
    }

    #[test]
    fn check_leaves_pot_untouched() {
        let mut hand = Hand::new(
            vec![
                Player {
                    name: s("Will"),
                    chips: test_chips(10),
                },
                Player {
                    name: s("Jean"),
                    chips: test_chips(2),
                },
            ],
            simple_deck(),
        );

        hand.play(PlayerMove::Check).expect("valid move");
        assert_eq!(hand.pot.count(), 0);

        // TODO: This should move the active player
        // assert_eq!(
        //     hand.players.first().unwrap().1,
        //     PlayerState::Dealt(HoleCards(
        //         simple_deck()[0].clone(),
        //         simple_deck()[1].clone()
        //     )),
        // );
        // assert_eq!(
        //     hand.players.last().unwrap().1,
        //     PlayerState::Active(HoleCards(
        //         simple_deck()[2].clone(),
        //         simple_deck()[3].clone()
        //     )),
        // );

        clear_hand_chips(hand);
    }

    #[test]
    fn bet_increases_the_pot() {
        let mut hand = Hand::new(
            vec![
                Player {
                    name: s("Will"),
                    chips: test_chips(10),
                },
                Player {
                    name: s("Jean"),
                    chips: test_chips(10), // <--- loser
                },
            ],
            simple_deck(),
        );

        let (next_player, _) = hand.players.first_mut().unwrap();
        let bet_3_chips = next_player.chips.take(3).expect("Will has enough chips");
        hand.play(PlayerMove::Bet(bet_3_chips)).expect("valid move");
        assert_eq!(hand.pot.count(), 3);

        // TODO: This should move the active player and decrease from active players chips

        clear_hand_chips(hand);
    }

    #[test]
    fn raise_increases_the_pot() {
        let mut hand = Hand::new(
            vec![
                Player {
                    name: s("Will"),
                    chips: test_chips(14),
                },
                Player {
                    name: s("Jean"),
                    chips: test_chips(2), // <--- loser
                },
            ],
            simple_deck(),
        );

        let (next_player, _) = hand.players.first_mut().unwrap();
        let raise_3_chips = next_player.chips.take(3).expect("Will has enough chips");
        hand.play(PlayerMove::Raise(raise_3_chips))
            .expect("valid move");
        assert_eq!(hand.pot.count(), 3);

        // TODO: This should move the active player and decrease from active players chips

        clear_hand_chips(hand);
    }

    #[test]
    fn fold_sets_the_player_state_to_folded() {
        let mut hand = Hand::new(
            vec![
                Player {
                    name: s("Will"),
                    chips: test_chips(10),
                },
                Player {
                    name: s("Jean"),
                    chips: test_chips(2), // <--- loser
                },
            ],
            simple_deck(),
        );

        hand.deal();
        hand.play(PlayerMove::Fold).expect("valid next move");
        assert_eq!(hand.players.first().unwrap().1, PlayerState::Folded);

        // TODO: This should move the active player

        clear_hand_chips(hand);
    }

    fn s(s: &str) -> String {
        s.to_owned()
    }

    fn simple_deck() -> Deck {
        Deck::new(vec![
            Card(CardSuit::Heart, CardRank::Ace),
            Card(CardSuit::Diamond, CardRank::Ace),
            Card(CardSuit::Club, CardRank::Ace),
            Card(CardSuit::Spade, CardRank::Ace),
        ])
    }

    fn full_deck() -> Deck {
        let mut deck = Deck::with_capacity(CardSuit::variant_count() * CardRank::variant_count());
        for ref suit in CardSuit::variants() {
            for ref rank in CardRank::all_variants() {
                deck.add_to_bottom(Card(suit.clone(), rank.clone()))
            }
        }
        deck
    }
}
