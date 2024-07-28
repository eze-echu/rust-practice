use super::media::Media;
#[derive(Debug)]
pub struct Catalog {
    pub items: Vec<Media>,
}
impl Catalog {
    pub fn new() -> Self {
        Catalog {
            items: vec![]
        }
    }
    pub fn add(&mut self, media: Media) {
        self.items.push(media);
    }
    pub fn search(&self, index: usize) -> Option<&Media> {
        // Option is a default enum that has 2 possible values
        // Some(T), where it returns the value indicated by some
        // or None, that doesn't return anything
        // It's kinda similar to result, just without the error handling
        return match self.items.get(index) {
            Some(value) => {
                Some(value)
            }
            None => {
                None
            }
        };
    }
}
