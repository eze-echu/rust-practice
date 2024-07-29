mod functions;
use functions::functions::*;

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    //print_elements(&colors);
    let screams = to_uppercase(&colors);
    //print_elements(&screams);
    shorten_strings(&mut colors);
    //print_elements(&colors);
    move_elements(screams, &mut colors);
    print_elements(&colors);
    println!("{:#?}", explode(&colors));
    println!("{:#?}", find_color_or(
        &colors,
        "re",
        "orange",
    ));
    // let mut colors_iter = colors.iter(); //iter are completely new structs in memory
    // They possess the following data
    // 1. A pointer to the object they are iterating over
    // 2. a pointer to the value they are currently accessing
    // 3. a pointer to the end of the object (to catch if it went pass-the-element)
    // colors_iter.next();
    // Next() provides a Some(T) of the value that is present in the 2nd binding of the pointer
    // it also moves that 2nd binding to the next value in the object being iterated
    //
    // Once it reaches the end of the object, the pointer to current value will be
    // equal to pointer to the end, and the iter will return a None (Options None)
}
