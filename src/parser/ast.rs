use std::collections::HashMap;



pub enum AstNode {
    Namespace(Namespace),
    Function(Function),
    Type(Type)
}

pub enum Visibility {
    Default,
    File,
    Private,
    Export
}

pub struct Annotation {
    name: String,
    arguments: HashMap<String, String>
}

pub struct Namespace {
    name: String,
    visibility: Visibility,
    annotations: Vec<Annotation>,
    members:Vec<AstNode>
}

pub struct Function {
    name: String,
    visibility: Visibility,
    constness:bool,
    annotations: Vec<Annotation>,
    arguments: Vec<Argument>,
    generic_arguments: Vec<GenericArgument>,
    return_type: PathName,
    state:bool,
    abi: String,
    body: Option<Body>
}

pub struct Body {
    statements: Vec<Statement>
}

pub enum Statement {
    Variable(VariableStatement),
    Return(ReturnStatement),
    Yield(YieldStatement),
    If(IfExpression),
    Loop(LoopExpression),
    FunctionCall(FunctionCallExpression),
}

pub enum Expression {
    If(IfExpression),
    Loop(LoopExpression),
    Switch(SwitchExpression),
    FunctionCall(FunctionCallExpression),
    Lambda(LambdaExpression),
    Operator(OperatorExpression),
    Instance(InstanceExpression),
    Literal(LiteralExpression),
    Block(BlockExpression),
    Identifier(String)
}

pub struct BlockExpression {
    statements: Vec<Statement>,
    annotations: Vec<Annotation>
}

pub enum LiteralExpression {
    String(StringLiteral),
    Numeric(NumericLiteral),
    Char(CharLiteral),
}

pub struct CharLiteral {
    char: String,
    type_: LiteralType
}

pub struct NumericLiteral {
    number: String,
    byte_size: u16
}

pub struct StringLiteral {
    text: String,
    type_: LiteralType,
    annotations: Vec<Annotation>
}

pub enum LiteralType {
    Default,
    Raw,
    Byte,
    RawByte
}

pub enum InstanceExpression {
    Struct(StructInstance),
    Union(UnionInstance),
    Array(ArrayInstance),
    Tuple(TupleInstance)
}

pub struct TupleInstance {
    elements: Vec<Expression>
}

pub struct ArrayInstance {
    elements: Vec<Expression>
}

pub struct UnionInstance {
    variant: String,
    annotations: Vec<Annotation>,
    type_: PathName,
}

pub struct StructInstance {
    arguments: Vec<StructFieldAssignment>,
    annotations: Vec<Annotation>,
    type_: PathName,
    copy_assignment: Option<Box<Expression>>
}

pub struct StructFieldAssignment {
    field_name: String,
    value: Expression
}

pub enum OperatorExpression {
    UnaryOperator(UnaryOperator),
    BinaryOperator(BinaryOperator)
}

pub struct UnaryOperator {
    input: Box<Expression>,
    operator: UnaryType,
    checked: bool,
}

pub enum UnaryType {
    Deref,
    Negate,
    Reference,
    LeftOpenRange(RangeType),
    RightOpenRange(RangeType),
    Branch
}

pub struct BinaryOperator {
    input1: Box<Expression>,
    input2: Box<Expression>,
    operator: BinaryType,
    checked:bool,
    reassigning: bool,
}

pub enum BinaryType {
    ReassigningType(ReassignableType, bool),
    Assignment,
    NonEqual,
    LogicAnd,
    LogicOr,
    Equal,
    Less,
    LessEquals,
    Grater,
    GraterEquals,
    FiledAccess,
    Range {
        left: RangeType,
        right:RangeType
    },

}

pub enum ReassignableType {
    Modulo,
    BitAnd,
    BitXor,
    BitOr,
    Multiply,
    Addition,
    Subtraction,
    Division,
    LeftShift,
    RightShift
}

pub enum RangeType {
    Inclusive,
    Exclusive
}

pub struct LambdaExpression {
    arguments: Vec<LambdaArgument>,
    return_type: Option<PathName>,
    annotations: Vec<Annotation>,
    body: Body
}

pub struct LambdaArgument {
    name: String,
    type_: Option<PathName>,
    annotations: Vec<Annotation>
}

pub struct SwitchExpression {
    argument: Box<Expression>,
    branches: Vec<SwitchBranch>
}

pub struct SwitchBranch {
    pattern: String,
    guard: Option<Expression>,
    body: Body
}

pub struct FunctionCallExpression {
    function: PathName,
    arguments: Vec<Expression>,
    generic_arguments: Vec<PathName>
}

pub enum LoopExpression {
    ForEach(ForEachExpression),
    While(WhileExpression)
}

pub struct ForEachExpression {
    variable: Argument,
    iterator: Box<Expression>,
    annotations: Vec<Annotation>,
    body: Body,
    label: String
}

pub struct WhileExpression {
    condition: Box<Expression>,
    annotations: Vec<Annotation>,
    body: Body,
    label: String
}

pub enum VariableStatement {
    Definition(VariableDefinition),
    Assignment(VariableDefinition),
}

pub struct VariableDefinition {
    name: String,
    mutability: bool,
    type_: Option<PathName>,
    value: Option<Expression>,
    annotations: Vec<Annotation>
}

pub struct VariableAssignment {
    name: String,
    value: Expression
}

pub struct ReturnStatement {
    value: Option<Expression>
}

pub struct YieldStatement {
    value: Option<Expression>
}

pub struct IfExpression {
    condition: Box<Expression>,
    annotations: Vec<Annotation>,
    else_block: Option<Body>,
    body: Body,
    return_type: Option<PathName>
}

pub struct PathName {
    name: String,
    path: Vec<String>,
}

pub struct Argument {
    name: String,
    type_: PathName,
    annotation: Annotation
}

pub enum GenericArgument {
    Type(TypeGeneric),
    Constant(Field)
}

pub struct TypeGeneric {
    name: String,
    default_value: Option<String>,
    constrains: Vec<Constrain>
}

pub enum Constrain {
    Satisfies(PathName),
    Is(PathName)
}

pub enum Type {
    Struct(Struct),
    Union(Union),
    Enum(Enum),
    Interface(Interface),
}

pub enum Struct {
    Field(FieldStruct),
    Tuple(TupleStruct),
    Structure(StructureStruct)
}

pub struct FieldStruct {
    name: String,
    visibility: Visibility,
    annotations: Vec<Annotation>,
    generic_arguments: Vec<GenericArgument>,
    fields: Vec<Field>
}

pub struct TupleStruct {
    name: String,
    visibility: Visibility,
    annotations: Vec<Annotation>,
    generic_arguments: Vec<GenericArgument>,
    types: Vec<PathName>
}

pub struct StructureStruct {
    fields: Vec<Field>
}

pub struct Field {
    name: String,
    type_: PathName,
    annotations: Vec<Annotation>,
    visibility: Visibility
}

pub enum Union {
    Field(FieldUnion),
    Structure(StructureUnion),
}

pub struct FieldUnion {
    name: String,
    visibility: Visibility,
    annotations: Vec<Annotation>,
    generic_arguments: Vec<GenericArgument>,
    variants: Vec<Field>
}

pub struct StructureUnion {
    variants: Vec<Field>
}

pub struct Enum {
    name: String,
    visibility: Visibility,
    annotations: Vec<Annotation>,
    generic_arguments: Vec<GenericArgument>,
    type_: PathName,
    values: Vec<EnumValue>
}

pub struct EnumValue {
    name: String,
    value: String,
    annotations: Vec<Annotation>
}

pub struct Interface {
    name: String,
    visibility: Visibility,
    annotations: Vec<Annotation>,
    generic_arguments: Vec<GenericArgument>,
    super_types: Vec<PathName>,
    functions: Vec<Function>,
    constants: Vec<Field>
}

pub struct Constant {
    field: Field,
    value: Expression,
}