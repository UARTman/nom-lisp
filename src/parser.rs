use crate::ast::Node;
use nom::{
    branch::alt,
    bytes::streaming::{escaped, tag, take_while, take_while1},
    character::{
        streaming::{one_of},
        is_alphabetic, is_digit, is_newline, is_space,
    },
    combinator::recognize,
    error::context,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, terminated},
    IResult,
};

fn is_separator(c: u8) -> bool {
    is_space(c) || is_newline(c)
}

fn is_identifier_start(c: u8) -> bool {
    is_alphabetic(c) || c == b'+' || c == b'-' || c == b'*' || c == b'/' || c == b'_'
}

fn is_identifier_body(c: u8) -> bool {
    is_identifier_start(c) || is_digit(c)
}

pub fn node(input: &[u8]) -> IResult<&[u8], Node> {
    alt((identifier, list, string_literal, integer_literal, quote))(input)
}

pub fn identifier(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, seq) = context(
        "Identifier",
        recognize(pair(
            take_while1(is_identifier_start),
            take_while(is_identifier_body),
        )),
    )(input)?;
    let s = String::from(std::str::from_utf8(seq).unwrap());
    Ok((input, Node::Identifier(s)))
}

pub fn quote(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, node) = context("Quote", preceded(tag("'"), node))(input)?;
    Ok((input, Node::Quote(Box::new(node))))
}

pub fn list(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, nodes) = context(
        "list",
        delimited(
            terminated(tag("("), take_while(is_separator)),
            separated_list1(take_while1(is_separator), node),
            preceded(take_while(is_separator), tag(")")),
        ),
    )(input)?;
    Ok((input, Node::List(nodes)))
}

pub fn string_literal(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, s) = context(
        "String literal",
        delimited(
            tag("\""),
            escaped(take_while(|x| x != b'"'), '\\', one_of(r#""n\"#)),
            tag("\""),
        ),
    )(input)?;
    Ok((
        input,
        Node::StringLiteral(String::from(std::str::from_utf8(s).unwrap())),
    ))
}

pub fn integer_literal(input: &[u8]) -> IResult<&[u8], Node> {
    let (input, span) = context("Integer literal", take_while1(is_digit))(input)?;
    let ustr = std::str::from_utf8(span).unwrap();
    let i = ustr.parse().unwrap();
    Ok((input, Node::IntegerLiteral(i)))
}

#[cfg(test)]
mod test {
    use crate::parser::{node, Node};

    fn assert_parses_into(expect: Node, input: &[u8]) {
        let (input, output) = node(input).unwrap();
        assert!(input.is_empty(), "Input is {:?}", input);
        assert_eq!(expect, output);
    }

    #[test]
    fn test_list() {
        assert_parses_into(
            Node::List(vec![
                Node::Identifier("hello".into()),
                Node::Identifier("world".into()),
            ]),
            b"(hello world)",
        );
        assert_parses_into(Node::List(vec![Node::Identifier("test".into())]), b"(test)");
        assert_parses_into(
            Node::List(vec![
                Node::Identifier("print".into()),
                Node::IntegerLiteral(1),
                Node::StringLiteral("Hello {}".into()),
                Node::List(vec![Node::Identifier("getName".into())]),
            ]),
            b"(print 1 \"Hello {}\" (getName))",
        );
        assert!(node(b"()").is_err());
        assert_parses_into(
            Node::List(vec![
                Node::Identifier("a".into()),
                Node::Identifier("b".into()),
                Node::Identifier("c".into()),
            ]),
            b"( a b c )",
        )
    }

    #[test]
    fn test_quote() {
        assert_parses_into(Node::List(vec![
            Node::Quote(Box::new(Node::List(vec![Node::IntegerLiteral(1)]))),
            Node::Quote(Box::new(Node::IntegerLiteral(1))),
            Node::Quote(Box::new(Node::StringLiteral("x".into()))),
            Node::Quote(Box::new(Node::Quote(Box::new(Node::IntegerLiteral(1)))))
        ]), b"('(1) '1 '\"x\" ''1)");
        // let (remaining, n) = node(b"''")
    }
}
