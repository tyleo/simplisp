use lisp::AbstractSyntaxTreeObject;

pub struct AbstractSyntaxTreeNode<'a> {
    objects: Vec<AbstractSyntaxTreeObject<'a>>,
}

impl <'a> AbstractSyntaxTreeNode<'a> {
    pub fn new(objects: Vec<AbstractSyntaxTreeObject<'a>>) -> Self {
        AbstractSyntaxTreeNode {
            objects: objects,
        }
    }

    pub fn get_objects(&self) -> &Vec<AbstractSyntaxTreeObject<'a>> {
        &self.objects
    }
}
