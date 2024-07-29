pub fn print_elements(elements: &[String]) { // &[String] is a slice of Stings
    // it is better to use this compared to Vec<String> because of memory and
    // because slices are compatible with vec, but not vice versa.

    // for element in elements {
    //     println!("{}", element.to_string());
    // }
    elements
        .iter()
        //.map(|x| { format!("{}{}", x, x) })
        .for_each(|element| println!("{}", element.to_string()));
}
pub fn shorten_strings(elements: &mut [String]) {
    elements
        .iter_mut() // Instead of just giving a borrowed value
        // it gives a mutable borrowed value, allowing mutations
        .for_each(|el| el.truncate(1));
}
pub fn to_uppercase(elements: &[String]) -> Vec<String> {
    elements.iter()
        .map(|x| { x.to_uppercase() })
        //    .collect() // Collect implies the type of value based on the return type of the fn
        .collect::<Vec<String>>() // We use this if there is no return type, or we want to save mem
    //    .collect::<Vec<_>>() // also works and infers the value based on the iter
}
pub fn move_elements(from: Vec<String>, to: &mut Vec<String>) {
    from.into_iter().for_each(|el| to.push(el.to_string()));
}

pub fn explode(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect()
}
pub fn find_color_or(elements: &[String], filter: &str, fallback: &str) -> String {
    elements
        .into_iter()
        .find(|&el| el.to_lowercase().contains(filter))
        .map_or(fallback.to_string(), |el| el.to_string())
}