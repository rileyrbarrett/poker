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
