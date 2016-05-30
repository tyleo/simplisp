use lisp::AbstractSyntaxTreeNode;

pub enum AbstractSyntaxTreeObject<'a> {
    Node(AbstractSyntaxTreeNode<'a>),
    String(&'a str),
}
