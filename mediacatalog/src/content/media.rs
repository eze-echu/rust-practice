#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    AudioBook { title: String },
    Podcast(usize),
    Placeholder,
}
impl Media {
    pub fn describe(&self) {
        // This works, but is not good
        //
        // if let Media::AudioBook { title } = self {
        //     println!("AudioBook: {:#?} \n", title);
        // } else if let Media::Movie { title, director } = self {
        //     println!("Movie: {:#?} \nDirector: {:#?}\n ", title, director);
        // } else if let Media::Book { title, author } = self {
        //     println!("Book: {:#?} \nAuthor: {:#?}\n ", title, author);
        // }
        match self {
            Media::AudioBook { title } => {
                println!("AudioBook: {:#?} \n", title);
            }
            Media::Movie { title, director } => {
                println!("Movie: {:#?} \nDirector: {:#?}\n ", title, director);
            }
            Media::Book { title, author } => {
                println!("Book: {:#?} \nAuthor: {:#?}\n ", title, author);
            }
            Media::Podcast(id) => {
                println!("Podcast Episode: {:#?}", id)
            }
            Media::Placeholder => {
                println!("Placeholder")
            }
            _ => {
                println!("HOW???");
            }
        }
    }
}