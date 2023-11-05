// use std::{
//     cell::{Ref, RefCell},
//     ops::Deref,
//     rc::{Rc, Weak},
// };

// use crate::error::BorrowError;

// type RcCell<T> = Rc<RefCell<T>>;
// type WeakCell<T> = Weak<RefCell<T>>;

// #[derive(Debug)]
// pub struct Root<RT, BT, LT> {
//     children: Vec<RcCell<Child<RT, BT, LT>>>,
//     value: RT,
// }

// impl<RT, BT, LT> Root<RT, BT, LT> {
//     pub fn new(value: RT) -> Self {
//         Self {
//             children: Vec::new(),
//             value,
//         }
//     }
// }

// pub trait RcParent<BT, LT> {
//     fn try_add_child(&mut self, child_value: ChildValue<BT, LT>) -> Result<(), BorrowError>;
// }

// impl<RT, BT, LT> RcParent<BT, LT> for RcCell<Parent<RT, BT, LT>> {
//     fn try_add_child(&mut self, child_value: ChildValue<BT, LT>) -> Result<(), BorrowError> {
//         let parent = Rc::downgrade(self);
//         let node = match child_value {
//             ChildValue::Brach(b) => Child::Branch(Branch::new(parent, b)),
//             ChildValue::Leaf(l) => Child::Leaf(Leaf::new(parent, l)),
//         };
//         self.try_borrow_mut()?
//             .vec_mut()
//             .push(Rc::new(RefCell::new(node)));

//         Ok(())
//     }
// }

// #[derive(Debug)]
// pub struct Branch<RT, BT, LT> {
//     parent: WeakCell<Parent<RT, BT, LT>>,
//     children: Vec<RcCell<Child<RT, BT, LT>>>,
//     value: BT,
// }

// impl<RT, BT, LT> Branch<RT, BT, LT> {
//     fn new(parent: WeakCell<Parent<RT, BT, LT>>, value: BT) -> Self {
//         Self {
//             parent,
//             children: Vec::new(),
//             value,
//         }
//     }
// }

// #[derive(Debug)]
// pub struct Leaf<RT, BT, LT> {
//     parent: WeakCell<Parent<RT, BT, LT>>,
//     value: LT,
// }

// impl<RT, BT, LT> Leaf<RT, BT, LT> {
//     fn new(parent: WeakCell<Parent<RT, BT, LT>>, value: LT) -> Self {
//         Self { parent, value }
//     }
// }

// #[derive(Debug)]
// pub enum Parent<RT, BT, LT> {
//     Root(Root<RT, BT, LT>),
//     Branch(Branch<RT, BT, LT>),
// }

// impl<RT, BT, LT> Parent<RT, BT, LT> {
//     pub fn value(&self) -> ParentValue<&RT, &BT> {
//         match self {
//             Parent::Root(r) => ParentValue::Root(&r.value),
//             Parent::Branch(b) => ParentValue::Branch(&b.value),
//         }
//     }

//     pub fn value_mut(&mut self) -> ParentValue<&mut RT, &mut BT> {
//         match self {
//             Parent::Root(r) => ParentValue::Root(&mut r.value),
//             Parent::Branch(b) => ParentValue::Branch(&mut b.value),
//         }
//     }

//     pub fn children(&self) -> &[RcCell<Child<RT, BT, LT>>] {
//         self.vec()
//     }

//     pub fn children_mut(&mut self) -> &mut [RcCell<Child<RT, BT, LT>>] {
//         self.vec_mut()
//     }

//     fn vec(&self) -> &Vec<RcCell<Child<RT, BT, LT>>> {
//         match self {
//             Parent::Root(r) => &r.children,
//             Parent::Branch(b) => &b.children,
//         }
//     }

//     ///
//     /// private as it could destroy the structure of the Tree if used incorrectly
//     ///
//     fn vec_mut(&mut self) -> &mut Vec<RcCell<Child<RT, BT, LT>>> {
//         match self {
//             Parent::Root(r) => &mut r.children,
//             Parent::Branch(b) => &mut b.children,
//         }
//     }
// }

// #[derive(Debug)]
// pub enum ParentValue<RT, BT> {
//     Root(RT),
//     Branch(BT),
// }

// #[derive(Debug)]
// pub enum Value<RT, BT, LT> {
//     Root(RT),
//     Branch(BT),
//     Leaf(LT),
// }

// #[derive(Debug)]
// pub enum Child<RT, BT, LT> {
//     Branch(Branch<RT, BT, LT>),
//     Leaf(Leaf<RT, BT, LT>),
// }

// impl<RT, BT, LT> Child<RT, BT, LT> {
//     pub fn value(&self) -> ChildValue<&BT, &LT> {
//         match self {
//             Child::Branch(b) => ChildValue::Brach(&b.value),
//             Child::Leaf(l) => ChildValue::Leaf(&l.value),
//         }
//     }

//     pub fn value_mut(&mut self) -> ChildValue<&mut BT, &mut LT> {
//         match self {
//             Child::Branch(b) => ChildValue::Brach(&mut b.value),
//             Child::Leaf(l) => ChildValue::Leaf(&mut l.value),
//         }
//     }

//     pub fn try_parent(&self) -> Result<Ref<Parent<RT, BT, LT>>, BorrowError> {
//         match self {
//             Child::Branch(b) => b
//                 .parent
//                 .upgrade()
//                 .expect("parent dropped before child")
//                 .try_borrow()
//                 .map_err(BorrowError::from),
//             Child::Leaf(_) => todo!(),
//         }
//     }
// }

// #[derive(Debug)]
// pub enum ChildValue<BT, LT> {
//     Brach(BT),
//     Leaf(LT),
// }

// // ///
// // /// Internal Node
// // ///
// // #[derive(Debug)]
// // enum Node<RT, BT, LT> {
// //     Root(Root<RT, BT, LT>),
// //     Branch(Branch<RT, BT, LT>),
// //     Leaf(Leaf<RT, BT, LT>),
// // }

// // impl<RT, BT, LT> Node<RT, BT, LT> {
// //     pub fn value(&self) -> Value<&RT, &BT, &LT> {
// //         match self {
// //             Node::Root(r) => Value::Root(&r.value),
// //             Node::Branch(b) => Value::Branch(&b.value),
// //             Node::Leaf(l) => Value::Leaf(&l.value),
// //         }
// //     }

// //     pub fn value_mut(&mut self) -> Value<&mut RT, &mut BT, &mut LT> {
// //         match self {
// //             Node::Root(r) => Value::Root(&mut r.value),
// //             Node::Branch(b) => Value::Branch(&mut b.value),
// //             Node::Leaf(l) => Value::Leaf(&mut l.value),
// //         }
// //     }
// // }

// // impl<RT, BT, LT> From<Parent<RT, BT, LT>> for Node<RT, BT, LT> {
// //     fn from(value: Parent<RT, BT, LT>) -> Self {
// //         match value {
// //             Parent::Root(r) => Node::Root(r),
// //             Parent::Branch(b) => Node::Branch(b),
// //         }
// //     }
// // }

// // impl<RT, BT, LT> From<Child<RT, BT, LT>> for Node<RT, BT, LT> {
// //     fn from(value: Child<RT, BT, LT>) -> Self {
// //         match value {
// //             Child::Branch(b) => Node::Branch(b),
// //             Child::Leaf(l) => Node::Leaf(l),
// //         }
// //     }
// // }
