use combine::{RangeStream, Parser, ParseError};
use combine::{between, choice, many, satisfy_map, token, try};
use combine::parser::combinator::{any_partial_state, AnyPartialState};
// use combine::parser::repeat::take_until;
use types::{Expr, List, Vector, Symbol};
use syntax::token::Token;

pub fn parser<'a, I>() -> impl Parser<Input = I, Output = Vec<Expr>, PartialState = AnyPartialState> + 'a
where
    I: RangeStream<Item = Token, Range = &'a [Token]> + 'a,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    any_partial_state(many(expr()))
}

parser!{
    #[inline]
    fn expr['a, I]()(I) -> Expr
    where [ 
        I: RangeStream<Item = Token, Range = &'a [Token]> + 'a,
    ]
        // I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        any_partial_state(
            choice((
                atom(),
                quote(),
                try(list()),
                try(vector()),
            ))
        )
    }
}

fn quote<'a, I>() -> impl Parser<Input = I, Output = Expr, PartialState = AnyPartialState> + 'a
where 
    I: RangeStream<Item = Token, Range = &'a [Token]> + 'a,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    any_partial_state(
        token(Token::Quote)
            .with(expr())
            .map(|e| {
                let quote_symbol = Expr::Sym(Symbol("quote".into()));
                Expr::List(List(vec![quote_symbol, e]))
            })
    )
}

fn atom<'a, I>() -> impl Parser<Input = I, Output = Expr, PartialState = ()> + 'a
where 
    I: RangeStream<Item = Token, Range = &'a [Token]> + 'a,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy_map(|token| match token {
        Token::Literal(lit) => Some(Expr::from(lit)),
        Token::Symbol(sym) => {
            if sym == "nil" {
                Some(Expr::Nil)
            } else {
                Some(Expr::from(Symbol(sym)))
            }
        },
        _ => None,
    })
}

fn list<'a, I>() -> impl Parser<Input = I, Output = Expr, PartialState = AnyPartialState> + 'a
where
    I: RangeStream<Item = Token, Range = &'a [Token]> + 'a,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    any_partial_state(
        between(
            token(Token::LParen),
            token(Token::RParen),
            many(expr()).map(List).map(Expr::List),
        ))
}

fn vector<'a, I>() -> impl Parser<Input = I, Output = Expr, PartialState = AnyPartialState> + 'a
where
    I: RangeStream<Item = Token, Range = &'a [Token]> + 'a,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    any_partial_state(
        between(
            token(Token::LBracket),
            token(Token::RBracket),
            many(expr()).map(Vector).map(Expr::Vector),
        ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty_list() {
        let input = vec![Token::LParen, Token::RParen];
        let output = vec![Expr::List(List(Vec::new()))];
        let empty: &[Token] = &[];
        assert_eq!(
            Ok((output, empty)),
            parser().parse(&*input)
        );
    }

    #[test]
    fn empty_vector() {
        let input = vec![Token::LBracket, Token::RBracket];
        let output = vec![Expr::Vector(Vector(Vec::new()))];
        let empty: &[Token] = &[];
        assert_eq!(
            Ok((output, empty)),
            parser().parse(&*input)
        );
    }
}