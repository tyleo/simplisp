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
