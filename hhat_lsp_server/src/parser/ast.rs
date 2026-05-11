//! AST following the grammar files.
//!

#[allow(dead_code)]

use std::ops::Range;
use std::vec::IntoIter;


fn is_quantum(value: &String) -> bool { value.starts_with(&"@") }

pub trait TokenValue {
    type Output;

    fn new(value: String) -> Self::Output;
}


pub type Span = Range<usize>;

pub type Spanned<T> = (T, Span);

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleId {
    pub value: String,
    pub is_quantum: bool,
}


impl TokenValue for SimpleId {
    type Output = Self;

    fn new(value: String) -> Self {
        Self { is_quantum: is_quantum(&value), value }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct CompositeId {
    pub members: Vec<Spanned<SimpleId>>,
}


#[derive(Clone, Debug, PartialEq)]
pub enum IdKind {
    SimpleId(SimpleId),
    CompositeId(CompositeId),
}


#[derive(Clone, Debug, PartialEq)]
pub struct FullId {
    pub id: Spanned<IdKind>,
    pub modifier: Option<Spanned<Modifier>>
}


#[derive(Clone, Debug, PartialEq)]
pub enum Modifier {
    Ref,
    Pointer,
    Variadic,
    Values(Vec<ModifierValues>),
}


#[derive(Clone, Debug, PartialEq)]
pub enum ModifierValues {
    CallArgs(Spanned<CallArgs>),
    SimpleId(Spanned<SimpleId>),
}


#[derive(Clone, Debug, PartialEq)]
pub struct CallArgs {
    pub id: Spanned<FullId>,
    pub value: Spanned<ValOnly>,
}


#[derive(Clone, Debug, PartialEq)]
pub enum ValOnly {
    Array(Vec<Spanned<ArrayElem>>),
    FullId(Spanned<FullId>),
    Literal(Spanned<Literal>),
}


#[derive(Clone, Debug, PartialEq)]
pub enum ArrayElem {
    Literal(Spanned<Literal>),
    CompositeIdWithClosure(Spanned<CompositeIdWithClosure>),
    FullId(Spanned<FullId>),
}


#[derive(Clone, Debug, PartialEq)]
pub struct Literal {
    pub value: LiteralOptions,
    pub modifier: Option<Modifier>,
}


#[derive(Clone, Debug, PartialEq)]
pub enum LiteralOptions {
    Bool(BoolLit),
    Int(IntLit),
    Float(FloatLit),
    Str(StrLit),
}


#[derive(Clone, Debug, PartialEq)]
pub struct BoolLit {
    pub value: String,
    pub is_quantum: bool,
}

impl TokenValue for BoolLit {
    type Output = Self;

    fn new(value: String) -> Self::Output {
        Self { is_quantum: is_quantum(&value), value }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct IntLit {
    pub value: String,
    pub is_quantum: bool,
}

impl TokenValue for IntLit {
    type Output = Self;

    fn new(value: String) -> Self::Output {
        Self { is_quantum: is_quantum(&value), value }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct FloatLit {
    pub value: String,
    pub is_quantum: bool,
}

impl TokenValue for FloatLit {
    type Output = Self;

    fn new(value: String) -> Self::Output {
        Self { is_quantum: is_quantum(&value), value }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct StrLit {
    pub value: String,
    pub is_quantum: bool,
}

impl TokenValue for StrLit {
    type Output = Self;

    fn new(value: String) -> Self::Output {
        Self { is_quantum: is_quantum(&value), value }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct CompositeIdWithClosure {
    pub root: Spanned<FullId>,
    pub children: Vec<Spanned<CompositeIdOptions>>
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompositeIdOptions {
    CompositeIdWithClosure(Spanned<CompositeIdWithClosure>),
    FullId(Spanned<FullId>),
}


/*------------
    Imports
------------*/

#[derive(Clone, Debug, PartialEq)]
pub enum Imports {
    TypeImport(Vec<Spanned<ImportElem>>),
    FnImport(Vec<Spanned<ImportElem>>),
    MetaFnImport(Vec<Spanned<ImportElem>>),
    ModifierImport(Vec<Spanned<ImportElem>>),
    SuperTypeImport(Vec<Spanned<ImportElem>>),
    ConstImport(Vec<Spanned<ImportElem>>),
}


#[derive(Clone, Debug, PartialEq)]
pub enum ImportElem {
    SingleImport(FullId),
    ManyImports(CompositeIdWithClosure),
}


/*-------------
    Closures
------------ */

#[derive(Clone, Debug, PartialEq)]
pub enum GroupFns {
    FnDef(FnDef),
    MetaFnDef(MetaFnDef),
    ModifierDef(ModifierDef),
    SuperTypeDef(SuperTypeDef),
}


#[derive(Clone, Debug, PartialEq)]
pub struct FnDef {
    pub name: Spanned<SimpleId>,
    pub args: Spanned<FnArgs>,
    pub ty: Spanned<TypeId>,
    pub body: Spanned<FnBody>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct FnArgs {
    pub args: Vec<Spanned<ArgType>>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct ArgType {
    pub arg: Spanned<FullId>,
    pub value: Spanned<TypeId>,
}


#[derive(Clone, Debug, PartialEq)]
pub enum TypeId {
    Single(FullId),
    Array(FullId),
}


#[derive(Clone, Debug, PartialEq)]
pub struct FnBody {
    body: Vec<FnBodyOptions>,
}

impl FnBody {
    pub fn new() -> Self { Self { body: vec![] } }

    pub fn push(&mut self, value: FnBodyOptions) {
        self.body.push(value)
    }

    pub fn into_iter(self) -> IntoIter<FnBodyOptions> {
        self.body.into_iter()
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum FnBodyOptions {
    FnReturn(Expr),
    DeclareAssign(DeclareAssign),
    DeclareAssignDS(DeclareAssignDS),
    Declare(Declare),
    AssignDS(AssignDS),
    Assign(Assign),
    Expr(Expr),
}


#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Cast(Cast),
    AssignDS(AssignDS),
    CallOptn(CallOptn),
    CallOptbdn(CallOptbdn),
    CallBdn(CallBdn),
    Call(Call),
    Array(Vec<Spanned<ArrayElem>>),
    FullId(Spanned<FullId>),
    Literal(Spanned<Literal>),
}


#[derive(Clone, Debug, PartialEq)]
pub struct Cast {
    pub data: CastFromData,
    pub to_ty: Spanned<TypeId>,
}


#[derive(Clone, Debug, PartialEq)]
pub enum CastFromData {
    Call(Spanned<Call>),
    Literal(Spanned<Literal>),
    Array(Vec<Spanned<ArrayElem>>),
    FullId(Spanned<FullId>),
}


#[derive(Clone, Debug, PartialEq)]
pub struct AssignDS {
    pub root: Spanned<FullId>,
    pub members: Spanned<MemberAssign>,
}


#[derive(Clone, Debug, PartialEq)]
pub enum MemberAssign {
    Assign(Vec<Assign>),
    Expr(Vec<Expr>),
}


#[derive(Clone, Debug, PartialEq)]
pub struct Assign {
    pub value: Spanned<FullId>,
    pub expr: Spanned<Expr>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct CallOptn {
    pub value: Spanned<FullId>,
    pub options: Vec<Spanned<Options>>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Options {
    pub entry: Spanned<Expr>,
    pub body: Spanned<Body>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Call {
    pub caller: Spanned<FullId>,
    pub args: Vec<Spanned<Args>>,
    pub modifier: Option<Spanned<Modifier>>,
}


#[derive(Clone, Debug, PartialEq)]
pub enum Args {
    CallArgs(CallArgs),
    Cast(Cast),
    Call(Call),
    ValOnly(ValOnly),
}


#[derive(Clone, Debug, PartialEq)]
pub struct CallOptbdn {
    pub value: Spanned<FullId>,
    pub args: Vec<Spanned<Args>>,
    pub options: Vec<Spanned<Options>>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct CallBdn {
    pub value: Spanned<FullId>,
    pub args: Vec<Spanned<Args>>,
    pub body: Spanned<Body>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Body {
    value: Vec<BodyStmt>,
}

impl Body {
    pub fn new() -> Self { Self { value: vec![] } }

    pub fn push(&mut self, value: BodyStmt) {
        self.value.push(value)
    }

    pub fn into_iter(self) -> IntoIter<BodyStmt> {
        self.value.into_iter()
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct Declare {
    pub value: Spanned<SimpleId>,
    pub modifier: Option<Spanned<Modifier>>,
    pub ty: Spanned<TypeId>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct DeclareAssign {
    pub value: Spanned<SimpleId>,
    pub modifier: Option<Spanned<Modifier>>,
    pub ty: Spanned<TypeId>,
    pub expr: Spanned<Expr>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct DeclareAssignDS {
    pub value: Spanned<SimpleId>,
    pub modifier: Option<Spanned<Modifier>>,
    pub ty: Spanned<TypeId>,
    pub members: Vec<Spanned<Assign>>,
}


#[derive(Clone, Debug, PartialEq)]
pub enum BodyStmt {
    DeclareAssign(DeclareAssign),
    DeclareAssignDS(DeclareAssignDS),
    Declare(Declare),
    Assign(Assign),
    Expr(Expr),
}


#[derive(Clone, Debug, PartialEq)]
pub struct MetaFnDef {
    pub value: Spanned<SimpleId>,
    pub args: Vec<FnArgs>,
    pub ty: Option<TypeId>,
    pub body: Spanned<MetaFnBody>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct MetaFnBody {
    body: Vec<MetaFnBodyStmt>,
}


#[derive(Clone, Debug, PartialEq)]
pub enum MetaFnBodyStmt {
    FnReturn(Expr),
    Option(Options),
    DeclareAssign(DeclareAssign),
    DeclareAssignDS(DeclareAssignDS),
    Declare(Declare),
    AssignDS(AssignDS),
    Assign(Assign),
    Expr(Expr),
}


#[derive(Clone, Debug, PartialEq)]
pub struct ModifierDef {
    pub value: Spanned<ModifierName>,
    pub args: Vec<Spanned<FnArgs>>,
    pub ty: Spanned<TypeId>,
    pub body: Spanned<MetaFnBody>,
}


#[derive(Clone, Debug, PartialEq)]
pub enum ModifierName {
    Ref,
    Pointer,
    Variadic,
    SimpleId(SimpleId),
}


#[derive(Clone, Debug, PartialEq)]
pub struct SuperTypeDef {
    pub value: Spanned<SimpleId>,
    pub body: Vec<Spanned<FullId>>,
}


#[derive(Clone, Debug, PartialEq)]
pub enum TypeDef {
    TypeStruct(TypeStruct),
    TypeEnum(TypeEnum),
}


#[derive(Clone, Debug, PartialEq)]
pub struct TypeStruct {
    pub name: Spanned<SimpleId>,
    pub members: Vec<StructMember>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct StructMember {
    pub member_name: Spanned<SimpleId>,
    pub ty: Spanned<TypeId>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct TypeEnum {
    pub name: Spanned<SimpleId>,
    pub variants: Vec<Spanned<EnumVariants>>,
}


#[derive(Clone, Debug, PartialEq)]
pub enum EnumVariants {
    Named(SimpleId),
    TaggedUnion(TypeStruct),
}


#[derive(Clone, Debug, PartialEq)]
pub struct ConstDef {
    pub name: Spanned<SimpleId>,
    pub ty: Spanned<TypeId>,
    pub value: Spanned<Expr>,
}


/*-------------
    Program
-------------*/

#[derive(Clone, Debug, PartialEq)]
pub struct FnProgram {
    pub imports: Vec<Imports>,
    pub group_fns: Vec<GroupFns>,
    pub main: Option<Body>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct TypeProgram {
    pub imports: Vec<Imports>,
    pub type_def: Vec<TypeDef>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct ConstProgram {
    pub imports: Vec<Imports>,
    pub const_def: Vec<ConstDef>,
}
