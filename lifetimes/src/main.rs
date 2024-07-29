mod languages;

fn main() {
    let langs = vec![
        "rust".to_string(),
        "go".to_string(),
        "python".to_string(),
    ];
    let lang = languages::languages::Languages::new(langs);
    let go = languages::languages::next_language(&lang.list, "rust");
    let last = languages::languages::last_language(&lang.list);
    println!("{} {}", go, last);
    println!("{}", languages::languages::longest(go, lang.list[0].as_str()))
}
