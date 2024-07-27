#[derive(Debug)] //Defines attributes for the Deck struct, it gives the compiler extra instructions
// `derive` is an attribute that specifies traits to automatically apply to the struct
// debug is a trait, traits are a set of functions
struct Deck{
    cards: Vec<String>
}

impl Deck {
    fn new() -> Self{
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
            "King"
        ];
        // "variables" or bindings in rust are immutable by default
        // to have one mutable, you need to add 'mut' when declaring it
        let mut cards = vec![];
        for suit in suits{
            for value in values{
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        // Rust returns the last line's value if no return is specified
        // this is the same as running return Deck {cards};
        // This doesn't work if there is a ';' since that would make it a fn
        Deck { cards }
        // if I run 1 == 0 as a last line the function would return the result of that
        //1 == 0
    }
    fn shuffle(&self){
        // TODO: Add shuffling mechanism
    }
}


fn main() {
    let deck = Deck::new();
    deck.shuffle();
    // {} Tells the formatter to expect a value
    // {:?} Tells the formatter that it should "Prettify it" or prepare it to be human-readable
    // {:#?} Makes it easier to read the values as it formats it line by line
    println!("Here is your deck: {:#?}", deck.cards);
    println!("Hello, world!");
}
