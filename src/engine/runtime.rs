use std::collections::HashMap;
use std::collections::linked_list::LinkedList;
use std::sync::RwLock;

use serde_json::Value;

use crate::engine::node::Node;
use crate::engine::parser::parse;

lazy_static! {
   /// for engine: if cache not have expr value,it will be redo parser code.not wait cache return for no blocking
   /// global expr cache,use RwLock but not blocking
   static ref  EXPR_CACHE: RwLock<HashMap<String, Node>> = RwLock::new(HashMap::new());
}

/// the express engine for  exe code on runtime
#[derive(Clone, Debug)]
pub struct RbatisEngine {
    pub opt_map: OptMap<'static>,
}

impl RbatisEngine {
    pub fn new() -> Self {
        return Self {
            opt_map: OptMap::new(),
        };
    }

    ///eval express with arg value,if cache have value it will no run parser expr.
    pub fn eval(&self, expr: &str, arg: &Value) -> Result<Value, crate::core::Error> {
        let cached = self.cache_read(expr);
        if cached.is_none() {
            let nodes = parse(expr, &self.opt_map);
            if nodes.is_err() {
                return Result::Err(nodes.err().unwrap());
            }
            let node = nodes.unwrap();
            self.cache_insert(expr.to_string(), node.clone());
            return node.eval(arg);
        } else {
            let nodes = cached.unwrap();
            return nodes.eval(arg);
        }
    }

    /// read from cache,if not exist return null
    fn cache_read(&self, arg: &str) -> Option<Node> {
        let cache_read = EXPR_CACHE.try_read();
        if cache_read.is_err() {
            return Option::None;
        }
        let cache_read = cache_read.unwrap();
        let r = cache_read.get(arg);
        return if r.is_none() {
            Option::None
        } else {
            r.cloned()
        };
    }

    /// save to cache,if fail nothing to do.
    fn cache_insert(&self, key: String, node: Node) -> Result<(), crate::core::Error> {
        let cache_write = EXPR_CACHE.try_write();
        if cache_write.is_err() {
            return Err(crate::core::Error::from(cache_write.err().unwrap().to_string()));
        }
        let mut cache_write = cache_write.unwrap();
        cache_write.insert(key, node);
        return Ok(());
    }

    /// no cache mode to run engine
    pub fn eval_no_cache(&self, lexer_arg: &str, arg: &Value) -> Result<Value, crate::core::Error> {
        let nodes = parse(lexer_arg, &self.opt_map);
        if nodes.is_err() {
            return Result::Err(nodes.err().unwrap());
        }
        let node = nodes.unwrap();
        return node.eval(arg);
    }
}


pub fn is_number(arg: &String) -> bool {
    let chars = arg.chars();
    for item in chars {
        if item == '-' ||
            item == '.' ||
            item == '0' ||
            item == '1' ||
            item == '2' ||
            item == '3' ||
            item == '4' ||
            item == '5' ||
            item == '6' ||
            item == '7' ||
            item == '8' ||
            item == '9'
        {
            // nothing do
        } else {
            return false;
        }
    }
    return true;
}


#[derive(Clone, Debug)]
pub struct OptMap<'a> {
    //全部操作符
    pub map: HashMap<&'a str, bool>,
    //复合操作符
    pub mul_ops_map: HashMap<&'a str, bool>,
    //单操作符
    pub single_opt_map: HashMap<&'a str, bool>,

    pub allow_sorted: Vec<&'a str>,
}

impl<'a> OptMap<'a> {
    pub fn new() -> Self {
        let mut all = HashMap::new();
        let mut mul_ops_map = HashMap::new();
        let mut single_opt_map = HashMap::new();

        //all opt
        let list = vec![
            "(", ")",
            "%", "^", "*","**", "/", "+", "-",
            "@", "#", "$", "=", "!", ">", "<", "&", "|",
            "==", "!=", ">=", "<=", "&&", "||"
        ];

        //all opt map
        for item in &list {
            all.insert(item.to_owned(), true);
        }
        //single opt and mul opt
        for item in &list {
            if item.len() > 1 {
                mul_ops_map.insert(item.to_owned(), true);
            } else {
                single_opt_map.insert(item.to_owned(), true);
            }
        }

        Self {
            map: all,
            mul_ops_map,
            single_opt_map,
            allow_sorted: vec!["%", "^", "*","**", "/", "+", "-", "<=", "<", ">=", ">", "!=", "==", "&&", "||"],
        }
    }

    ///The or operation in the nonoperational > arithmetic operator > relational operator > logical operator and operation > logical operator
    pub fn priority_array(&self) -> &Vec<&str> {
        return &self.allow_sorted;
    }

    pub fn is_opt(&self, arg: &str) -> bool {
        let opt = self.map.get(arg);
        return opt.is_none() == false;
    }

    pub fn is_allow_opt(&self, arg: &str) -> bool {
        for item in &self.allow_sorted {
            if arg == *item {
                return true;
            }
        }
        return false;
    }
}