use rand::{thread_rng, seq::SliceRandom};

#[derive(Debug)] //Defines attributes for the deck struct, it gives the compiler extra instructions
// `derive` is an attribute that specifies traits to automatically apply to the struct
// debug is a trait, traits are a set of functions
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        // List of 'suits'
        // List of 'values'
        // double nested for loop

        let suits = ["Hearts", "Clubs", "Diamonds", "Pikes"];
        let values = [
            "Ace",
            "Two",
            "Three",
            "Four",
            "Five",
            "Six",
            "Seven",
            "Eight",
            "Nine",
            "Ten",
            "Jack",
            "Queen",
            "King",
        ];
        // "variables" or bindings in rust are immutable by default
        // to have one mutable, you need to add 'mut' when declaring it
        let mut cards = vec![];
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }

        // Rust returns the last line's value if no return is specified
        // this is the same as running return deck {cards};
        // This doesn't work if there is a ';' since that would make it a fn
        Deck { cards }
        // if I run 1 == 0 as a last line the function would return the result of that
        //1 == 0
    }
    fn shuffle(&mut self) {
        let mut randomizer = thread_rng();
        self.cards.shuffle(&mut randomizer);
    }
    fn deal(&mut self, amount: usize) -> Result<Vec<String>, &str> {
        let check: Result<Vec<String>, &str>;
        if amount > self.cards.len() {
            check = Err("You asking for too much, chief");
            println!("{:?}", check);
            return check;
        }
        check = Ok(self.cards.split_off(self.cards.len() - amount));
        println!("{:?}", check);
        return check;
    }
}


fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    // {} Tells the formatter to expect a value
    // {:?} Tells the formatter that it should "Prettify it" or prepare it to be human-readable
    // {:#?} Makes it easier to read the values as it formats it line by line
    println!("Here is your deck: {:#?}", deck.cards);
    let hand = deck.deal(200);
    if hand.is_ok() {
        println!("Here is your hand: {:#?}", deck.deal(200));
    } else {
        println!("Error: {:?}", hand.unwrap_err())
    }
    /*
        Here is your hand: [
            "Ace of Hearts",
            "Two of Hearts",
        ]
     */
    println!("Hello, world!");
}
