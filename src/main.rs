extern crate linenoise;
extern crate lisp_rs;

use lisp_rs::atom;

fn main() {
    println!("lisp-rs");

    while let Some(input) = linenoise::input("> ") {
        linenoise::history_add(&input);

        match input.as_str() {
            "clear" => linenoise::clear_screen(),
            "exit" => break,
            _ => {
                let parsed = atom::parse_Expr(&input);

                match parsed {
                        Ok(result) => {
                            println!("{}", result);
                            match result.eval() {
                                Ok(value) => println!("{}", value),
                                Err(e) => println!("{}", e),
                            }
                        },
                        Err(e) => println!("error: {:?}", e),
                }
            }
        };
    }
}

#[test]
fn calc() {
    assert!(atom::parse_Expr("22").is_ok());
    assert!(atom::parse_Expr("(22)").is_ok());
    assert!(atom::parse_Expr("((((22))))").is_ok());
    assert!(atom::parse_Expr("((( ( 22 ) )))").is_ok());
    assert!(atom::parse_Expr("((22)").is_err());
}

#[test]
fn calc2() {
    assert!(atom::parse_Expr("(+ 1 2 3)").is_ok());
    assert!(atom::parse_Expr("(+ 4 (/ 5 6))").is_ok());
}
