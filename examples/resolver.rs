// Instruction:
//
// run `cargo run --example resolver -- `pwd` test.js`
// or `cargo watch -x "run --example resolver" -- `pwd` test.js`
//
// NOTE: The first argument must be a absolute path.

use std::{env, path::PathBuf};

use oxc_resolver::{AliasValue, ResolveOptions, Resolver};

fn main() {
    let path = env::args().nth(1).expect("require path");
    let request = env::args().nth(2).expect("require request");
    let path = PathBuf::from(path).canonicalize().unwrap();

    println!("path: {path:?}");
    println!("request: {request}");

    let options = ResolveOptions {
        alias_fields: vec![vec!["browser".into()]],
        alias: vec![("/asdf".into(), vec![AliasValue::Path("./test.js".into())])],
        ..ResolveOptions::default()
    };

    match Resolver::new(options).resolve(path, &request) {
        Err(error) => println!("Error: {error}"),
        Ok(resolution) => println!("Resolved: {}", resolution.full_path().to_string_lossy()),
    }
}
