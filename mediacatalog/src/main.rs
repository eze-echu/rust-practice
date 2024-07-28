use crate::content::media::Media;
use crate::content::catalog::Catalog;

mod content;
fn main() {
    let audiobook = Media::AudioBook {
        title: "Go the Fuck to Sleep".to_string()
    };
    let movie = Media::Movie {
        title: "Pulp Fiction".to_string(),
        director: "Quentin Tarantino".to_string(),
    };
    let book = Media::Book {
        title: "The Odyssey".to_string(),
        author: "Homer".to_string(),
    };
    let mut catalog = Catalog::new();
    catalog.add(audiobook);
    catalog.add(movie);
    catalog.add(book);
    println!("{:?}", catalog.search(2));
    println!("{:?}", catalog.search(5));
    // if this fails (None), the program panics. Use when debugging values
    println!("{:#?}", catalog.items.get(1).unwrap());
    // expect returns the value if present, if none, it panics and displays the msg
    // use to crash if something doesn't work
    println!("{:#?}", catalog.items.get(0).expect("No value at index"));
    // it either returns the unwrapped value or presents a default value
    // use when a fallback is available or preferred.
    println!("{:#?}", catalog.items.get(3).unwrap_or(&Media::Placeholder));
    // audiobook.describe();
    // movie.describe();
    // book.describe();
}
