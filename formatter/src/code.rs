use crate::config::FormattingConfig;

use parser::ast::Expression;
use tokenizer::tokens::CommentedToken;

use crate::format::{Doc, GroupDocProperties, ShouldBreak};
use std::rc::Rc;
use tokenizer::Token;

pub(crate) trait Code {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc>;
}

// Macro that creates a Doc::Group
macro_rules! group {
    ($doc:expr) => {
        Rc::new(Doc::Group(GroupDocProperties($doc, ShouldBreak::No)))
    };
}

// Macro that creates a Doc::Nest
macro_rules! nest {
    ($indent:expr, $doc:expr) => {
        Rc::new(Doc::Nest($indent, $doc))
    };
}

// Macro that creates a Doc::Cons
macro_rules! cons {
    ($first:expr, $second:expr) => {
        Rc::new(Doc::Cons($first, $second))
    };
}

// Macro that creates a Doc::Break
macro_rules! nl {
    ($txt:expr) => {
        Rc::new(Doc::Break($txt))
    };
}

// Macro that creates a Doc::Text
macro_rules! text {
    ($txt: expr) => {
        Rc::new(Doc::Text(Rc::from($txt)))
    };
}

// Macro that surrounds a doc with parentheses
macro_rules! delimited_doc {
    ($doc:expr, $ldelim: expr, $rdelim: expr) => {
        Rc::new(Doc::Group(GroupDocProperties(
            Rc::new(Doc::Cons(
                $ldelim,
                Rc::new(Doc::Cons(
                    Rc::new(Doc::Break("")),
                    Rc::new(Doc::Cons(
                        $doc,
                        Rc::new(Doc::Cons(Rc::new(Doc::Break("")), $rdelim)),
                    )),
                )),
            )),
            ShouldBreak::No,
        )))
    };
}

// TODO: Make this a macro
pub(crate) fn with_optional_break(
    first_doc: Rc<Doc>,
    second_doc: Rc<Doc>,
    break_text: &'static str,
) -> Rc<Doc> {
    cons!(cons!(first_doc, nl!(break_text)), second_doc)
}

impl<'a> Code for Token<'a> {
    fn to_docs(&self, _: &impl FormattingConfig) -> Rc<Doc> {
        match self {
            Token::Symbol(s) | Token::Literal(s) => Rc::new(Doc::Text(Rc::from(*s))),
            Token::Semicolon => Rc::new(Doc::Text(Rc::from(";"))),
            Token::Newline => Rc::new(Doc::Text(Rc::from("\n"))),
            Token::LParen => Rc::new(Doc::Text(Rc::from("("))),
            Token::RParen => Rc::new(Doc::Text(Rc::from(")"))),
            Token::LBrace => Rc::new(Doc::Text(Rc::from("{"))),
            Token::RBrace => Rc::new(Doc::Text(Rc::from("}"))),
            Token::LSubscript => Rc::new(Doc::Text(Rc::from("["))),
            Token::RSubscript => Rc::new(Doc::Text(Rc::from("]"))),
            Token::Comma => Rc::new(Doc::Text(Rc::from(","))),
            Token::Continue => Rc::new(Doc::Text(Rc::from("continue"))),
            Token::Break => Rc::new(Doc::Text(Rc::from("break"))),
            Token::If => Rc::new(Doc::Text(Rc::from("if"))),
            Token::Else => Rc::new(Doc::Text(Rc::from("else"))),
            Token::While => Rc::new(Doc::Text(Rc::from("while"))),
            Token::For => Rc::new(Doc::Text(Rc::from("for"))),
            Token::Repeat => Rc::new(Doc::Text(Rc::from("repeat"))),
            Token::In => Rc::new(Doc::Text(Rc::from("in"))),
            Token::Function => Rc::new(Doc::Text(Rc::from("function"))),
            Token::Lambda => Rc::new(Doc::Text(Rc::from("\\"))),
            Token::LAssign => Rc::new(Doc::Text(Rc::from("<-"))),
            Token::RAssign => Rc::new(Doc::Text(Rc::from("->"))),
            Token::OldAssign => Rc::new(Doc::Text(Rc::from("="))),
            Token::Equal => Rc::new(Doc::Text(Rc::from("=="))),
            Token::NotEqual => Rc::new(Doc::Text(Rc::from("!="))),
            Token::LowerThan => Rc::new(Doc::Text(Rc::from("<"))),
            Token::GreaterThan => Rc::new(Doc::Text(Rc::from(">"))),
            Token::LowerEqual => Rc::new(Doc::Text(Rc::from("<="))),
            Token::GreaterEqual => Rc::new(Doc::Text(Rc::from(">="))),
            Token::Power => Rc::new(Doc::Text(Rc::from("^"))),
            Token::Divide => Rc::new(Doc::Text(Rc::from("/"))),
            Token::Multiply => Rc::new(Doc::Text(Rc::from("*"))),
            Token::Minus => Rc::new(Doc::Text(Rc::from("-"))),
            Token::Plus => Rc::new(Doc::Text(Rc::from("+"))),
            Token::Help => Rc::new(Doc::Text(Rc::from("?"))),
            Token::And => Rc::new(Doc::Text(Rc::from("&&"))),
            Token::VectorizedAnd => Rc::new(Doc::Text(Rc::from("&"))),
            Token::Or => Rc::new(Doc::Text(Rc::from("||"))),
            Token::VectorizedOr => Rc::new(Doc::Text(Rc::from("|"))),
            Token::Dollar => Rc::new(Doc::Text(Rc::from("$"))),
            Token::Pipe => Rc::new(Doc::Text(Rc::from("|>"))),
            Token::Modulo => Rc::new(Doc::Text(Rc::from("%"))),
            Token::NsGet => Rc::new(Doc::Text(Rc::from("::"))),
            Token::NsGetInt => Rc::new(Doc::Text(Rc::from(":::"))),
            Token::Tilde => Rc::new(Doc::Text(Rc::from("~"))),
            Token::Colon => Rc::new(Doc::Text(Rc::from(":"))),
            Token::Slot => Rc::new(Doc::Text(Rc::from("@"))),
            Token::Special(s) => Rc::new(Doc::Text(Rc::from(*s))),
            Token::UnaryNot => Rc::new(Doc::Text(Rc::from("!"))),
            Token::InlineComment(s) => Rc::new(Doc::Text(Rc::from(*s))),
            Token::Comment(s) => Rc::new(Doc::Text(Rc::from(*s))),
            Token::EOF => Rc::new(Doc::Break("")),
        }
    }
}

impl Code for CommentedToken<'_> {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc> {
        // let mut docs = VecDeque::new();
        // // for comment in self.leading_comments {
        //     docs.push_back(comment.to_docs());
        //     // TODO: check if this works
        //     // Force a new line (I am not sure if the code already does it somewhere else)
        //     docs.push_back((INDENT, Mode::Flat, Rc::new(Doc::Text(Rc::from("\n")))));
        // }
        // docs.push_back(self.token.to_docs(config));
        // // if let Some(inline) = &self.inline_comment {
        //     docs.push_back(inline.to_docs());
        // }

        self.token.to_docs(config)
    }
}

fn join_docs_using_newlines<I, F>(docs: I, _config: &F) -> Rc<Doc>
where
    I: IntoIterator<Item = Rc<Doc>>,
    F: FormattingConfig,
{
    let mut docs = docs.into_iter();
    let mut res = Rc::new(Doc::Nil);

    if let Some(first_doc) = docs.next() {
        res = Rc::new(Doc::Cons(first_doc, res));
    }

    for next_doc in docs {
        res = Rc::new(Doc::Cons(res, Rc::new(Doc::Break("\n"))));
        res = Rc::new(Doc::Cons(res, next_doc));
    }

    res = Rc::new(Doc::Group(GroupDocProperties(res, ShouldBreak::Yes)));
    res
}

impl<'a> Code for Expression<'a> {
    fn to_docs(&self, config: &impl FormattingConfig) -> Rc<Doc> {
        let indent = config.indent();

        match self {
            Expression::Symbol(token) | Expression::Literal(token) | Expression::Comment(token) => {
                token.to_docs(config)
            }
            Expression::Term(term_expr) => {
                let (pre, term, post) = (
                    &term_expr.pre_delimiters,
                    &term_expr.term,
                    &term_expr.post_delimiters,
                );
                match (pre, term, post) {
                    (Some(pre), xprs, Some(post)) => {
                        delimited_doc!(
                            join_docs_using_newlines(xprs.iter().map(|t| t.to_docs(config)), config),
                            pre.to_docs(config),
                            post.to_docs(config)
                        )
                    }
                    _ => panic!("A term without matching delimiteres encountered"),
                }
            }
            Expression::Bop(op, lhs, rhs) => match op.token {
                Token::LAssign
                | Token::RAssign
                | Token::OldAssign
                | Token::Equal
                | Token::NotEqual
                | Token::LowerThan
                | Token::GreaterThan
                | Token::LowerEqual
                | Token::GreaterEqual
                | Token::Power
                | Token::Divide
                | Token::Multiply
                | Token::Minus
                | Token::Plus
                | Token::Help
                | Token::And
                | Token::VectorizedAnd
                | Token::Or
                | Token::VectorizedOr
                | Token::Pipe
                | Token::Modulo
                | Token::Tilde
                | Token::Special(_) => group!(nest!(
                    indent,
                    with_optional_break(
                        cons!(cons!(lhs.to_docs(config), text!(" ")), op.to_docs(config)),
                        rhs.to_docs(config),
                        " "
                    )
                )),
                Token::Dollar | Token::NsGet | Token::NsGetInt | Token::Colon | Token::Slot => {
                    group!(nest!(
                        indent,
                        with_optional_break(
                            cons!(cons!(lhs.to_docs(config), text!("")), op.to_docs(config)),
                            rhs.to_docs(config),
                            ""
                        )
                    ))
                },
                _ => panic!("Got a not a binary operator token inside a binary expression when formatting. Token: {:?}", &op.token)
            },
            Expression::Newline(_) => Rc::new(Doc::Break("\n")),
            Expression::EOF(_) => Rc::new(Doc::Nil),
            Expression::Whitespace(_) => Rc::new(Doc::Break("\n")),
            Expression::FunctionDef(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::format::{format_to_sdoc, simple_doc_to_string, Mode};

    use super::*;

    struct MockConfig;

    impl FormattingConfig for MockConfig {
        fn line_length(&self) -> i32 {
            120
        }
        fn indent(&self) -> i32 {
            0
        }
    }
    impl std::fmt::Display for MockConfig {
        fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unimplemented!()
        }
    }
    use std::collections::VecDeque;

    #[test]
    fn joining_docs_with_newlines_produces_newlines() {
        let docs = [
            Rc::new(Doc::Text(Rc::from("test"))),
            Rc::new(Doc::Text(Rc::from("test2"))),
        ];
        let mock_config = MockConfig {};
        let mut doc =
            VecDeque::from([(0, Mode::Flat, join_docs_using_newlines(docs, &mock_config))]);

        let sdoc = Rc::new(format_to_sdoc(0, &mut doc, &mock_config));

        assert_eq!(simple_doc_to_string(sdoc), "test\ntest2")
    }

    #[test]
    fn joinin_docs_with_newlines_does_nothing_for_just_one_doc() {
        let docs = [Rc::new(Doc::Text(Rc::from("test")))];
        let mock_config = MockConfig {};
        let mut doc =
            VecDeque::from([(0, Mode::Flat, join_docs_using_newlines(docs, &mock_config))]);

        let sdoc = Rc::new(format_to_sdoc(0, &mut doc, &mock_config));

        assert_eq!(simple_doc_to_string(sdoc), "test")
    }
}
