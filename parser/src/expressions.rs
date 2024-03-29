use nom::branch::alt;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;
use tokenizer::Token::*;

use crate::ast::CommentedToken;
use crate::ast::Expression;
use crate::ast::TermExpr;
use crate::token_parsers::*;

fn symbol_expr<'a, 'b: 'a>(
    tokens: &'b [CommentedToken<'a>],
) -> IResult<&'b [CommentedToken<'a>], Expression<'a>> {
    map(symbol, Expression::Symbol)(tokens)
}

fn literal_expr<'a, 'b: 'a>(
    tokens: &'b [CommentedToken<'a>],
) -> IResult<&'b [CommentedToken<'a>], Expression<'a>> {
    map(literal, Expression::Literal)(tokens)
}

fn term_expr<'a, 'b: 'a>(
    tokens: &'b [CommentedToken<'a>],
) -> IResult<&'b [CommentedToken<'a>], Expression<'a>> {
    alt((
        map(symbol_expr, |symbol| symbol),
        map(literal_expr, |literal| literal),
        map(
            tuple((
                lparen,
                delimited(many0(newline), term_expr, many0(newline)),
                rparen,
            )),
            |(lparen, term, rparen)| {
                Expression::Term(Box::new(TermExpr::new(Some(lparen), term, Some(rparen))))
            },
        ),
        map(
            tuple((
                lbrace,
                delimited(many0(newline), term_expr, many0(newline)),
                rbrace,
            )),
            |(lbrace, term, rbrace)| {
                Expression::Term(Box::new(TermExpr::new(Some(lbrace), term, Some(rbrace))))
            },
        ),
    ))(tokens)
}

// Precedence table from https://github.com/SurajGupta/r-source/blob/master/src/main/gram.y
// /* This is the precedence table, low to high */
// %left		'?'
// %left		LOW WHILE FOR REPEAT
// %right		IF
// %left		ELSE
// %right		LEFT_ASSIGN
// %right		EQ_ASSIGN
// %left		RIGHT_ASSIGN
// %left		'~' TILDE
// %left		OR OR2
// %left		AND AND2
// %left		UNOT NOT
// %nonassoc   	GT GE LT LE EQ NE
// %left		'+' '-'
// %left		'*' '/'
// %left		SPECIAL
// %left		':'
// %left		UMINUS UPLUS
// %right		'^'
// %left		'$' '@'
// %left		NS_GET NS_GET_INT
// %nonassoc	'(' '[' LBB

#[derive(Debug, Clone, PartialEq)]
enum Associativity {
    Left,
    Right,
    Non,
}

fn associativity(token: &CommentedToken) -> Associativity {
    match &token.token.token {
        Help | RAssign | Tilde | Or | VectorizedOr | And | VectorizedAnd | NotEqual | Plus
        | Minus | Multiply | Divide | Colon | Dollar | Slot | NsGet | NsGetInt => {
            Associativity::Left
        }
        LAssign | OldAssign | Power => Associativity::Right,

        _ => Associativity::Non,
    }
}

fn precedence(token: &CommentedToken) -> u8 {
    match &token.token.token {
        Help => 0,
        LAssign => 4,
        OldAssign => 5,
        RAssign => 6,
        Tilde => 7,
        Or | VectorizedOr => 8,
        And | VectorizedAnd => 9,
        GreaterThan | GreaterEqual | LowerThan | LowerEqual | Equal | NotEqual => 11,
        Plus | Minus => 12,
        Multiply | Divide => 13,
        Special(_) => 14,
        Colon => 15,
        Power => 17,
        Dollar | Slot => 18,
        NsGet | NsGetInt => 19,
        _ => panic!("{token:?} is not a binary operator"),
    }
}

fn is_binary_operator(token: &CommentedToken) -> bool {
    matches!(
        &token.token.token,
        Help | RAssign
            | Tilde
            | Or
            | VectorizedOr
            | And
            | VectorizedAnd
            | NotEqual
            | GreaterThan
            | GreaterEqual
            | LowerThan
            | LowerEqual
            | Equal
            | Plus
            | Minus
            | Multiply
            | Divide
            | Colon
            | Dollar
            | Slot
            | NsGet
            | NsGetInt
            | LAssign
            | OldAssign
            | Power
            | Special(_)
    )
}

// This implements the precedence climbing method described here:
// https://www.engr.mun.ca/~theo/Misc/exp_parsing.htm#climbing
struct ExprParser(u8);

impl ExprParser {
    fn parse<'a, 'b: 'a>(
        &self,
        mut lhs: Expression<'a>,
        mut tokens: &'b [CommentedToken<'a>],
    ) -> IResult<&'b [CommentedToken<'a>], Expression<'a>> {
        let mut lookahead = &tokens[0];
        while is_binary_operator(lookahead) && precedence(lookahead) >= self.0 {
            let op = lookahead;
            tokens = &tokens[1..];
            let (new_tokens, mut rhs) = term_expr(tokens)?;
            tokens = new_tokens;
            lookahead = &tokens[0];
            while is_binary_operator(lookahead)
                && (precedence(lookahead) > precedence(op)
                    || (associativity(lookahead) == Associativity::Right
                        && precedence(op) == precedence(lookahead)))
            {
                let q = precedence(op)
                    + (if precedence(lookahead) > precedence(op) {
                        1
                    } else {
                        0
                    });
                let parser = ExprParser(q);
                let (new_tokens, new_rhs) = parser.parse(rhs, tokens)?;
                rhs = new_rhs;
                tokens = new_tokens;
                lookahead = &tokens[0];
            }
            lhs = Expression::Bop(op, Box::new(lhs), Box::new(rhs));
        }
        Ok((tokens, lhs))
    }
}

pub(crate) fn expr<'a, 'b: 'a>(
    tokens: &'b [CommentedToken<'a>],
) -> IResult<&'b [CommentedToken<'a>], Expression<'a>> {
    let (tokens, term) = term_expr(tokens)?;
    if !tokens.is_empty() {
        let parser = ExprParser(0);
        parser.parse(term, tokens)
    } else {
        Ok((tokens, term))
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::commented_tokens;
    use crate::helpers::located_tokens;

    use super::*;
    use tokenizer::{
        LocatedToken,
        Token::{self},
    };

    fn binary_op_tokens() -> Vec<Token<'static>> {
        vec![
            Help,
            RAssign,
            Tilde,
            Or,
            VectorizedOr,
            And,
            VectorizedAnd,
            NotEqual,
            GreaterThan,
            GreaterEqual,
            LowerThan,
            LowerEqual,
            Equal,
            NotEqual,
            Plus,
            Minus,
            Multiply,
            Divide,
            Colon,
            Dollar,
            Slot,
            NsGet,
            NsGetInt,
            LAssign,
            OldAssign,
            Power,
            Special("%>%"),
        ]
    }

    #[test]
    fn symbol_exprs() {
        let located_tokens = located_tokens!(Symbol("a"));
        let tokens = commented_tokens(&located_tokens);
        let res = symbol_expr(&tokens).unwrap().1;
        assert_eq!(res, Expression::Symbol(&tokens[0]));
    }

    #[test]
    fn literal_exprs() {
        let located_tokens = located_tokens!(Literal("1"));
        let tokens = commented_tokens(&located_tokens);
        let res = literal_expr(&tokens).unwrap().1;
        assert_eq!(res, Expression::Literal(&tokens[0]));
    }

    #[test]
    fn expressions() {
        for token in binary_op_tokens() {
            let located_tokens = located_tokens!(Literal("1"), token, Literal("1"), EOF);
            let tokens = commented_tokens(&located_tokens);
            let res = expr(&tokens).unwrap().1;
            assert_eq!(
                res,
                Expression::Bop(
                    &tokens[1],
                    Box::new(Expression::Literal(&tokens[0])),
                    Box::new(Expression::Literal(&tokens[2]))
                )
            );
        }
    }

    #[test]
    fn right_associative_bop() {
        let located_tokens =
            located_tokens!(Literal("1"), Power, Literal("2"), Power, Literal("3"), EOF);
        let tokens = commented_tokens(&located_tokens);
        let res = expr(&tokens).unwrap().1;
        assert_eq!(
            res,
            Expression::Bop(
                &tokens[1],
                Box::new(Expression::Literal(&tokens[0])),
                Box::new(Expression::Bop(
                    &tokens[3],
                    Box::new(Expression::Literal(&tokens[2])),
                    Box::new(Expression::Literal(&tokens[4]))
                ))
            )
        );
    }

    #[test]
    fn left_associative_bop() {
        let located_tokens =
            located_tokens!(Literal("1"), Plus, Literal("2"), Plus, Literal("3"), EOF);
        let tokens = commented_tokens(&located_tokens);
        let res = expr(&tokens).unwrap().1;
        assert_eq!(
            res,
            Expression::Bop(
                &tokens[3],
                Box::new(Expression::Bop(
                    &tokens[1],
                    Box::new(Expression::Literal(&tokens[0])),
                    Box::new(Expression::Literal(&tokens[2]))
                )),
                Box::new(Expression::Literal(&tokens[4]))
            )
        );
    }

    #[test]
    fn bop_precedence() {
        let located_tokens = located_tokens!(
            Literal("1"),
            Multiply,
            Literal("2"),
            Plus,
            Literal("3"),
            EOF
        );
        let tokens = commented_tokens(&located_tokens);
        let res = expr(&tokens).unwrap().1;
        assert_eq!(
            res,
            Expression::Bop(
                &tokens[3],
                Box::new(Expression::Bop(
                    &tokens[1],
                    Box::new(Expression::Literal(&tokens[0])),
                    Box::new(Expression::Literal(&tokens[2]))
                )),
                Box::new(Expression::Literal(&tokens[4]))
            )
        )
    }
}
