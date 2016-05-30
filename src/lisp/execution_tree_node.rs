use error::LispResult as R;
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

    pub fn to_string(&self) -> R<String> {
        let mut result = String::new();
        result.push('(');
        for object in &self.objects {
            let string = try!(object.to_string());
            result.push_str(&string);
            result.push(' ');
        }
        if let Some(last_char) = result.pop() {
            if last_char != ' ' {
                result.push(last_char);
            }
        } else {
            panic!();
        }
        result.push(')');
        Ok(result)
    }
}
