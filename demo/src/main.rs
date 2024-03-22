use untwine::{parser, ParserContext};

parser! {
    int = <{char::is_ascii_digit}+> -> &str;
    sep = #{char::is_ascii_whitespace}*;
    comma = sep? "," sep?;

    pub lit: thing=int$comma+ -> Vec<&str> { thing }
}

fn main() {
    // parser! {
    //     pub sep: #{char::is_whitespace}* -> () {}
    //     comma: sep "," sep -> () {}
    //     int: <"-"? '0'-'9'+> -> JSONValue { Int(int.parse()?) }
    //     float: <"-"? '0'-'9'+ ("." '0'-'9'+)?> -> JSONValue { Float(float.parse()?) }
    //     str_char: <("\\" . | [^"\""])> -> char { str_char.chars().last().unwrap() /* doesn't handle escape seqs properly */ }
    //     string: "\"" str_char+ "\"" -> JSONValue { String(str_char.iter().cloned().collect()) }
    //     null: "null" -> JSONValue { Null }
    //     bool: ("true" | "false") -> JSONValue { Bool(bool == "true") }
    //     list: "[" elems=json$comma* "]" -> JSONValue { List(elems) }
    //     mapEntry: string sep ":" sep json -> (JSONValue, JSONValue) { (string, json) }
    //     map: "{" entries=mapEntry$comma* "}" -> JSONValue { Map(entries.into_iter().collect()) }
    //     pub json: (int | float | bool | string | null | list | map) -> JSONValue { json }
    // }
    let input = "1, 2, 3, 4";
    let ctx = ParserContext::new(input);
    let output = lit(&ctx).unwrap();
    println!("{output:?}");
}
