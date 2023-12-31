// use nom::{
//     branch::alt,
//     character::complete::{multispace0, multispace1},
//     combinator::{map, opt},
//     multi::many0,
//     sequence::{delimited, tuple},
//     IResult,
// };
//
// use crate::{
//     ast::{AstNode, Expression},
//     expression::expr_or_assign_or_help,
//     helpers::CodeSpan,
// };
//
// pub fn program(input: CodeSpan) -> IResult<CodeSpan, Vec<AstNode>> {
//     many0(alt((
//         delimited(
//             multispace0,
//             expr_or_assign_or_help,
//             tuple((
//                 multispace0,
//                 opt(nom::character::complete::char(';')),
//                 multispace0,
//             )),
//         ),
//         map(multispace1, |_| {
//             AstNode::new(
//                 Box::new(Expression::Expressions(vec![])),
//                 input.location_line(),
//                 input.location_offset(),
//             )
//         }),
//     )))(input)
// }
//
// #[cfg(test)]
// mod tests {
//     use crate::ast::Literal;
//
//     use super::*;
//
//     #[test]
//     fn test_program() {
//         let tests = [
//             // surrounding empty lines
//             (
//                 r#"
//         TRUE
//         "#,
//                 vec![AstNode::new(
//                     Box::new(Expression::Literal(Literal::True)),
//                     0,
//                     0,
//                 )],
//             ),
//             // empty program
//             ("", vec![]),
//             // multiple lines
//             (
//                 r#"
//         TRUE
//
//         TRUE
//         "#,
//                 vec![
//                     AstNode::new(Box::new(Expression::Literal(Literal::True)), 0, 0),
//                     AstNode::new(Box::new(Expression::Literal(Literal::True)), 0, 0),
//                 ],
//             ),
//             // multiline expression
//             (
//                 r#"
//         if
//         (FALSE) {} else
//         if (FALSE) {}
//         "#,
//                 vec![AstNode::new(
//                     Box::new(Expression::If(
//                         vec![
//                             (
//                                 AstNode::new(Box::new(Expression::Literal(Literal::False)), 0, 0),
//                                 AstNode::new(Box::new(Expression::Expressions(vec![])), 0, 0),
//                             ),
//                             (
//                                 AstNode::new(Box::new(Expression::Literal(Literal::False)), 0, 0),
//                                 AstNode::new(Box::new(Expression::Expressions(vec![])), 0, 0),
//                             ),
//                         ],
//                         None,
//                     )),
//                     0,
//                     0,
//                 )],
//             ),
//         ];
//         for (input, expected) in tests {
//             let input = CodeSpan::new(input);
//             assert_eq!(program(input).unwrap().1, expected);
//         }
//     }
// }
