#[cfg(test)]
use std::cmp;
#[cfg(test)]
use ::card;

#[test]
fn create_card() {
    let rank: card::Rank = card::Rank::Queen;
    let suit: card::Suit = card::Suit::Clubs;
    let card = card::Card::new(rank, suit);

    assert_eq!(card.rank, rank);
    assert_eq!(card.suit, suit);
}

#[test]
fn card_cmp_less() {
    let lesser = card::Card {
        rank: card::Rank::Five,
        suit: card::Suit::Hearts
    };
    let greater = card::Card {
        rank: card::Rank::Six,
        suit: card::Suit::Spades
    };

    assert_eq!(lesser.partial_cmp(&greater).unwrap(), cmp::Ordering::Less);
}

#[test]
fn card_cmp_greater() {
    let lesser = card::Card {
        rank: card::Rank::Five,
        suit: card::Suit::Hearts
    };
    let greater = card::Card {
        rank: card::Rank::Six,
        suit: card::Suit::Spades
    };

    assert_eq!(greater.partial_cmp(&lesser).unwrap(), cmp::Ordering::Greater);
}

#[test]
fn card_cmp_equal() {
    let heart_card = card::Card {
        rank: card::Rank::Five,
        suit: card::Suit::Hearts
    };
    let spade_card = card::Card {
        rank: card::Rank::Five,
        suit: card::Suit::Spades
    };

    assert_eq!(heart_card.partial_cmp(&spade_card).unwrap(), cmp::Ordering::Equal);
}

#[test]
fn card_id() {
    let two_spades = card::Card {
        rank: card::Rank::Two,
        suit: card::Suit::Spades
    };
    let id: i64 = 39;

    assert_eq!(two_spades.get_id(), id);
}

#[test]
fn suit_mask() {
    let two_clubs = card::Card {
        rank: card::Rank::Two,
        suit: card::Suit::Clubs
    };
    let ace_clubs = card::Card {
        rank: card::Rank::Ace,
        suit: card::Suit::Clubs
    };
    let five_spades = card::Card {
        rank: card::Rank::Five,
        suit: card::Suit::Spades
    };

    assert_eq!(-card::Suit::get_suit_mask(card::Suit::Clubs) & two_clubs.get_id(), 0);
    assert_eq!(-card::Suit::get_suit_mask(card::Suit::Clubs) & ace_clubs.get_id(), 0);
    assert!(-card::Suit::get_suit_mask(card::Suit::Clubs) & five_spades.get_id() != 0);
}
