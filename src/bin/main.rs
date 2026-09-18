use lisp::{eval::Context, lexer::Lexer, parser::Parser};

/*
"(let x 2) (let x (add x x)) (write x)"
"(let x 2) (let y (add x x x)) (write y)"
"(let x (quote (add 1 2 3))) (write (eval x)) (write x)"
*/

fn main() {
    let source = std::env::args().nth(1).expect("expected lisp expression");

    let lexer = Lexer::new(source.as_str());
    let mut parser = Parser::new(lexer);
    let mut context = Context::new();

    while let Some(expr) = parser.parse() {
        match expr {
            Ok(expr) => {
                match expr.eval(&mut context) {
                    Err(e) => eprintln!("Evaluation error: {e:?}"),
                    _ => {},
                }
            }
            Err(e) => {
                eprintln!("Parser error: {e:?}")
            }
        }
    }
}

