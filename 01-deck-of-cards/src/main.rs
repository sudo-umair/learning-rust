use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let ranks = ["Ace", "2", "3", "King"];
        let mut cards = vec![];

        for suit in suits {
            for rank in ranks {
                let card = format!("{} of {}", rank, suit);
                cards.push(card);
            }
        }

        // let deck = Deck { cards };
        // return deck;

        // return Deck { cards };

        Self { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    println!("Here's your deck: {:#?}", deck);
    println!("Dealing 5 cards...");
    let dealt_cards = deck.deal(5);
    println!("Here are your dealt cards: {:#?}", dealt_cards);
}
