// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

extern crate parse_mediawiki_dump;

use std::fs::File;
use std::io::BufReader;

const DUMP_OCT: &str = "./tests/test.xml";

#[test]
fn main() {
    let  file = File::open(DUMP_OCT).unwrap();
    let reader = BufReader::new(file);
    let mut parser =
        parse_mediawiki_dump::parse(reader);
    assert!(match parser.next() {
        Some(Ok(parse_mediawiki_dump::Page {
            format: Some(format),
            model: Some(model),
            namespace: 0,
            text,
            title,
        })) => format == "text/x-wiki" && text == "#REDIRECT [[Computer accessibility]]
{{R from move}}
{{R from CamelCase}}
{{R unprintworthy}}" && model == "wikitext" && title == "AccessibleComputing",
        _ => false,
    });
    assert!(parser.next().is_none());
}

