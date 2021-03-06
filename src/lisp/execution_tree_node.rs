use error::*;
use lisp::ExecutionTreeObject;

#[derive(Clone)]
pub struct ExecutionTreeNode {
    objects: Vec<ExecutionTreeObject>,
}

impl ExecutionTreeNode {
    pub fn new(objects: Vec<ExecutionTreeObject>) -> Self {
        ExecutionTreeNode {
            objects: objects,
        }
    }

    pub fn get_objects(&self) -> &Vec<ExecutionTreeObject> {
        &self.objects
    }

    pub fn into_objects(self) -> Vec<ExecutionTreeObject> {
        self.objects
    }

    pub fn nil() -> Self {
        ExecutionTreeNode {
            objects: Vec::new(),
        }
    }

    pub fn to_string(&self) -> Result<String> {
        let mut result = String::new();
        result.push('(');
        for object in &self.objects {
            let string = try!(object.to_string());
            result.push_str(&string);
            result.push(' ');
        }
        if result.len() != 1 {
            result.pop();
        }
        result.push(')');
        Ok(result)
    }
}
