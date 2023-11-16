use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Nil,
    Obj(Obj),
}

impl Value {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_nil(&self) -> bool {
        match self {
            Value::Nil => true,
            _ => false,
        }
    }

    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Nil | Value::Bool(false) => true,
            _ => false,
        }
    }

    pub fn is_obj_type(&self, obj_type: ObjType) -> bool {
        match self {
            Value::Obj(obj) => obj.is_type(obj_type),
            _ => false,
        }
    }

    pub fn as_string(self) -> Option<String> {
        match self {
            Value::Obj(Obj::String(s)) => Some(s),
            _ => None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Obj(Obj::String(s)) => write!(f, "{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Obj {
    String(String),
}

impl Obj {
    fn is_type(&self, obj_type: ObjType) -> bool {
        match self {
            Obj::String(_) => obj_type == ObjType::String,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ObjType {
    String,
}
