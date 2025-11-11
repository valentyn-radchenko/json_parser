use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use thiserror::Error;



/// JSON parser implementing a set of the standard JSON grammar.
///
/// Each rule below corresponds to a rule in the `json.pest` file.
#[derive(Parser)]
#[grammar = "./json.pest"]
pub struct JSONParser {}

#[derive(Debug, Error)]
pub enum JSONError {
    #[error("Failed to read file {0}")]
    IO(#[from] std::io::Error),

    #[error("Failed to parse float {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),

    #[error("Pest parse error: {0}")]
    Pest(#[from] Box<pest::error::Error<Rule>>),

    #[error("Unexpected parser structure")]
    Unexpected,
}

impl From<pest::error::Error<Rule>> for JSONError {
    fn from(err: pest::error::Error<Rule>) -> Self{
        JSONError::Pest(Box::new(err))
    }
}

#[derive(Debug)]
pub enum JSONValue<'a> {
    Object(Vec<(&'a str, JSONValue<'a>)>),
    Array(Vec<JSONValue<'a>>),
    String(&'a str),
    Number(f64),
    Boolean(bool),
    Null,
}

pub fn parse_json_file(input: &str) -> Result<JSONValue<'_>, JSONError> {
    let json = JSONParser::parse(Rule::file, input)?
        .next()
        .ok_or(JSONError::Unexpected)?;

    fn parse_value(pair: Pair<Rule>) -> JSONValue {
        match pair.as_rule() {
            Rule::object => JSONValue::Object(
                pair.into_inner()
                    .map(|pair| {
                        let mut pair_rules = pair.into_inner();
                        let pair_name = pair_rules
                            .next()
                            .unwrap()
                            .into_inner()
                            .next()
                            .unwrap()
                            .as_str();
                        let pair_value = parse_value(pair_rules.next().unwrap());

                        (pair_name, pair_value)
                    })
                    .collect(),
            ),
            Rule::array => {
                JSONValue::Array(pair.into_inner().map(|value| parse_value(value)).collect())
            }
            Rule::string => JSONValue::String(pair.into_inner().next().unwrap().as_str()),
            Rule::number => JSONValue::Number(pair.as_str().parse::<f64>().unwrap()),
            Rule::boolean => JSONValue::Boolean(pair.as_str().parse().unwrap()),
            Rule::null => JSONValue::Null,
            Rule::file
            | Rule::char
            | Rule::exp
            | Rule::frac
            | Rule::inner
            | Rule::int
            | Rule::EOI
            | Rule::value
            | Rule::pair
            | Rule::WHITESPACE => unreachable!(),
        }
    }

    Ok(parse_value(json))
}

pub fn serialize_jsonvalue(val: &JSONValue) -> String {
    use JSONValue::*;

    match val {
        Object(o) => {
            let contents: Vec<_> = o
                .iter()
                .map(|(name, value)| format!("\"{}\":{}", name, serialize_jsonvalue(value)))
                .collect();
            format!("{{{}}}", contents.join(","))
        }
        Array(a) => {
            let contents: Vec<_> = a.iter().map(serialize_jsonvalue).collect();
            format!("[{}]", contents.join(","))
        }
        String(s) => format!("\"{}\"", s),
        Number(n) => format!("{}", n),
        Boolean(b) => format!("{}", b),
        Null => "null".to_string(),
    }
}


/// The top-level rule representing a complete JSON document.
///
/// Parses either a JSON object or array, ensuring start- and end-of-input markers.
///
/// **Grammar**
/// ```text
/// file = _{ SOI ~ (object | array) ~ EOI }
/// ```
pub const RULE_FILE: Rule = Rule::file;

/// Represents a JSON object — a collection of key/value pairs within `{}` braces.
///
/// Objects may be empty (`{}`) or contain one or more pairs separated by commas.
///
/// **Grammar**
/// ```text
/// object = { "{" ~ "}" | "{" ~ pair ~ ("," ~ pair)* ~ "}" }
/// ```
///
/// **Example**
/// ```json
/// { "name": "Alice", "age": 30 }
/// ```
pub const RULE_OBJECT: Rule = Rule::object;

/// Represents a JSON array — a sequence of values within `[]` brackets.
///
/// Arrays may be empty (`[]`) or contain one or more values separated by commas.
///
/// **Grammar**
/// ```text
/// array = { "[" ~ "]" | "[" ~ value ~ ("," ~ value)* ~ "]" }
/// ```
///
/// **Example**
/// ```json
/// [1, "two", null, true]
/// ```
pub const RULE_ARRAY: Rule = Rule::array;

/// Represents a JSON key/value pair inside an object.
///
/// Each pair consists of a string key followed by a colon and a value.
///
/// **Grammar**
/// ```text
/// pair = { string ~ ":" ~ value }
/// ```
///
/// **Example**
/// ```json
/// "age": 42
/// ```
pub const RULE_PAIR: Rule = Rule::pair;

/// Represents a JSON value, which can be an object, array, string, number, boolean, or null.
///
/// **Grammar**
/// ```text
/// value = _{ object | array | string | number | boolean | null }
/// ```
///
/// **Example**
/// ```json
/// "hello"
/// ```
pub const RULE_VALUE: Rule = Rule::value;

/// Represents a JSON string literal enclosed in double quotes.
///
/// Supports escaped characters and Unicode code points (`\uXXXX`).
///
/// **Grammar**
/// ```text
/// string = ${ "\"" ~ inner ~ "\"" }
/// inner  = @{ char* }
/// char   = {
///     !("\"" | "\\") ~ ANY
///     | "\\" ~ ("\"" | "\\" | "/" | "b" | "f" | "n" | "r" | "t")
///     | "\\" ~ ("u" ~ ASCII_HEX_DIGIT{4})
/// }
/// ```
///
/// **Example**
/// ```json
/// "Hello\nWorld"
/// ```
pub const RULE_STRING: Rule = Rule::string;

/// Represents a JSON number, supporting integers, fractions, and exponents.
///
/// **Grammar**
/// ```text
/// int    = @{"0" | (ASCII_NONZERO_DIGIT ~ ASCII_DIGIT*)}
/// frac   = @{"." ~ ASCII_DIGIT+}
/// exp    = @{("E" | "e") ~ ("+" | "-")? ~ ASCII_DIGIT+}
/// number = @{"-"? ~ int ~ frac? ~ exp?}
/// ```
///
/// **Example**
/// ```json
/// -12.34e+5
/// ```
pub const RULE_NUMBER: Rule = Rule::number;

/// Represents a JSON boolean literal — either `true` or `false`.
///
/// **Grammar**
/// ```text
/// boolean = { "true" | "false" }
/// ```
///
/// **Example**
/// ```json
/// false
/// ```
pub const RULE_BOOLEAN: Rule = Rule::boolean;

/// Represents the JSON null literal.
///
/// **Grammar**
/// ```text
/// null = { "null" }
/// ```
///
/// **Example**
/// ```json
/// null
/// ```
pub const RULE_NULL: Rule = Rule::null;

/// Represents whitespace (spaces, tabs, carriage returns, or newlines).
///
/// Used internally by the grammar to skip insignificant characters.
///
/// **Grammar**
/// ```text
/// WHITESPACE = _{ " " | "\t" | "\r" | "\n" }
/// ```
pub const RULE_WHITESPACE: Rule = Rule::WHITESPACE;

/// Represents the integer portion of a number (no sign or decimal).
///
/// **Grammar**
/// ```text
/// int = @{"0" | (ASCII_NONZERO_DIGIT ~ ASCII_DIGIT*)}
/// ```
pub const RULE_INT: Rule = Rule::int;

/// Represents the fractional portion of a number (the part after the decimal point).
///
/// **Grammar**
/// ```text
/// frac = @{"." ~ ASCII_DIGIT+}
/// ```
pub const RULE_FRAC: Rule = Rule::frac;

/// Represents the exponential part of a number (scientific notation).
///
/// **Grammar**
/// ```text
/// exp = @{("E" | "e") ~ ("+" | "-")? ~ ASCII_DIGIT+}
/// ```
pub const RULE_EXP: Rule = Rule::exp;

/// Represents the interior characters of a string (excluding quotes and backslashes).
///
/// **Grammar**
/// ```text
/// inner = @{ char* }
/// ```
pub const RULE_INNER: Rule = Rule::inner;

/// Represents a single valid character within a string, handling escapes and Unicode codes.
///
/// **Grammar**
/// ```text
/// char = {
///     !("\"" | "\\") ~ ANY
///     | "\\" ~ ("\"" | "\\" | "/" | "b" | "f" | "n" | "r" | "t")
///     | "\\" ~ ("u" ~ ASCII_HEX_DIGIT{4})
/// }
/// ```
pub const RULE_CHAR: Rule = Rule::char;