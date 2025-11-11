# json_parser

## Description

`json_parser` is a Rust project that implements a JSON parser using the [Pest](https://pest.rs) parsing library.  
It reads JSON text input (objects, arrays, strings, numbers, booleans, and null values) and converts it into a structured internal representation Abstract Syntax Tree (AST).

The parser can validate, analyze, and transform JSON data within Rust applications.

## Technical Description

### What Is Being Parsed

The input to this parser is a JSON-formatted string, for example:

```json
{
  "name": "Valentyn",
  "age": 69,
  "skills": ["Rust", "Java"],
  "active": true
}
```

## JSON Grammar Overview

```text
file     = { object | array }
object   = { "{" ~ pair* ~ "}" }
pair     = { string ~ ":" ~ value }
array    = { "[" ~ value* ~ "]" }
value    = { object | array | string | number | boolean | null }
string   = { "\"" ~ inner ~ "\"" }
number   = { "-"? ~ int ~ frac? ~ exp? }
boolean  = { "true" | "false" }
null     = { "null" }
```
