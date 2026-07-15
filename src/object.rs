use std::{any::Any, collections::HashMap};

use crate::{ast, object};

type ObjectType = &'static str;

pub trait Object {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
    fn clone_box(&self) -> Box<dyn Object>;
}

pub const INTEGER_OBJ: &str = "INTEGER";
pub const BOOLEAN_OBJ: &str = "BOOLEAN";
pub const NULL_OBJ: &str = "NULL";
pub const RETURN_VALUE_OBJ: &str = "RETURN_VALUE";
pub const ERROR_OBJ: &str = "ERROR";
pub const FUNCTION_OBJ: &str = "FUNCTION";

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

// Environment
pub struct Environment {
    store: HashMap<String, Box<dyn object::Object>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Box<dyn object::Object>> {
        self.store.get(name).map(|val| val.clone_box())
    }

    pub fn set(&mut self, name: String, val: Box<dyn object::Object>) {
        self.store.insert(name, val);
    }
}
