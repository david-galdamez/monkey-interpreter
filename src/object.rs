use std::any::Any;

type ObjectType = &'static str;

pub trait Object {
    fn object_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
    fn as_any(&self) -> &dyn Any;
}

const INTEGER_OBJ: &str = "INTEGER";
const BOOLEAN_OBJ: &str = "BOOLEAN";
const NULL_OBJ: &str = "NULL";

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
}

struct Boolean {
    value: bool,
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
}

struct Null;

impl Object for Null {
    fn inspect(&self) -> String {
        format!("null")
    }

    fn object_type(&self) -> ObjectType {
        NULL_OBJ
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
