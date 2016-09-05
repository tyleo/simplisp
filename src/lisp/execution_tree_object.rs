use error::*;
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
    pub fn nil() -> Self {
        ExecutionTreeObject::Node(ExecutionTreeNode::nil())
    }

    pub fn bool_str() -> &'static str {
        "ExecutionTreeObject::Bool"
    }

    pub fn char_str() -> &'static str {
        "ExecutionTreeObject::Char"
    }

    pub fn f32_str() -> &'static str {
        "ExecutionTreeObject::F32"
    }

    pub fn f64_str() -> &'static str {
        "ExecutionTreeObject::F64"
    }

    pub fn i8_str() -> &'static str {
        "ExecutionTreeObject::I8"
    }

    pub fn i16_str() -> &'static str {
        "ExecutionTreeObject::I16"
    }

    pub fn i32_str() -> &'static str {
        "ExecutionTreeObject::I32"
    }

    pub fn i64_str() -> &'static str {
        "ExecutionTreeObject::I64"
    }

    pub fn isize_str() -> &'static str {
        "ExecutionTreeObject::ISize"
    }

    pub fn node_str() -> &'static str {
        "ExecutionTreeObject::Node"
    }

    pub fn symbol_str() -> &'static str {
        "ExecutionTreeObject::Symbol"
    }

    pub fn string_str() -> &'static str {
        "ExecutionTreeObject::String"
    }

    pub fn u8_str() -> &'static str {
        "ExecutionTreeObject::U8"
    }

    pub fn u16_str() -> &'static str {
        "ExecutionTreeObject::U16"
    }

    pub fn u32_str() -> &'static str {
        "ExecutionTreeObject::U32"
    }

    pub fn u64_str() -> &'static str {
        "ExecutionTreeObject::U64"
    }

    pub fn usize_str() -> &'static str {
        "ExecutionTreeObject::USize"
    }

    pub fn enum_to_string(&self) -> &'static str {
        match self {
            &ExecutionTreeObject::Bool(_) => Self::bool_str(),
            &ExecutionTreeObject::Char(_) => Self::char_str(),
            &ExecutionTreeObject::F32(_) => Self::f32_str(),
            &ExecutionTreeObject::F64(_) => Self::f64_str(),
            &ExecutionTreeObject::I8(_) => Self::i8_str(),
            &ExecutionTreeObject::I16(_) => Self::i16_str(),
            &ExecutionTreeObject::I32(_) => Self::i32_str(),
            &ExecutionTreeObject::I64(_) => Self::i64_str(),
            &ExecutionTreeObject::ISize(_) => Self::isize_str(),
            &ExecutionTreeObject::Node(_) => Self::node_str(),
            &ExecutionTreeObject::Symbol(_) => Self::symbol_str(),
            &ExecutionTreeObject::String(_) => Self::string_str(),
            &ExecutionTreeObject::U8(_) => Self::u8_str(),
            &ExecutionTreeObject::U16(_) => Self::u16_str(),
            &ExecutionTreeObject::U32(_) => Self::u32_str(),
            &ExecutionTreeObject::U64(_) => Self::u64_str(),
            &ExecutionTreeObject::USize(_) => Self::usize_str(),
        }
    }

    pub fn to_string(&self) -> Result<String> {
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

impl From<ExecutionTreeObject> for Result<bool> {
    fn from(object: ExecutionTreeObject) -> Result<bool> {
        match object {
            ExecutionTreeObject::Bool(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::bool_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<char> {
    fn from(object: ExecutionTreeObject) -> Result<char> {
        match object {
            ExecutionTreeObject::Char(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::char_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<f32> {
    fn from(object: ExecutionTreeObject) -> Result<f32> {
        match object {
            ExecutionTreeObject::F32(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::f32_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<f64> {
    fn from(object: ExecutionTreeObject) -> Result<f64> {
        match object {
            ExecutionTreeObject::F64(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::f64_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<i8> {
    fn from(object: ExecutionTreeObject) -> Result<i8> {
        match object {
            ExecutionTreeObject::I8(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::i8_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<i16> {
    fn from(object: ExecutionTreeObject) -> Result<i16> {
        match object {
            ExecutionTreeObject::I16(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::i16_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<i32> {
    fn from(object: ExecutionTreeObject) -> Result<i32> {
        match object {
            ExecutionTreeObject::I32(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::i32_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<i64> {
    fn from(object: ExecutionTreeObject) -> Result<i64> {
        match object {
            ExecutionTreeObject::I64(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::i64_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<isize> {
    fn from(object: ExecutionTreeObject) -> Result<isize> {
        match object {
            ExecutionTreeObject::ISize(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::isize_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<ExecutionTreeNode> {
    fn from(object: ExecutionTreeObject) -> Result<ExecutionTreeNode> {
        match object {
            ExecutionTreeObject::Node(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::node_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<String> {
    fn from(object: ExecutionTreeObject) -> Result<String> {
        match object {
            ExecutionTreeObject::String(result) => Ok(result),
            ExecutionTreeObject::Symbol(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::string_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<u8> {
    fn from(object: ExecutionTreeObject) -> Result<u8> {
        match object {
            ExecutionTreeObject::U8(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::u8_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<u16> {
    fn from(object: ExecutionTreeObject) -> Result<u16> {
        match object {
            ExecutionTreeObject::U16(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::u16_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<u32> {
    fn from(object: ExecutionTreeObject) -> Result<u32> {
        match object {
            ExecutionTreeObject::U32(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::u32_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<u64> {
    fn from(object: ExecutionTreeObject) -> Result<u64> {
        match object {
            ExecutionTreeObject::U64(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::u64_str().to_string()).into())
            },
        }
    }
}

impl From<ExecutionTreeObject> for Result<usize> {
    fn from(object: ExecutionTreeObject) -> Result<usize> {
        match object {
            ExecutionTreeObject::USize(result) => Ok(result),
            object => {
                Err(ErrorKind::InvalidExecutionTreeObjectConversion(object.enum_to_string().to_string(), ExecutionTreeObject::usize_str().to_string()).into())
            },
        }
    }
}
