#[derive(PartialEq, Debug)]
enum Rank {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _J,
    _Q,
    _K,
    _A,
}

#[derive(PartialEq, Debug)]
enum Suit {
    S,
    H,
    C,
    D,
}

pub struct Card {
    rank: Rank,
    suit: Suit,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_card() {
        let ace = Card {
            rank: Rank::_A,
            suit: Suit::S,
        };
        assert_eq!(ace.rank, Rank::_A);
    }
}
