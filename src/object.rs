use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

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
        Box::new(self.clone())
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
        Box::new(self.clone())
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
        Box::new(self.clone())
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

#[derive(Debug, Clone)]
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
        buffer.push_str("[");
        buffer.push_str(&elements.join(", "));
        buffer.push_str("]");

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
