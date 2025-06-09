// Example text:
// hello {icon:gold} {hinted:{i feel i need {named:apple_count} apples}{so good}}
//
// hinted left out for now ...will i have time ? i doubt it.
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum ParseNode<'a> {
    Nodes(Vec<ParseNode<'a>>),
    Text(&'a str),
    Space,
    Icon(&'a str),
    // Hinted(Box<ParseNode<'a>>, Box<ParseNode<'a>>),
    Named(&'a str),
}

// there ain't no backtracking on this train

pub(super) fn parse(input: &str) -> Option<ParseNode<'_>> {
    parse_nodes(input).and_then(|(parse_node, rest)| {
        if rest.is_empty() {
            Some(parse_node)
        } else {
            None
        }
    })
}

fn parse_nodes(input: &str) -> Option<(ParseNode<'_>, &str)> {
    let mut nodes = Vec::new();
    let mut current_input = input;
    while let Some((node, after_node_input)) = parse_node(current_input) {
        nodes.push(node);
        current_input = after_node_input;
        if let Some((space_node, after_space_input)) = parse_space(current_input) {
            nodes.push(space_node);
            current_input = after_space_input;
        }
    }
    if nodes.is_empty() {
        None
    } else if nodes.len() == 1 {
        Some((nodes.pop().unwrap(), current_input))
    } else {
        Some((ParseNode::Nodes(nodes), current_input))
    }
}

fn parse_node(input: &str) -> Option<(ParseNode<'_>, &str)> {
    parse_text(input)
        .or_else(|| parse_icon(input))
        // .or_else(|| parse_hinted(input))
        .or_else(|| parse_named(input))
}

fn parse_text(input: &str) -> Option<(ParseNode<'_>, &str)> {
    let bytes = input
        .chars()
        .take_while(|c| !(['{', '}'].contains(c) || c.is_whitespace()))
        .map(|c| c.len_utf8())
        .sum::<usize>();
    if bytes == 0 {
        None
    } else {
        Some((ParseNode::Text(&input[..bytes]), &input[bytes..]))
    }
}

fn parse_icon(input: &str) -> Option<(ParseNode<'_>, &str)> {
    let input = input.strip_prefix("{icon:")?;
    let (name, rest) = parse_name(input)?;
    let rest = rest.strip_prefix('}')?;
    Some((ParseNode::Icon(name), rest))
}

// fn parse_hinted(input: &str) -> Option<(ParseNode, &str)> {
//     let input = input.strip_prefix("{hinted:")?;
//     let input = input.strip_prefix('{')?;
//     let (left, rest) = parse_nodes(input)?;
//     let rest = rest.strip_prefix('}')?;
//     let rest = rest.strip_prefix('{')?;
//     let (right, rest) = parse_nodes(rest)?;
//     let rest = rest.strip_prefix('}')?;
//     let rest = rest.strip_prefix('}')?;
//     Some((ParseNode::Hinted(Box::new(left), Box::new(right)), rest))
// }

fn parse_named(input: &str) -> Option<(ParseNode<'_>, &str)> {
    let input = input.strip_prefix("{named:")?;
    let (name, rest) = parse_name(input)?;
    let rest = rest.strip_prefix('}')?;
    Some((ParseNode::Named(name), rest))
}

fn parse_name(input: &str) -> Option<(&str, &str)> {
    let (mut s1, mut rest) = parse_alpha(input)?;
    while let Some((s, new_rest)) = parse_alpha_numeric(rest) {
        s1 += s;
        rest = new_rest;
    }
    Some((&input[..s1], rest))
}

const ALPHA: RangeInclusive<char> = 'a'..='z';
const DIGITS: RangeInclusive<char> = '0'..='9';

fn parse_alpha_numeric(input: &str) -> Option<(usize, &str)> {
    input
        .chars()
        .next()
        .filter(|c| ALPHA.contains(c) || DIGITS.contains(c) || *c == '_')
        .map(|c| {
            let len = c.len_utf8();
            (len, &input[len..])
        })
}

fn parse_alpha(input: &str) -> Option<(usize, &str)> {
    input
        .chars()
        .next()
        .filter(|c| ALPHA.contains(c) || *c == '_')
        .map(|c| {
            let len = c.len_utf8();
            (len, &input[len..])
        })
}

fn parse_space(input: &str) -> Option<(ParseNode<'_>, &str)> {
    let len = input
        .chars()
        .take_while(|c| {
            // dbg!(c);
            c.is_whitespace()
        })
        .map(|c| c.len_utf8())
        .sum::<usize>();
    // dbg!(len);
    if len == 0 {
        None
    } else {
        Some((ParseNode::Space, &input[len..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_space() {
        assert_eq!(parse_space(" "), Some((ParseNode::Space, "")));
        assert_eq!(parse_space("  "), Some((ParseNode::Space, "")));
        assert_eq!(parse_space("{"), None);
        assert_eq!(parse_space("{icon:turn}left on"), None);
        assert_eq!(
            parse_space("{icon:turn}left on{icon:round}{named:round}"),
            None
        );
    }

    #[test]
    fn test_parse_alpha() {
        assert_eq!(parse_alpha("abc"), Some((1, "bc")));
        assert_eq!(parse_alpha("_abc"), Some((1, "abc")));
        assert_eq!(parse_alpha("1abc"), None);
        assert_eq!(parse_alpha(""), None);
    }

    #[test]
    fn test_parse_alpha_numeric() {
        assert_eq!(parse_alpha_numeric("abc"), Some((1, "bc")));
        assert_eq!(parse_alpha_numeric("_abc"), Some((1, "abc")));
        assert_eq!(parse_alpha_numeric("1abc"), Some((1, "abc")));
        assert_eq!(parse_alpha_numeric(""), None);
    }

    #[test]
    fn test_parse_name() {
        assert_eq!(parse_name("abc"), Some(("abc", "")));
        assert_eq!(parse_name("_abc"), Some(("_abc", "")));
        assert_eq!(parse_name("a1_bc"), Some(("a1_bc", "")));
        assert_eq!(parse_name("a1_bc}"), Some(("a1_bc", "}")));
        assert_eq!(parse_name("1abc"), None);
        assert_eq!(parse_name(""), None);
    }

    #[test]
    fn test_parse_text() {
        assert_eq!(parse_text("hello"), Some((ParseNode::Text("hello"), "")));
        assert_eq!(
            parse_text("hello{world}"),
            Some((ParseNode::Text("hello"), "{world}"))
        );
        assert_eq!(parse_text(""), None);
        assert_eq!(parse_text("{"), None);
    }

    #[test]
    fn test_parse_icon() {
        assert_eq!(
            parse_icon("{icon:gold}"),
            Some((ParseNode::Icon("gold"), ""))
        );
        assert_eq!(
            parse_icon("{icon:_abc}{def}"),
            Some((ParseNode::Icon("_abc"), "{def}"))
        );
        assert_eq!(parse_icon("abc"), None);
        assert_eq!(parse_icon("{icon:"), None);
    }

    #[test]
    fn test_parse_named() {
        assert_eq!(
            parse_named("{named:apple_count}"),
            Some((ParseNode::Named("apple_count"), ""))
        );
        assert_eq!(
            parse_named("{named:banana_count}{rest}"),
            Some((ParseNode::Named("banana_count"), "{rest}"))
        );
        assert_eq!(parse_named("abc"), None);
        assert_eq!(parse_named("{named:"), None);
    }

    // #[test]
    // fn test_parse_hinted() {
    //     assert_eq!(
    //         parse_hinted("{hinted:{hello}{world}}"),
    //         Some((
    //             ParseNode::Hinted(
    //                 Box::new(ParseNode::Text("hello")),
    //                 Box::new(ParseNode::Text("world"))
    //             ),
    //             ""
    //         ))
    //     );
    //     assert_eq!(
    //         parse_hinted("{hinted:{hello}{world}}{rest}"),
    //         Some((
    //             ParseNode::Hinted(
    //                 Box::new(ParseNode::Text("hello")),
    //                 Box::new(ParseNode::Text("world"))
    //             ),
    //             "{rest}"
    //         ))
    //     );
    //     assert_eq!(
    //         parse_hinted("{hinted:{{icon:gold}}{world}}"),
    //         Some((
    //             ParseNode::Hinted(
    //                 Box::new(ParseNode::Icon("gold")),
    //                 Box::new(ParseNode::Text("world"))
    //             ),
    //             ""
    //         ))
    //     );
    //     assert_eq!(parse_hinted("abc"), None);
    //     assert_eq!(parse_hinted("{hinted:"), None);
    //     assert_eq!(parse_hinted("{hinted:{}"), None);
    //     assert_eq!(parse_hinted("{hinted:{}}"), None);
    //     assert_eq!(parse_hinted("{hinted:{}{"), None);
    //     assert_eq!(parse_hinted("{hinted:{}}"), None);
    //     assert_eq!(parse_hinted("{hinted:{}{}"), None);
    //     assert_eq!(parse_hinted("{hinted:{{}}{}}"), None);
    // }

    #[test]
    fn test_parse_node() {
        assert_eq!(parse_node("hello"), Some((ParseNode::Text("hello"), "")));
        assert_eq!(
            parse_node("{icon:gold}"),
            Some((ParseNode::Icon("gold"), ""))
        );
        // assert_eq!(
        //     parse_node("{hinted:{hello}{world}}"),
        //     Some((
        //         ParseNode::Hinted(
        //             Box::new(ParseNode::Text("hello")),
        //             Box::new(ParseNode::Text("world"))
        //         ),
        //         ""
        //     ))
        // );
        assert_eq!(
            parse_node("{named:apple_count}"),
            Some((ParseNode::Named("apple_count"), ""))
        );
        assert_eq!(parse_node(""), None);
    }

    #[test]
    fn test_parse_nodes() {
        assert_eq!(
            parse_nodes("hello world"),
            Some((
                ParseNode::Nodes(vec![
                    ParseNode::Text("hello"),
                    ParseNode::Space,
                    ParseNode::Text("world")
                ]),
                ""
            ))
        );
        assert_eq!(parse_nodes("hello"), Some((ParseNode::Text("hello"), "")));
        assert_eq!(
            parse_nodes("hello{world}"),
            Some((ParseNode::Text("hello"), "{world}"))
        );
        assert_eq!(
            parse_nodes("hello{icon:gold}world"),
            Some((
                ParseNode::Nodes(vec![
                    ParseNode::Text("hello"),
                    ParseNode::Icon("gold"),
                    ParseNode::Text("world")
                ]),
                ""
            ))
        );
        assert_eq!(
            parse_nodes("{icon:gold}{named:apple}"),
            Some((
                ParseNode::Nodes(vec![ParseNode::Icon("gold"), ParseNode::Named("apple")]),
                ""
            ))
        );
        assert_eq!(parse_nodes(""), None);
    }

    #[test]
    fn test_parse() {
        assert_eq!(parse("hello"), Some(ParseNode::Text("hello")));
        assert_eq!(
            parse("hello{icon:gold}world"),
            Some(ParseNode::Nodes(vec![
                ParseNode::Text("hello"),
                ParseNode::Icon("gold"),
                ParseNode::Text("world")
            ]))
        );
        assert_eq!(parse("{hinted:{hi}{there}}hello{world}"), None);
        assert_eq!(parse(""), None);
        assert_eq!(parse("}"), None);

        assert_eq!(parse("{hinted:{hi}{there}}hello{world}}"), None);
        assert_eq!(parse("{hinted:{hi}{there"), None);
        // this was for debuging
        // assert_eq!(
        //     parse("{named:turns_left}{icon:turn}left on{icon:round}{named:round}"),
        //     None
        // );
    }
}
