use crate::lib::RustExpressionEngine::parser;
use serde_json::json;
use serde_json::Value;
use crate::lib::RustExpressionEngine::runtime::OptMap;
use crate::lib::RustExpressionEngine::node::{Node,NodeType};
use crate::utils;
use chrono::Local;
use crate::utils::time_util;
use std::thread::Thread;
use test::Bencher;
use crate::lib::RustExpressionEngine::node::NodeType::{NNumber, NOpt};
use std::rc::Rc;
use std::sync::Arc;


#[test]
fn TestParser() {
    let (mut boxNode,_ )= parser::Parser(String::from("'1' + '2'"), &OptMap::new());
    let john = json!({
        "name": "John Doe",
        "age": Value::Null,
         "sex":{
            "a":"i'm a",
            "b":"i'm b",
         },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    println!("result >>>>>>>>>>   =  {}", boxNode.eval(&john));

}

#[test]
fn TestBenchmark() {
    let (mut boxNode,_ )= parser::Parser(String::from("1<=2"), &OptMap::new());
    let john = json!({
        "name": "John Doe",
    });
    let total=100000;
    let now=Local::now();
    for _ in 0..total{
        for _ in 0..1{
            boxNode.clone();
            // boxNode.clone();
            // n2.eval(&john);
        }
    }
    utils::time_util::count_time(total,now);
    utils::time_util::count_tps(total,now);
}



#[bench]
fn Bench_Parser(b: &mut Bencher) {
    let (mut boxNode,_ )= parser::Parser(String::from("'1'+'2'"), &OptMap::new());
    let john = json!({
        "name": "John Doe",
    });
    let now=Local::now();
    b.iter(|| {
        boxNode.eval(&john);
    });
}