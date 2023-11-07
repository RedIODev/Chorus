// mod tree {
//     use crate::tree;

//     use super::{AstRootNode, AstBranchNode, AstLeafNode};
//     pub type Root = tree::Root<AstRootNode, AstBranchNode, AstLeafNode>;
//     pub type Branch = tree::Branch<AstRootNode, AstBranchNode, AstLeafNode>;
//     pub type Leaf = tree::Leaf<AstRootNode, AstBranchNode, AstLeafNode>;
// }

use bitflags::bitflags;

pub struct ParentPtr {
    parent: *mut AstNode,
}

pub trait AstNodeTrait {
    fn raw(&self) -> &str;
}

pub trait Child {
    fn parent(&self) -> &AstNode;

    fn parent_mut(&mut self) -> &mut AstNode;
}

pub trait Parent {
    fn children(&self) -> [&AstNode];

    fn children_mut(&mut self) -> [&mut AstNode];
}

pub enum AccessModifier {
    Default,
    Local,
    Public,
}

pub enum AllocationLocation {
    Heap,
    Stack
}

pub enum ConstrainType {
    Is,
    Satisfies
}

bitflags! {
    struct FuctionModifier: u8 {
        const DYNAMIC = 1;
        const INLINE = 1 << 1;
        const UNSAFE = 1 << 2;

    }
}

pub enum AstNode {
    Namespace(NamespaceNode),
    File(FileNode),
    Field(FileNode),
    UnnamedField(UnnamedFieldNode),
    NamedField(NamedFieldNode),
    GenericParameter(GenericParameterNode),
    Type(TypeNode),
    Struct(StructNode),
    PrimitiveStruct(PrimitiveStructNode),
    TupleStruct(TupleStructNode),
    FieldStruct(FieldStructNode),
    Interface(InterfaceNode),
    Function(FunctionNode),
    Implement(ImplementNode),
}

pub struct FileNode {
    name: String,
    namespaces: Vec<NamespaceNode>,
    imports: Vec<ImportNode>,
}

pub struct NamespaceNode {
    parent: ParentPtr,
    name: String,
}

pub struct ImportNode {
    parent: ParentPtr,
    path: NamespaceNode,
    r#type: Option<TypeNode>,
    alias: Option<String>,
}

pub enum FieldNode {
    Unnamed(UnnamedFieldNode),
    Named(NamedFieldNode),
}

pub struct UnnamedFieldNode {
    parent: ParentPtr,
    r#type: TypeNode,
    access_modifier: AccessModifier,
    is_const: bool,
    name: String,
}

pub struct NamedFieldNode {
    field_node: UnnamedFieldNode,
    name: String,
}

pub struct GenericParameterNode {
    parent: ParentPtr,
    name: String,
    default: Option<Box<TypeNode>>,
}

pub enum ConstrainNode {
    Generic(GenericContstrainNode),
    AllocationContext(AllocationContextContrainNode)
}

pub struct GenericContstrainNode {
    parent: ParentPtr,
    target: GenericParameterNode,
    constrain: InterfaceNode,
    constrain_type: ConstrainType
}

pub struct AllocationContextContrainNode {
    parent: ParentPtr,
    target: AllocationContextNode,
    constrain: AllocationContextNode,
}

pub struct AllocationContextNode {
    parent: ParentPtr,
    name: String,
    allocation_location:AllocationLocation

}

pub enum TypeNode {
    Struct(StructNode),
    Interface(InterfaceNode),
    Generic(GenericParameterNode),
}

pub enum StructNode {
    Primitive(PrimitiveStructNode),
    Tuple(TupleStructNode),
    Field(FieldStructNode),
}

pub struct PrimitiveStructNode {
    parent: ParentPtr,
    signed: bool,
    bits: u32,
    access_modifier: AccessModifier,
    name: String,
}

pub struct TupleStructNode {
    parent: ParentPtr,
    generic_parameters: Vec<GenericParameterNode>,
    generic_constrains: Vec<GenericContstrainNode>,
    fields: Vec<UnnamedFieldNode>,
    access_modifier: AccessModifier,
    name: String,
}

pub struct FieldStructNode {
    parent: ParentPtr,
    generic_parameters: Vec<GenericParameterNode>,
    generic_constrains: Vec<GenericContstrainNode>,
    fields: Vec<NamedFieldNode>,
    access_modifier: AccessModifier,
    name: String,
}

pub struct InterfaceNode {
    parent: ParentPtr,
    generic_parameters: Vec<GenericParameterNode>,
    generic_constrains: Vec<GenericContstrainNode>,
    access_modifier: AccessModifier,
    functions: Vec<FunctionNode>,
    name: String,

}

pub struct AnonymousTupleNode {
    parent: ParentPtr,
    fields: Vec<UnnamedFieldNode>
}

pub struct AnonymousUnionNode {
    parent: ParentPtr,
    varients: Vec<UnnamedFieldNode>,
}

pub enum StatementNode {
    Assignment(AssignmentNode),
    Scope(ScopeNode),
    If(IfNode),
    Else(ElseNode)
}

pub struct AssignmentNode {
    parent: ParentPtr,
    target: NamedFieldNode,
    source: Box<StatementNode>,
}

pub struct ScopeNode {
    parent: ParentPtr,
    statements: Vec<StatementNode>
}

pub struct IfNode {
    parent: ParentPtr,
    condition: Box<StatementNode>,
    body: Option<ScopeNode>,
    r#else: Option<ElseNode>,
}

pub struct ElseNode {
    parent: ParentPtr,
    body: Option<ScopeNode>
}

pub struct FunctionNode {
    parent: ParentPtr,
    generic_parameters: Vec<GenericParameterNode>,
    allocation_contexts: Vec<AllocationContextNode>,
    constrains: Vec<ConstrainType>,
    access_modifier: AccessModifier,
    function_modifier: FuctionModifier,
    parameters: Vec<NamedFieldNode>,
    return_type: TypeNode,
    body: Option<ScopeNode>,
}

pub struct ImplementNode {
    parent: ParentPtr,
}
