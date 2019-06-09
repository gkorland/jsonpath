#![feature(test)]
extern crate bencher;
extern crate jsonpath_lib as jsonpath;
extern crate serde;
extern crate serde_json;
extern crate test;

use std::io::Read;

use serde::Deserialize;
use serde_json::Value;

use jsonpath::{SelectorMut, Selector};

use self::test::Bencher;

fn read_json(path: &str) -> String {
    let mut f = std::fs::File::open(path).unwrap();
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    contents
}

fn get_string() -> String {
    read_json("./benches/example.json")
}

fn get_json() -> Value {
    let string = get_string();
    serde_json::from_str(string.as_str()).unwrap()
}

fn get_path() -> &'static str {
    r#"$..book[?(@.price<30 && @.category=="fiction")]"#
}

#[bench]
fn bench_selector(b: &mut Bencher) {
    let json = get_json();
    let mut selector = jsonpath::selector(&json);
    b.iter(move || {
        for _ in 1..100 {
            let _ = selector(get_path()).unwrap();
        }
    });
}

#[bench]
fn bench_selector_as(b: &mut Bencher) {
    let json = get_json();
    let mut selector = jsonpath::selector_as::<Value>(&json);
    b.iter(move || {
        for _ in 1..100 {
            let _ = selector(get_path()).unwrap();
        }
    });
}

#[bench]
fn bench_select_val(b: &mut Bencher) {
    let json = get_json();
    b.iter(move || {
        for _ in 1..100 {
            let _ = jsonpath::select(&json, get_path()).unwrap();
        }
    });
}

#[bench]
fn bench_select_as_str(b: &mut Bencher) {
    let json = get_string();
    b.iter(move || {
        for _ in 1..100 {
            let _ = jsonpath::select_as_str(&json, get_path()).unwrap();
        }
    });
}

#[bench]
fn bench_compile(b: &mut Bencher) {
    let json = get_json();
    let mut template = jsonpath::compile(get_path());
    b.iter(move || {
        for _ in 1..100 {
            let _ = template(&json).unwrap();
        }
    });
}

#[bench]
fn bench_select_as(b: &mut Bencher) {
    let json = get_string();

    #[derive(Deserialize, PartialEq, Debug)]
    struct Book {
        category: String,
        author: String,
        title: String,
        price: f64,
    }

    b.iter(move || {
        for _ in 1..100 {
            let _: Vec<Book> = jsonpath::select_as(&json, r#"$..book[?(@.price<30 && @.category=="fiction")][0]"#).unwrap();
        }
    });
}

#[bench]
fn bench_delete(b: &mut Bencher) {
    let json = get_json();
    let mut selector = SelectorMut::new();
    let _ = selector.str_path(get_path());

    b.iter(move || {
        for _ in 1..100 {
            let _ = selector.value(json.clone()).delete();
        }
    });
}

#[bench]
fn bench_select_to_compare_with_delete(b: &mut Bencher) {
    let json = &get_json();

    let mut selector = Selector::new();
    let _ = selector.str_path(get_path());
    let _ = selector.value(json);

    b.iter(move || {
        for _ in 1..100 {
            let _ = json.clone();
            let _ = selector.reset_value().select();
        }
    });
}