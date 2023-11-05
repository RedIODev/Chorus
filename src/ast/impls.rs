use std::path::PathBuf;

use refbox::RefBox;

use crate::{lexer::Token, error::AstError};

use super::{types::RootNode, ContainerNodeTrait, ContainedNode, NodeTrait, ContainerNode, ContainedNodeTrait};

impl NodeTrait for ContainerNode {
    fn token(&self) -> Result<Vec<&Token>, AstError> {
       match self {
        ContainerNode::Root(r) => todo!(),
        ContainerNode::Branch(b) => todo!(),
    }
    }
}

impl ContainerNodeTrait for ContainerNode {
    fn children(&self) -> Result<&Vec<ContainedNode>, AstError> {
        match self {
            ContainerNode::Root(r) => todo!(),
            ContainerNode::Branch(_) => todo!(),
        }
    }

    fn children_mut(&mut self) -> Result<&mut Vec<ContainedNode>, AstError> {
        todo!()
    }
}

impl NodeTrait for ContainedNode {
    fn token(&self) -> Result<Vec<&Token>, AstError> {
        match self {
            ContainedNode::Branch(b) => todo!(),
            ContainedNode::Leaf(l) => todo!(),
        }
    }
}

impl ContainedNodeTrait for ContainedNode {
    fn parent(&self) -> Result<&ContainerNode, AstError> {
        todo!()
    }

    fn parent_mut(&mut self) -> Result<&mut ContainerNode, AstError> {
        todo!()
    }
}

impl RootNode {
    pub fn new(path: PathBuf) -> RefBox<RootNode> {
        RefBox::new(Self { path, children: Vec::new() })
    }
}

impl NodeTrait for RootNode {
    fn token(&self) -> Result<Vec<&Token>, AstError> {
        let mut vec = Vec::new();
        for child in self.children.iter() {
            for token in child.token()? {
                vec.push(token);
            }
        }
        Ok(vec)
    }
}

impl ContainerNodeTrait for RootNode {
    fn children(&self) -> Result<&Vec<ContainedNode>, AstError> {
        Ok(&self.children)
    }

    fn children_mut(&mut self) -> Result<&mut Vec<ContainedNode>, AstError> {
        Ok(&mut self.children)
    }
}

pub mod test {
    use crate::ast::types::test::{NodeTrait, RootNode, ContainedNode, ContainerNode, BranchNode, LeafNode};

    impl NodeTrait for RootNode {
        fn token(&self) -> Vec<&crate::lexer::Token> {
            let mut vec = Vec::new();
            for child in self.children.iter() {
                for token in child.token() {
                    vec.push(token)
                }
            }
            vec
        }
    }

    impl NodeTrait for ContainerNode {
        fn token(&self) -> Vec<&crate::lexer::Token> {
            match self {
                ContainerNode::Root(r) => r.token(),
                ContainerNode::Branch(b) => b.token(),
            }
        }
    }

    impl NodeTrait for ContainedNode {
        fn token(&self) -> Vec<&crate::lexer::Token> {
            match self {
                ContainedNode::Branch(b) => b.token(),
                ContainedNode::Leaf(l) => l.token(),
            }
        }
    }

    impl NodeTrait for BranchNode {
        fn token(&self) -> Vec<&crate::lexer::Token> {
            todo!()
        }
    }

    impl NodeTrait for LeafNode {
        fn token(&self) -> Vec<&crate::lexer::Token> {
            todo!()
        }
    }
}