use std::{collections::HashMap, fmt::Write};

use crate::{parser::Expr::{self}};

pub struct Context {
    vars: HashMap<String, Expr>,
}

impl Context {
    pub fn new() -> Self {
        Self { 
            vars: HashMap::new(),
        }
    }
}


#[derive(Debug)]
pub enum Error {
    UndefinedVariable(String),
    UndefinedFunction(String),
    TypeMissmatch,
    IllegalFunctionCall,
    WrongNumberOfArgs
}

fn expect_num_args(list: &Vec<Expr>, num: usize) -> Result<(), Error> {
    if list.len() != num {
        Err(Error::WrongNumberOfArgs)
    }
    else {
        Ok(())
    }
}

impl Expr {
    pub fn eval(self, context: &mut Context) -> Result<Self, Error> {
        match self {
            Expr::Symbol(symbol) => {
                match context.vars.get(&symbol) {
                    Some(var) => Ok(var.clone()),
                    None => Err(Error::UndefinedVariable(symbol))
                }
            }
            Expr::List(mut list) if !list.is_empty() => {
                

                if let Expr::Symbol(op) = list.remove(0) {
                    match op.as_str() {
                        "quote" => {
                            Ok(list.remove(0))
                        }
                        "add" => {
                            let mut sum = 0;
                            for arg in list.drain(..) {
                                match arg.eval(context)? {
                                    Expr::Number(num) => sum += num,
                                    _ => { return Err(Error::TypeMissmatch)}
                                }
                            }
                            Ok(Expr::Number(sum))
                        },
                        "eval" => {
                            expect_num_args(&list, 1)?;
                            list.remove(0).eval(context)?.eval(context)
                        }
                        "write" => {
                            expect_num_args(&list, 1)?;
                            let expr = list.remove(0).eval(context)?;
                            println!("{expr}");
                            Ok(expr)
                        },
                        "let" => {
                            expect_num_args(&list, 2)?;
                            if let Expr::Symbol(key) = list.remove(0) {
                                let value = list.remove(0).eval(context)?;
                                context.vars.insert(key.clone(), value);
                                Ok(Expr::Symbol(key))
                            }
                            else {
                                Err(Error::IllegalFunctionCall)
                            }
                        }
                        _ => Err(Error::UndefinedFunction(op.to_string()))
                    }
                }
                else {
                    Err(Error::IllegalFunctionCall)
                }
            }
            expr => Ok(expr)
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::List(list) => {
                f.write_char('(')?;
                if let Some(expr) = list.first() {
                    expr.fmt(f)?;
                    for expr in list.iter().skip(1) {
                        f.write_char(' ')?;
                        expr.fmt(f)?;
                    }
                }
                f.write_char(')')?;
                Ok(())
            },
            Expr::Symbol(sym) => f.write_str(sym),
            Expr::Number(num) => f.write_str(&num.to_string()),
        }
    }
}


// pub mod test {
//     use crate::{eval::Context, lexer::Lexer, parser::Parser};


//     fn parse_and_eval(source: &str) -> Option<Expr> {
//         let lexer = Lexer::new(source);
//         let mut parser = Parser::new(lexer);
//         let mut result = None;
//         let mut context = Context::new();
//         while let Some(expr) = parser.parse() {
//             result.replace(expr.map(|e| e.eval(&mut context)))
//         }
//         result
//     } 

// }