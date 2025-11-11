use std::any;

use anyhow::*;
use json_parser::*;
use pest::Parser;

#[test]
fn test_char() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::char, "0f")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!("0", res1.as_str());
    println!("{res1:?}");
    Ok(())
}

#[test]
fn test_char_is_backslash() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::char, r"\");

    assert!(res1.is_err());

    Ok(())
}

#[test]
fn test_char_is_quote() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::char, "\"");

    println!("{res1:?}");
    assert!(res1.is_err());
    //assert_ne!("\"", res1.as_str());

    Ok(())
}

#[test]
fn test_char_is_escaped() -> anyhow::Result<()> {
    let escaped_chars = vec![r"\/", r"\b", r"\r", r"\n", r"\f", r"\t", r"\u1111"];

    for char in escaped_chars {
        let res = JSONParser::parse(Rule::char, char)?
            .next()
            .ok_or_else(|| anyhow!("no pair"))?;

        println!("{}", res.as_str());
        assert_eq!(char, res.as_str());
    }

    Ok(())
}

#[test]
fn test_inner() -> anyhow::Result<()> {
    let res = JSONParser::parse(Rule::inner, "abc")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;

    println!("{}", res);
    assert_eq!("abc", res.as_str());
    Ok(())
}

#[test]
fn test_string() -> anyhow::Result<()> {
    let mut res = JSONParser::parse(Rule::string, r#""abc def""#).unwrap();

    assert_eq!(r#""abc def""#, res.as_str());
    //println!("{:?}", res.next().unwrap().into_inner().next().unwrap().into_inner());

    Ok(())
}
