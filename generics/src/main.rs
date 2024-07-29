use num_traits::{ToPrimitive};

fn pythagoras_theorem<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let x = a.to_f64().unwrap();
    let y = b.to_f64().unwrap();

    ((x.powi(2)) + (y.powi(2))).sqrt()
}

fn main() {
    let a: f64 = 3.0;
    let b = 4.0;

    // let a_f64 = a as f64;
    // let a_f64_nums = a.to_f64().unwrap(); // uses num_traits::ToPrimitive;
    println!("{}", pythagoras_theorem(a, b));
}
