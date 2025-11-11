use anyhow::*;
use json_parser::*;
use pest::Parser;

#[test]
fn test_int() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::int, "12321");
    assert!(res1.is_ok());

    let tokens = res1.unwrap().next().unwrap();
    println!("{tokens:?}");

    let res2 = JSONParser::parse(Rule::int, "0")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(res2.as_str(), "0");

    let res3 = JSONParser::parse(Rule::int, "023")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(res3.as_str(), "0");

    Ok(())
}

#[test]
fn test_frac() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::frac, ".00")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(res1.as_str(), ".00");

    Ok(())
}

#[test]
fn test_exp() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::exp, "e2")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(res1.as_str(), "e2");

    let res2 = JSONParser::parse(Rule::exp, "E2")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(res2.as_str(), "E2");

    Ok(())
}
#[test]
fn test_number() -> anyhow::Result<()> {
    let res1 = JSONParser::parse(Rule::number, "123.1e34")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(res1.as_str(), "123.1e34");
    println!("{:?}", res1);

    let res2 = JSONParser::parse(Rule::number, "123.1")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(res2.as_str(), "123.1");

    let res3 = JSONParser::parse(Rule::number, "0.1")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(res3.as_str(), "0.1");

    let res4 = JSONParser::parse(Rule::number, "0e3.4")?
        .next()
        .ok_or_else(|| anyhow!("no pair"))?;
    assert_eq!(res4.as_str(), "0e3");

    Ok(())
}
