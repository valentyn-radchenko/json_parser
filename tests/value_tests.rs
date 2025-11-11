use anyhow::*;
use json_parser::*;
use pest::Parser;

#[test]
fn test_boolean() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::boolean, "true")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!("true", res1.as_str());

    let res2 = JSONParser::parse(Rule::boolean, "false")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!("false", res2.as_str());

    Ok(())
}

#[test]
fn test_null() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::null, "null")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!("null", res1.as_str());

    Ok(())
}

#[test]
fn test_pair() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::pair, "\"age\": 42")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!("\"age\": 42", res1.as_str());

    Ok(())
}

#[test]
fn test_object() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::object, "{\"age\": 42}")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!("{\"age\": 42}", res1.as_str());

    Ok(())
}

#[test]
fn test_array() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::array, "[1,2,3]")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!("[1,2,3]", res1.as_str());

    Ok(())
}
#[test]
fn test_file() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::file, "[1,2,3]")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!("[1,2,3]", res1.as_str());
    let res2 = JSONParser::parse(Rule::file, "{\"age\": 42}")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!("{\"age\": 42}", res2.as_str());

    Ok(())
}
