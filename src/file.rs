use {lexer, parser, ast};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use combine;
use combine::State;
use error::*;
use eval::Env;
use token::Token;

pub fn run(path: &str, mut env: &mut Env) -> Result<()> {
    let mut token_buf = Vec::<Token>::with_capacity(1024);

    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut source = String::with_capacity(128);
    buf_reader.read_to_string(&mut source)?;
    exec(State::new(source.as_str()), &mut token_buf, &mut env)
}

fn exec<S>(input: S, mut token_buf: &mut Vec<Token>, mut env: &mut Env) -> Result<()>
where
    S: combine::Stream<Item = char>
{
    let exprs = read(input, &mut token_buf)?;
    let result = eval(&exprs, &mut env)?;
    print(result);
    Ok(())
}

fn read<S>(input: S, mut token_buf: &mut Vec<Token>) -> Result<Vec<ast::Expr>>
where
    S: combine::Stream<Item = char>
{
    let tokens = lexer::lex(State::new(input))
        .map(|x| x.0)
        .unwrap_or(Vec::new());
    let all_tokens = token_buf.drain(..).chain(tokens).collect::<Vec<_>>();
    let token_iter = combine::from_iter(all_tokens.into_iter());
    let (exprs, unparsed) = parser::parse(combine::State::new(token_iter))?;
    token_buf.extend(unparsed.input);
    Ok(exprs)
}

fn eval(exprs: &[ast::Expr], mut env: &mut Env) -> Result<ast::Expr> {
    match exprs.split_last() {
        Some((last, rest)) => {
            for expr in rest {
                expr.eval(&mut env)?;
            }
            last.eval(&mut env)
        }
        None => Ok(ast::Expr::Nil),
    }
}

fn print(result: ast::Expr) {
    if result != ast::Expr::Nil {
        println!("{}", result);
    }
}
