use lisp::AbstractSyntaxTreeNode;

#[derive(Debug)]
pub enum AbstractSyntaxTreeObject<'a> {
    Node(AbstractSyntaxTreeNode<'a>),
    String(&'a str),
}
