pub struct Languages {
    pub list: Vec<String>,
}

impl Languages {
    pub fn new(values: Vec<String>) -> Self {
        Languages { list: values.clone() }
    }
}
// we have to declare 'a since we have two references as parameters
// rust guesses by default what the lifetime is if you only have one reference and return a reference
// but when we have 2 references and return a reference, we must specify which lifetime it uses

// fn func(foo: &[String]) -> &str // guesses that the lifetime will be of foo
//
// fn func(foo: i32, bar: &[i32]) -> &str // guesses that the lifetime will be of bar
//
// fn func(&self, foo: &str) -> &str // if &self is present, always guesses that if you want another
// you have to specify with lifetime annotations
//
// if you omit the lifetime annotation it is called elision (verb: elide)
pub fn next_language<'a>(list: &'a [String], current: &str) -> &'a str {
    let mut found = false;
    for lang in list {
        if found {
            return lang;
        }

        if lang == current {
            found = true;
        }
    }
    list.last().unwrap()
}
pub fn last_language(list: &[String]) -> &str {
    list.iter().last().unwrap().as_str()
}
pub fn longest<'a>(one: &'a str, two: &'a str) -> &'a str {
    if one.len() >= two.len() {
        one
    } else {
        two
    }
}