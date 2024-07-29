use std::io::{Error, ErrorKind};
use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") { results.push(line.to_string()); }
    }
    results
}

fn main() -> Result<(), Error> {
    //These three are all different ways to handle errors
    /// # Try operator '?'
    ///
    /// When you have no way to handle the error that occurred
    let text = fs::read_to_string("logs.txt")?;
    let error_lines = extract_errors(&text);
    if error_lines.len() > 0 {
        fs::write("errors.txt", error_lines.join("\n"))?;
    } else {
        return Err(Error::new(ErrorKind::NotFound, "No errors were found")).unwrap();
    }

    //return Ok(());
    /// # unwarp() and expect() on result
    ///
    /// Should be used when debugging quickly or when intending to crash on an ERR
    let text = fs::read_to_string("logs.txt").expect("Unable to read file");

    let error_lines = extract_errors(&text);

    if error_lines.len() != 0 {
        fs::write(
            "errors.txt",
            error_lines.
                join("\n")).
            expect("Unable to write to file");
    } else {
        println!(
            "Error: {}",
            Error::new(
                ErrorKind::NotFound, "No errors were found",
            )
        )
    }
    /// # match or "if let"
    ///
    /// should be used when you have the intention of dealing with an error.
    match fs::read_to_string("logs.txt") { // WOOOOO RESULT
        Ok(read_text) => {
            //println!("{}", read_text.len());
            let lines = extract_errors(read_text.as_str());
            if lines.len() == 0 {
                Err(Error::new(ErrorKind::NotFound, "No error was found"))
            } else {
                println!("{:#?}", lines);
                match fs::write("errors.txt", lines.join("\n")) {
                    Ok(..) => Ok(()),
                    Err(e) => {
                        panic!("{}", e)
                    }
                }
            }
        }
        Err(error_message) => {
            panic!("{}", error_message);
        }
    }


    // match divide(8000.00, 10.00) {
    //     Ok(value) => {
    //         println!("{:.2}", value);
    //     }
    //     Err(value) => {
    //         panic!("{:}", value)
    //     }
    // }
    //
    // match validate_email("ghj@sadas.com".to_string()) {
    //     Ok(..) => println!("validated"),
    //     Err(error_message) => println!("{}", error_message)
    // }
}

// fn validate_email(email: String) -> Result<(), Error> {
//     if email.contains("@") {
//         Ok(())
//     } else {
//         Err(Error::new(ErrorKind::InvalidInput, "Invalid email"))
//     }
// }
//
// fn divide(a: f64, b: f64) -> Result<f64, Error> {
//     if b == 0.0 {
//         Err(Error::new(ErrorKind::InvalidInput, "Can't Divide by Zero"))
//     } else {
//         Ok(a / b)
//     }
// }