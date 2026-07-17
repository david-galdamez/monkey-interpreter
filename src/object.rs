use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
    rc::Rc,
};

use crate::ast;

type ObjectType = &'static str;

pub trait Object {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn clone_box(&self) -> Box<dyn Object>;
}

type BuiltinFunction = fn(&[Box<dyn Object>]) -> Box<dyn Object>;

pub const INTEGER_OBJ: &str = "INTEGER";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";
pub const NULL_OBJ: &str = "NULL";
pub const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
pub const ERROR_OBJ: &str = "ERROR";
pub const FUNCTION_OBJ: &str = "FUNCTION";
pub const STRING_OBJ: &str = "STRING";
pub const BUILTIN_OBJ: &str = "BUILTIN";
pub const ARRAY_OBJ: &str = "ARRAY";
pub const HASH_OBJ: &str = "HASH";

#[derive(Debug, Clone, Copy)]
pub struct Integer {
    pub value: i64,
}

impl Object for Integer {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn object_type(&self) -> ObjectType {
        INTEGER_OBJ
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Boolean {
    pub value: bool,
}

impl Object for Boolean {
    fn inspect(&self) -> String {
        format!("{}", self.value)
    }

    fn object_type(&self) -> ObjectType {
        BOOLEAN_OBJ
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(*self)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Null;

impl Object for Null {
    fn inspect(&self) -> String {
        "null".to_string()
    }

    fn object_type(&self) -> ObjectType {
        NULL_OBJ
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(*self)
    }
}

pub struct ReturnValue {
    pub value: Box<dyn Object>,
}

impl Clone for ReturnValue {
    fn clone(&self) -> Self {
        ReturnValue {
            value: self.value.clone_box(),
        }
    }
}

impl Object for ReturnValue {
    fn inspect(&self) -> String {
        self.value.inspect().to_string()
    }

    fn object_type(&self) -> ObjectType {
        RETURN_VALUE_OBJ
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl Object for Error {
    fn inspect(&self) -> String {
        format!("ERROR: {}", self.message)
    }

    fn object_type(&self) -> ObjectType {
        ERROR_OBJ
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone, Hash)]
pub struct StringObject {
    pub value: String,
}

impl Object for StringObject {
    fn inspect(&self) -> String {
        self.value.clone()
    }

    fn object_type(&self) -> ObjectType {
        STRING_OBJ
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct Builtin {
    pub func: BuiltinFunction,
}

impl Object for Builtin {
    fn inspect(&self) -> String {
        "builtin function".to_string()
    }

    fn object_type(&self) -> ObjectType {
        BUILTIN_OBJ
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

pub struct Array {
    pub elements: Vec<Box<dyn Object>>,
}

impl Clone for Array {
    fn clone(&self) -> Self {
        Array {
            elements: self.elements.iter().map(|e| e.clone_box()).collect(),
        }
    }
}

impl Object for Array {
    fn inspect(&self) -> String {
        let elements: Vec<String> = self.elements.iter().map(|p| p.inspect()).collect();

        let mut buffer = String::new();
        buffer.push('[');
        buffer.push_str(&elements.join(", "));
        buffer.push(']');

        buffer
    }

    fn object_type(&self) -> ObjectType {
        ARRAY_OBJ
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

pub struct Function {
    pub parameters: Vec<ast::Identifier>,
    pub body: ast::BlockStatement,
    pub env: Rc<RefCell<Environment>>,
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Function {
            parameters: self.parameters.clone(),
            body: self.body.clone(),
            env: Rc::clone(&self.env),
        }
    }
}

impl Object for Function {
    fn inspect(&self) -> String {
        let params: Vec<String> = self.parameters.iter().map(|p| format!("{}", p)).collect();
        let mut buf = String::new();
        buf.push_str("fn(");
        buf.push_str(&params.join(", "));
        buf.push_str(") {");
        buf.push_str(&format!("{}", self.body));
        buf.push_str("\n}");

        buf
    }

    fn object_type(&self) -> ObjectType {
        FUNCTION_OBJ
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

pub struct Environment {
    store: HashMap<String, Box<dyn Object>>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Environment {
            store: HashMap::new(),
            outer: Some(outer),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Box<dyn Object>> {
        match self.store.get(name) {
            Some(val) => Some(val.clone_box()),
            None => self
                .outer
                .as_ref()
                .and_then(|outer| outer.borrow().get(name)),
        }
    }

    pub fn set(&mut self, name: String, val: Box<dyn Object>) {
        self.store.insert(name, val);
    }
}

pub trait Hashable {
    fn hash_key(&self) -> HashKey;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HashKey {
    object_type: ObjectType,
    value: u64,
}

impl Hashable for Boolean {
    fn hash_key(&self) -> HashKey {
        let value = if self.value { 1 } else { 0 };

        HashKey {
            object_type: self.object_type(),
            value,
        }
    }
}

impl Hashable for Integer {
    fn hash_key(&self) -> HashKey {
        HashKey {
            object_type: self.object_type(),
            value: u64::try_from(self.value).unwrap_or_default(),
        }
    }
}

impl Hashable for StringObject {
    fn hash_key(&self) -> HashKey {
        let mut hasher = DefaultHasher::new();
        self.value.hash(&mut hasher);
        HashKey {
            object_type: self.object_type(),
            value: hasher.finish(),
        }
    }
}

pub struct HashPair {
    pub key: Box<dyn Object>,
    pub value: Box<dyn Object>,
}

pub struct HashObject {
    pub pairs: HashMap<HashKey, HashPair>,
}

impl Clone for HashPair {
    fn clone(&self) -> Self {
        HashPair {
            key: self.key.clone_box(),
            value: self.value.clone_box(),
        }
    }
}

impl Clone for HashObject {
    fn clone(&self) -> Self {
        HashObject {
            pairs: self.pairs.iter().map(|(k, v)| (*k, v.clone())).collect(),
        }
    }
}

impl Object for HashObject {
    fn inspect(&self) -> String {
        let mut pairs = Vec::new();

        for pair in self.pairs.values() {
            pairs.push(format!("{}: {}", pair.key.inspect(), pair.value.inspect()));
        }

        let mut buf = String::new();
        buf.push('{');
        buf.push_str(&pairs.join(", ").to_string());
        buf.push('}');

        buf
    }

    fn object_type(&self) -> ObjectType {
        HASH_OBJ
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn clone_box(&self) -> Box<dyn Object> {
        Box::new(self.clone())
    }
}

mod tests {
    use crate::object::{Hashable, StringObject};

    #[test]
    fn test_string_hash_key() {
        let hello1 = StringObject {
            value: String::from("Hello World"),
        };
        let hello2 = StringObject {
            value: String::from("Hello World"),
        };
        let diff2 = StringObject {
            value: String::from("My name is johnny"),
        };
        let diff1 = StringObject {
            value: String::from("My name is johnny"),
        };

        assert_eq!(
            hello1.hash_key(),
            hello2.hash_key(),
            "strings with same content have different hash keys"
        );

        assert_eq!(
            diff1.hash_key(),
            diff2.hash_key(),
            "strings with same content have different hash keys"
        );

        assert_ne!(
            hello1.hash_key(),
            diff1.hash_key(),
            "strings with same content have different hash keys"
        );
    }
}
