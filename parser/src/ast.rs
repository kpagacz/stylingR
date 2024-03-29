use tokenizer::LocatedToken;

#[derive(Debug, Clone, PartialEq)]
pub struct CommentedToken<'a> {
    pub token: &'a LocatedToken<'a>,
    pub leading_comments: &'a [LocatedToken<'a>],
    pub inline_comment: Option<LocatedToken<'a>>,
}

impl<'a> CommentedToken<'a> {
    pub fn new(
        token: &'a LocatedToken<'a>,
        leading_comments: &'a [LocatedToken<'a>],
        inline_comment: Option<LocatedToken<'a>>,
    ) -> Self {
        Self {
            token,
            leading_comments,
            inline_comment,
        }
    }
}

impl<'a> From<&'a LocatedToken<'a>> for CommentedToken<'a> {
    fn from(token: &'a LocatedToken<'a>) -> Self {
        Self::new(token, &[], None)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Symbol(&'a CommentedToken<'a>),
    Literal(&'a CommentedToken<'a>),
    Comment(&'a CommentedToken<'a>),
    Term(Box<TermExpr<'a>>),
    Bop(
        &'a CommentedToken<'a>,
        Box<Expression<'a>>,
        Box<Expression<'a>>,
    ),
    Newline(&'a CommentedToken<'a>),
    Whitespace(&'a [CommentedToken<'a>]),
    EOF(&'a CommentedToken<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TermExpr<'a> {
    pub pre_delimiters: Option<&'a CommentedToken<'a>>,
    pub term: Expression<'a>,
    pub post_delimiters: Option<&'a CommentedToken<'a>>,
}

impl<'a> TermExpr<'a> {
    pub fn new(
        pre_delimiters: Option<&'a CommentedToken<'a>>,
        term: Expression<'a>,
        post_delimiters: Option<&'a CommentedToken<'a>>,
    ) -> Self {
        Self {
            pre_delimiters,
            term,
            post_delimiters,
        }
    }
}

impl<'a> From<Expression<'a>> for TermExpr<'a> {
    fn from(expr: Expression<'a>) -> Self {
        Self::new(None, expr, None)
    }
}
