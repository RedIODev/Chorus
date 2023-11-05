use std::path::{Path, PathBuf};

use refbox::{Ref, RefBox};

use uuid::Uuid;

use crate::{lexer::Token, error::AstError};



pub trait NodeTrait {
    fn token(&self) -> Result<Vec<&Token>, AstError>;
}

pub trait ContainerNodeTrait: NodeTrait {
    fn children(&self) -> Result<&Vec<ContainedNode>, AstError>;
    fn children_mut(&mut self) -> Result<&mut Vec<ContainedNode>, AstError>;
}

pub enum ContainerNode {
    Root(Ref<RootNode>),
    Branch(Ref<BranchNode>)
}

pub trait ContainedNodeTrait: NodeTrait {
    fn parent(&self) -> Result<&ContainerNode, AstError>;
    fn parent_mut(&mut self) -> Result<&mut ContainerNode, AstError>;
}

pub enum ContainedNode {
    Branch(RefBox<BranchNode>),
    Leaf(RefBox<LeafNode>)
}

pub struct RootNode {
    pub(super) path:PathBuf,
    pub(super) children:Vec<ContainedNode>
}



pub enum BranchNode {

}

pub enum LeafNode {

}


pub mod test {
    use std::{path::PathBuf, rc::Rc};

    use crate::lexer::Token;

    pub type RcBox<T> = Rc<T>;

    pub trait NodeTrait {
        fn token(&self) -> Vec<&Token>;
    }

    pub struct RootNode {
        pub path:PathBuf,
        pub children:Vec<ContainedNode>
    }

    pub enum BranchNode {

    }
    
    pub enum LeafNode {
    
    }

    pub enum ContainerNode {
        Root(RcBox<RootNode>),
        Branch(RcBox<BranchNode>)
    }

    pub enum ContainedNode {
        Branch(RcBox<BranchNode>),
        Leaf(RcBox<LeafNode>)
    }
}




///
/// Old
/// 


pub trait AstNodeTrait {
    fn raw(&self) -> &str;
    fn children(&self) -> &[&AstNodeOld];
    fn parent(&self) -> Option<&AstNodeOld>;
}

#[derive(Debug)]
pub enum AstNodeOld {

}

#[derive(Debug)]
pub enum Visability {
    Public,
    Local,
    Default
}

pub struct Constrain {

}

pub struct GenericParameterAstNode {
    raw: Box<str>,
    ident: Identifier,
    parent: Ref<AstNodeOld>
}

pub struct Identifier {
    uuid: Uuid,// based on path and name, recursive until root
    name: Box<str>,
    parent:Ref<AstNodeOld>,
    path: Box<[Identifier]>
}

pub struct PrimitiveStructAstNode {
    raw: Box<str>,
    visability: Visability,
    ident: Identifier,
    bytes: u8,
    signed: bool,
    generic_parameters: Box<[GenericParameterAstNode]>,
}