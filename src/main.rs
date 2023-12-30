//! JSON Example following [pest-book: JSON-example](https://pest.rs/book/examples/json.html)
use pest::Parser;
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use std::fs;

// #[derive(Parser)]
// #[grammar = "json.pest"]
// struct JSONParser;
//

#[derive(Parser)]
#[grammar = "xml.pest"]
struct XMLParser;

// fn parse_value(pair: Pair<Rule>) -> &str {

    // match pair.as_rule() {
    //     Rule::Char => pair.get_input(),
    //     _ => pair.get_input(),
    // }
// }

// enum JSONValue<'a> {
//     Object(Vec<(&'a str, JSONValue<'a>)>),
//     Array(Vec<JSONValue<'a>>),
//     String(&'a str),
//     Number(f64),
//     Boolean(bool),
//     Null,
// }
//
// fn serialize_jsonvalue(val: &JSONValue) -> String {
//     use JSONValue::*;
//
//     match val {
//         Object(o) => {
//             let contents: Vec<_> = o
//                 .iter()
//                 .map(|(name, value)|
//                      format!("\"{}\":{}", name, serialize_jsonvalue(value)))
//                 .collect();
//             format!("{{{}}}", contents.join(","))
//         }
//         Array(a) => {
//             let contents: Vec<_> = a.iter().map(serialize_jsonvalue).collect();
//             format!("[{}]", contents.join(","))
//         }
//         String(s) => format!("\"{}\"", s),
//         Number(n) => format!("{}", n),
//         Boolean(b) => format!("{}", b),
//         Null => format!("null"),
//     }
// }
//
// fn parse_json_file(file: &str) -> Result<JSONValue, Error<Rule>> {
//     let json = JSONParser::parse(Rule::json, file)?.next().unwrap();
//
//
//     fn parse_value(pair: Pair<Rule>) -> JSONValue {
//         match pair.as_rule() {
//             Rule::object => JSONValue::Object(
//                 pair.into_inner()
//                     .map(|pair| {
//                         let mut inner_rules = pair.into_inner();
//                         let name = inner_rules
//                             .next()
//                             .unwrap()
//                             .into_inner()
//                             .next()
//                             .unwrap()
//                             .as_str();
//                         let value = parse_value(inner_rules.next().unwrap());
//                         (name, value)
//                     })
//                     .collect(),
//             ),
//             Rule::array => JSONValue::Array(pair.into_inner().map(parse_value).collect()),
//             Rule::string => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
//             Rule::number => JSONValue::Number(pair.as_str().parse().unwrap()),
//             Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
//             Rule::null => JSONValue::Null,
//             Rule::json
//             | Rule::EOI
//             | Rule::pair
//             | Rule::value
//             | Rule::inner
//             | Rule::char
//             | Rule::WHITESPACE => unreachable!(),
//         }
//     }
//     Ok(parse_value(json))
// }

fn main() {
    // let unparsed_file = fs::read_to_string("./src/data.json").expect("cannot read file");
    //
    // let json: JSONValue = parse_json_file(&unparsed_file).expect("unsuccessful parse");
    //
    // println!("{}", serialize_jsonvalue(&json));
}

#[cfg(test)]
mod tests {
    use pest::{parses_to, consumes_to, fails_with};

    use super::*;

    // #[test]
    // fn test_xml_char() {
    //
    //     // let c = "a";
    //     // let xml = XMLParser::parse(Rule::Char, c).unwrap().next().unwrap();
    //     // assert_eq!("a",parse_value(xml));
    //     parses_to! {
    //         parser: XMLParser, input: " ", rule: Rule::Char, tokens: [Char(0,1)]
    //     }
    // }


    mod general_tests {
        use super::*;

        #[test]
        fn test_xml_s() {
            parses_to! {
                parser: XMLParser, input: " ", rule: Rule::S, tokens: [S(0,1)]
            }
            parses_to! {
                parser: XMLParser, input: "\n", rule: Rule::S, tokens: [S(0,1)]
            }
            fails_with! {
                parser: XMLParser, input: "<![CDATA", rule: Rule::S,
                positives: vec![Rule::S], negatives: vec![], pos: 0
            };
        }

        #[test]
        fn test_xml_quote() {
            parses_to! {
                parser: XMLParser, input: r#"""#, rule: Rule::Quote, tokens: [Quote(0,1)]
            }
            parses_to! {
                parser: XMLParser, input: r#"'"#, rule: Rule::Quote, tokens: [Quote(0,1)]
            }
        }

        #[test]
        fn test_xml_comment() {
            parses_to! {
                parser: XMLParser, input: "<!-- -->", rule: Rule::Comment, tokens: [Comment(0,8)]
            }
            parses_to! {
                parser: XMLParser, input: "<!--abce&h_jkl:;-->", rule: Rule::Comment, tokens: [Comment(0,19)]
            }

            // ending with '-' :(
            fails_with! {
                parser: XMLParser, input: "<!-- a--->", rule: Rule::Comment,
                positives: vec![Rule::Comment], negatives: vec![], pos: 0
            };
        }

        #[test]
        fn test_xml_charref() {
            parses_to! {
                parser: XMLParser, input: "&#160;", rule: Rule::CharRef, tokens: [CharRef(0,6)]
            }

            parses_to! {
                parser: XMLParser, input: "&#x1f0;", rule: Rule::CharRef, tokens: [CharRef(0,7)]
            }
        }

        #[test]
        fn test_xml_name() {

            // unicode-encoded emojis are allowed
            parses_to! {
                parser: XMLParser, input: "\u{1F600}ABCDEF \u{1F600}", rule: Rule::Name, tokens: [Name(0,15)]
            }

            // eg NUL-char not allowed
            fails_with! {
                parser: XMLParser, input: "\x00", rule: Rule::Name,
                positives: vec![Rule::Name], negatives: vec![], pos: 0
            };
        }

        #[test]
        fn test_xml_attribute_value() {

            // unicode-encoded emojis are allowed
            parses_to! {
                parser: XMLParser, input: r#""my-custom-attr""#, rule: Rule::AttValue, tokens: [AttValue(0,16)]
            }

            // `&` disallowed: Parser waits for `Entity`-Name to follow `&`-char
            fails_with! {
                parser: XMLParser, input: r#""some-attr&""#, rule: Rule::AttValue,
                positives: vec![Rule::Name], negatives: vec![], pos: 11
            };
        }
    }

    mod prolog_tests {
        use super::*;

        #[test]
        fn test_encoding_decl() {

            parses_to! {
                parser: XMLParser, input: r#" encoding='UTF-8'"# , rule: Rule::EncodingDecl,
                tokens: [EncodingDecl(0,17, [S(0,1),Eq(9,10),Quote(10,11),EncName(11,16)])]
            }
            parses_to! {
                parser: XMLParser, input: r#" encoding='my-custom_NAME'"#, rule: Rule::EncodingDecl,
                tokens: [EncodingDecl(0,26, [S(0,1),Eq(9,10),Quote(10,11),EncName(11,25)])]
            }

            // no space (`S`) at start
            fails_with! {
                parser: XMLParser, input: r#"encoding='my-custom_NAME'"#, rule: Rule::EncodingDecl,
                positives: vec![Rule::S], negatives: vec![], pos: 0
            };
            // illegal char in `EncName`
            fails_with! {
                parser: XMLParser, input: r#" encoding='^my-custom_NAME'"#, rule: Rule::EncodingDecl,
                positives: vec![Rule::EncName], negatives: vec![], pos: 11
            };
            // starting quote!=ending quote for `EncName`
            // see same as `test_xml_version_info`: idk: opened [discussion: pest Github](https://github.com/pest-parser/pest/discussions/957)
            fails_with! {
                parser: XMLParser, input: r#" encoding='my-custom_NAME""#, rule: Rule::EncodingDecl,
                positives: vec![Rule::EncodingDecl], negatives: vec![], pos: 25
            };
        }

        #[test]
        fn test_encoding_name() {
            parses_to! {
                parser: XMLParser, input: "my-custom_NAME", rule: Rule::EncName, tokens: [EncName(0,14)]
            }

            fails_with! {
                parser: XMLParser, input: "<![CDATA", rule: Rule::EncName,
                positives: vec![Rule::EncName], negatives: vec![], pos: 0
            };
        }

        #[test]
        fn test_standalone_decl() {
            parses_to! {
                parser: XMLParser, input: r#" standalone="yes""#, rule: Rule::StandaloneDecl, tokens: [StandaloneDecl(0,17, [
                    S(0,1), Eq(11,12), Quote(12,13)
                ])]
            }


            parses_to! {
                parser: XMLParser, input: r#" standalone='no'"#, rule: Rule::StandaloneDecl, tokens: [StandaloneDecl(0,16, [
                    S(0,1), Eq(11,12), Quote(12,13)
                ])]
            }

            // missing quotes
            fails_with! {
                parser: XMLParser, input: " standlone=yes", rule: Rule::StandaloneDecl,
                positives: vec![Rule::StandaloneDecl], negatives: vec![], pos: 0
            };
        }

        #[test]
        fn test_xml_decl() {
            parses_to! {
                parser: XMLParser, input: r#"<?xml version="1.0"?>"#, rule: Rule::XMLDecl, tokens: [XMLDecl(0,21, [
                    VersionInfo(5,19, [S(5,6), Eq(13,14),Quote(14,15),VersionNum(15,18)])
                ])]
            }
        }
    }

    #[test]
    fn test_xml_eq() {
        parses_to! {
            parser: XMLParser, input: "=", rule: Rule::Eq, tokens: [Eq(0,1)]
        }
        parses_to! {
            parser: XMLParser, input: "    = \t", rule: Rule::Eq, tokens: [Eq(0,7, [S(0,4), S(5,7)])]
        }
        fails_with! {
            parser: XMLParser, input: "<![CDATA", rule: Rule::Eq,
            positives: vec![Rule::S], negatives: vec![], pos: 0
        };
        fails_with! {
            parser: XMLParser, input: " <![CDATA", rule: Rule::Eq,
            positives: vec![Rule::Eq], negatives: vec![], pos: 0
        };

    }


    #[test]
    fn test_xml_version_num() {
        parses_to! {
            parser: XMLParser, input: "1.1", rule: Rule::VersionNum, tokens: [VersionNum(0,3)]
        }
        fails_with! {
            parser: XMLParser, input: "2.1", rule: Rule::VersionNum,
            positives: vec![Rule::VersionNum], negatives: vec![], pos: 0
        };

        fails_with! {
            parser: XMLParser, input: "1.a", rule: Rule::VersionNum,
            positives: vec![Rule::VersionNum], negatives: vec![], pos: 0
        };

    }

    #[test]
    fn test_xml_version_info() {
        parses_to! {
            parser: XMLParser, input: r#" version="1.1""#, rule: Rule::VersionInfo, tokens: [
                VersionInfo(0,14, [S(0,1),Eq(8,9),Quote(9,10),VersionNum(10,13)])]
        }
        parses_to! {
            parser: XMLParser, input: r#" version='1.1'"#, rule: Rule::VersionInfo, tokens: [
                VersionInfo(0,14, [S(0,1),Eq(8,9),Quote(9,10),VersionNum(10,13)])]
        }

        // missing whitespace before `version`:
        fails_with! {
            parser: XMLParser, input: r#"version="1.1""#, rule: Rule::VersionInfo,
            positives: vec![Rule::S], negatives: vec![], pos: 0
        };
        // missing quotes around version-nr:
        fails_with! {
            parser: XMLParser, input: r#" version=1.1"#, rule: Rule::VersionInfo,
            positives: vec![Rule::S, Rule::Quote], negatives: vec![], pos: 9
        };
        // opening and closing quotes not matching
        // idk: opened [discussion: pest Github](https://github.com/pest-parser/pest/discussions/957)
        fails_with! {
            parser: XMLParser, input: r#" version='1.1""#, rule: Rule::VersionInfo,
            positives: vec![Rule::VersionInfo], negatives: vec![], pos: 13
        };

    }

    #[test]
    fn test_xml_cdata() {

        // CDStart
        parses_to! {
            parser: XMLParser, input: "<![CDATA[", rule: Rule::CDStart,
            tokens: [CDStart(0,9)]
        };
        fails_with! {
            parser: XMLParser, input: "<![CDATA", rule: Rule::CDStart,
            positives: vec![Rule::CDStart], negatives: vec![], pos: 0
        };

        // CDEnd
        parses_to! {
            parser: XMLParser, input: "]]>", rule: Rule::CDEnd,
            tokens: [CDEnd(0,3)]
        };
        fails_with! {
            parser: XMLParser, input: "]>", rule: Rule::CDEnd,
            positives: vec![Rule::CDEnd], negatives: vec![], pos: 0
        };


        // CData
        parses_to! {
            parser: XMLParser, input: "<greeting>Hello world</greeting>", rule: Rule::CData,
            tokens: [CData(0,32)]
        };
        fails_with! {
            parser: XMLParser, input: "<![CDAA[<greeting>Hello, world!</greeting>]]>", rule: Rule::CData,
            positives: vec![Rule::CData], negatives: vec![], pos: 42
        };
        let cddata_example = "<greeting>Hello world</greeting>";
        assert!(XMLParser::parse(Rule::CData, cddata_example).is_ok());
        let cddata_example = "<![CDAA[<greeting>Hello, world!</greeting>]]>";
        println!("{:?}", XMLParser::parse(Rule::CData, cddata_example));
        assert!(XMLParser::parse(Rule::CData, cddata_example).is_err());

        let cdsect_example = "<![CDATA[<greeting>Hello, world!</greeting>]]>";
        assert!(XMLParser::parse(Rule::CDSect, cdsect_example).is_ok());
        let cdsect_example = "<![CDAA[<greeting>Hello, world!</greeting>]]>";
        assert!(XMLParser::parse(Rule::CDSect, cdsect_example).is_err());
    }
}
