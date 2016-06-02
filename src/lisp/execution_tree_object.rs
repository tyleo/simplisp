use error::LispResult as R;
use lisp::ExecutionTreeNode;

#[derive(Clone)]
pub enum ExecutionTreeObject {
    Bool(bool),
    Char(char),
    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    ISize(isize),
    Node(ExecutionTreeNode),
    Symbol(String),
    String(String),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    USize(usize),
}

impl ExecutionTreeObject {
    pub fn to_string(&self) -> R<String> {
        let result =
            match self {
                &ExecutionTreeObject::Bool(ref some) => some.to_string(),
                &ExecutionTreeObject::Char(ref some) => format!("'{}'", some.to_string()),
                &ExecutionTreeObject::F32(ref some) => format!("{}f32", some.to_string()),
                &ExecutionTreeObject::F64(ref some) => format!("{}f64", some.to_string()),
                &ExecutionTreeObject::I8(ref some) => format!("{}i8", some.to_string()),
                &ExecutionTreeObject::I16(ref some) => format!("{}i16", some.to_string()),
                &ExecutionTreeObject::I32(ref some) => format!("{}i32", some.to_string()),
                &ExecutionTreeObject::I64(ref some) => format!("{}i64", some.to_string()),
                &ExecutionTreeObject::ISize(ref some) => format!("{}isize", some.to_string()),
                &ExecutionTreeObject::Node(ref some) => try!(some.to_string()),
                &ExecutionTreeObject::Symbol(ref some) => some.to_string(),
                &ExecutionTreeObject::String(ref some) => format!("\"{}\"", some),
                &ExecutionTreeObject::U8(ref some) => format!("{}u8", some.to_string()),
                &ExecutionTreeObject::U16(ref some) => format!("{}u16", some.to_string()),
                &ExecutionTreeObject::U32(ref some) => format!("{}u32", some.to_string()),
                &ExecutionTreeObject::U64(ref some) => format!("{}u64", some.to_string()),
                &ExecutionTreeObject::USize(ref some) => format!("{}usize", some.to_string()),
            };
        Ok(result)
    }
}

impl From<ExecutionTreeObject> for R<bool> {
    fn from(object: ExecutionTreeObject) -> R<bool> {
        match object {
            ExecutionTreeObject::Bool(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<char> {
    fn from(object: ExecutionTreeObject) -> R<char> {
        match object {
            ExecutionTreeObject::Char(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<f32> {
    fn from(object: ExecutionTreeObject) -> R<f32> {
        match object {
            ExecutionTreeObject::F32(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<f64> {
    fn from(object: ExecutionTreeObject) -> R<f64> {
        match object {
            ExecutionTreeObject::F64(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<i8> {
    fn from(object: ExecutionTreeObject) -> R<i8> {
        match object {
            ExecutionTreeObject::I8(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<i16> {
    fn from(object: ExecutionTreeObject) -> R<i16> {
        match object {
            ExecutionTreeObject::I16(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<i32> {
    fn from(object: ExecutionTreeObject) -> R<i32> {
        match object {
            ExecutionTreeObject::I32(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<i64> {
    fn from(object: ExecutionTreeObject) -> R<i64> {
        match object {
            ExecutionTreeObject::I64(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<isize> {
    fn from(object: ExecutionTreeObject) -> R<isize> {
        match object {
            ExecutionTreeObject::ISize(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<ExecutionTreeNode> {
    fn from(object: ExecutionTreeObject) -> R<ExecutionTreeNode> {
        match object {
            ExecutionTreeObject::Node(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<String> {
    fn from(object: ExecutionTreeObject) -> R<String> {
        match object {
            ExecutionTreeObject::String(result) => Ok(result),
            ExecutionTreeObject::Symbol(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<u8> {
    fn from(object: ExecutionTreeObject) -> R<u8> {
        match object {
            ExecutionTreeObject::U8(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<u16> {
    fn from(object: ExecutionTreeObject) -> R<u16> {
        match object {
            ExecutionTreeObject::U16(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<u32> {
    fn from(object: ExecutionTreeObject) -> R<u32> {
        match object {
            ExecutionTreeObject::U32(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<u64> {
    fn from(object: ExecutionTreeObject) -> R<u64> {
        match object {
            ExecutionTreeObject::U64(result) => Ok(result),
            _ => panic!(),
        }
    }
}

impl From<ExecutionTreeObject> for R<usize> {
    fn from(object: ExecutionTreeObject) -> R<usize> {
        match object {
            ExecutionTreeObject::USize(result) => Ok(result),
            _ => panic!(),
        }
    }
}
