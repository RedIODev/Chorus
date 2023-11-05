use crate::{
    error::BorrowError,
    weakbox::{RcBox, Ref, RefMut, Weak},
};

#[derive(Debug)]
pub struct Tree<RT, BT, LT> {
    root: RcBox<Parent<RT, BT, LT>>,
}

impl<RT,BT,LT> Tree<RT,BT,LT> {
    
}



#[derive(Debug)]
pub struct Root<RT, BT, LT> {
    children: Vec<RcBox<Child<RT, BT, LT>>>,
    value: RT,
}

impl<RT, BT, LT> Root<RT, BT, LT> {
    pub fn new(value: RT) -> Self {
        Self {
            children: Vec::new(),
            value,
        }
    }

    pub fn value(&self) -> &RT {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut RT {
        &mut self.value
    }

    pub fn children(&self) -> &[RcBox<Child<RT, BT, LT>>] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut [RcBox<Child<RT, BT, LT>>] {
        &mut self.children
    }
}

#[derive(Debug)]
pub struct Branch<RT, BT, LT> {
    parent: Weak<Parent<RT, BT, LT>>,
    children: Vec<RcBox<Child<RT, BT, LT>>>,
    value: BT,
}

impl<RT, BT, LT> Branch<RT, BT, LT> {
    pub fn new(parent: Weak<Parent<RT, BT, LT>>, value: BT) -> Self {
        Self {
            parent,
            children: Vec::new(),
            value,
        }
    }

    pub fn value(&self) -> &BT {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut BT {
        &mut self.value
    }

    pub fn parent(&self) -> Result<Ref<Parent<RT, BT, LT>>, BorrowError> {
        self.parent.try_borrow().map(Option::unwrap)
    }

    pub fn parent_mut(&mut self) -> Result<RefMut<Parent<RT, BT, LT>>, BorrowError> {
        self.parent.try_borrow_mut().map(Option::unwrap)
    }

    pub fn children(&self) -> &[RcBox<Child<RT, BT, LT>>] {
        &self.children
    }

    pub fn children_mut(&mut self) -> &mut [RcBox<Child<RT, BT, LT>>] {
        &mut self.children
    }
}

#[derive(Debug)]
pub struct Leaf<RT, BT, LT> {
    parent: Weak<Parent<RT, BT, LT>>,
    value: LT,
}

impl<RT, BT, LT> Leaf<RT, BT, LT> {
    pub fn new(parent: Weak<Parent<RT, BT, LT>>, value: LT) -> Self {
        Self { parent, value }
    }

    pub fn value(&self) -> &LT {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut LT {
        &mut self.value
    }

    pub fn parent(&self) -> Result<Ref<Parent<RT, BT, LT>>, BorrowError> {
        self.parent.try_borrow().map(Option::unwrap)
    }

    pub fn parent_mut(&mut self) -> Result<RefMut<Parent<RT, BT, LT>>, BorrowError> {
        self.parent.try_borrow_mut().map(Option::unwrap)
    }
}

#[derive(Debug)]
pub enum Value<RT, BT, LT> {
    Root(RT),
    Branch(BT),
    Leaf(LT),
}

#[derive(Debug)]
pub enum Child<RT, BT, LT> {
    Branch(Branch<RT, BT, LT>),
    Leaf(Leaf<RT, BT, LT>),
}

impl<RT, BT, LT> Child<RT, BT, LT> {
    pub fn value(&self) -> ChildValue<&BT, &LT> {
        match self {
            Child::Branch(b) => ChildValue::Branch(b.value()),
            Child::Leaf(l) => ChildValue::Leaf(l.value()),
        }
    }

    pub fn value_mut(&mut self) -> ChildValue<&mut BT, &mut LT> {
        match self {
            Child::Branch(b) => ChildValue::Branch(b.value_mut()),
            Child::Leaf(l) => ChildValue::Leaf(l.value_mut()),
        }
    }

    pub fn parent(&self) -> Result<Ref<Parent<RT, BT, LT>>, BorrowError> {
        match self {
            Child::Branch(b) => b.parent(),
            Child::Leaf(l) => l.parent(),
        }
    }

    pub fn parent_mut(&mut self) -> Result<RefMut<Parent<RT, BT, LT>>, BorrowError> {
        match self {
            Child::Branch(b) => b.parent_mut(),
            Child::Leaf(l) => l.parent_mut(),
        }
    }
}

impl<RT, BT, LT> From<Branch<RT, BT, LT>> for Child<RT, BT, LT> {
    fn from(value: Branch<RT, BT, LT>) -> Self {
        Child::Branch(value)
    }
}

impl<RT, BT, LT> From<Leaf<RT, BT, LT>> for Child<RT, BT, LT> {
    fn from(value: Leaf<RT, BT, LT>) -> Self {
        Child::Leaf(value)
    }
}

#[derive(Debug)]
pub enum ChildValue<BT, LT> {
    Branch(BT),
    Leaf(LT),
}

#[derive(Debug)]
pub enum Parent<RT, BT, LT> {
    Root(Root<RT, BT, LT>),
    Branch(Branch<RT, BT, LT>),
}

impl<RT, BT, LT> Parent<RT, BT, LT> {
    fn vec(&self) -> &Vec<RcBox<Child<RT, BT, LT>>> {
        match self {
            Parent::Root(r) => &r.children,
            Parent::Branch(b) => &b.children,
        }
    }

    fn vec_mut(&mut self) -> &mut Vec<RcBox<Child<RT, BT, LT>>> {
        match self {
            Parent::Root(r) => &mut r.children,
            Parent::Branch(b) => &mut b.children,
        }
    }

    pub fn value(&self) -> ParentValue<&RT, &BT> {
        match self {
            Parent::Root(r) => ParentValue::Root(r.value()),
            Parent::Branch(b) => ParentValue::Branch(b.value()),
        }
    }

    pub fn value_mut(&mut self) -> ParentValue<&mut RT, &mut BT> {
        match self {
            Parent::Root(r) => ParentValue::Root(r.value_mut()),
            Parent::Branch(b) => ParentValue::Branch(b.value_mut()),
        }
    }

    pub fn children(&self) -> &[RcBox<Child<RT, BT, LT>>] {
        self.vec()
    }

    pub fn children_mut(&mut self) -> &mut [RcBox<Child<RT, BT, LT>>] {
        self.vec_mut()
    }
}

trait RcParent<RT, BT, LT> {
    fn add(&mut self, value: ChildValue<BT, LT>) -> Result<(), BorrowError>;
}

impl<RT, BT, LT> RcParent<RT, BT, LT> for RcBox<Parent<RT, BT, LT>> {
    fn add(&mut self, value: ChildValue<BT, LT>) -> Result<(), BorrowError> {
        let child = match value {
            ChildValue::Branch(b) => Branch::new(self.weak(), b).into(),
            ChildValue::Leaf(l) => Leaf::new(self.weak(), l).into(),
        };
        self.try_borrow_mut()?.vec_mut().push(RcBox::new(child));
        Ok(())
    }
}

impl<RT, BT, LT> From<Root<RT, BT, LT>> for Parent<RT, BT, LT> {
    fn from(value: Root<RT, BT, LT>) -> Self {
        Parent::Root(value)
    }
}

impl<RT, BT, LT> From<Branch<RT, BT, LT>> for Parent<RT, BT, LT> {
    fn from(value: Branch<RT, BT, LT>) -> Self {
        Parent::Branch(value)
    }
}

#[derive(Debug)]
pub enum ParentValue<RT, BT> {
    Root(RT),
    Branch(BT),
}
