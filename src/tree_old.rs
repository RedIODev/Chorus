// use std::{
//     cell::RefCell,
//     marker::PhantomData,
//     ptr::NonNull,
//     slice::{Iter, IterMut},
// };

// use crate::error::BorrowError;

// #[derive(Debug)]
// pub struct Tree<RT, BT, LT> {
//     root: Option<TreeBox<RT, BT, LT>>,
// }

// impl<RT, BT, LT> Tree<RT, BT, LT> {
//     pub fn new() -> Self {
//         Self { root: None }
//     }

//     pub fn set_root(&mut self, root_data: RT) -> &mut Root<RT, BT, LT> {
//         self.root = Some(TreeBox::new(root_data));
//         match self.root {
//             Some(ref mut t) => &mut t.boks,
//             None => unreachable!(),
//         }
//     }

//     pub fn get_root(&self) -> Option<&Root<RT, BT, LT>> {
//         self.root.as_ref().map(|r| &*r.boks)
//     }

//     pub fn get_root_mut(&mut self) -> Option<&mut Root<RT, BT, LT>> {
//         self.root.as_mut().map(|r| &mut *r.boks)
//     }
// }

// #[derive(Debug)]
// pub struct Root<RT, BT, LT> {
//     children: TreeVec<RT, BT, LT>,
//     data: RT,
//     borrow_state: BorrowState,
// }

// impl<RT, BT, LT> Root<RT, BT, LT> {
//     pub fn new(data: RT) -> Self {
//         Self {
//             children: TreeVec::new(),
//             data,
//             borrow_state: BorrowState::NoBorrow,
//         }
//     }

//     pub fn data(&self) -> &RT {
//         &self.data
//     }

//     pub fn data_mut(&mut self) -> &mut RT {
//         &mut self.data
//     }

//     pub fn borrow_state(&self) -> &BorrowState {
//         &self.borrow_state
//     }

//     pub fn children(&self) -> &[Contained<RT, BT, LT>] {
//         self.children.as_slice()
//     }

//     pub fn children_mut(&mut self) -> &mut [Contained<RT, BT, LT>] {
//         self.children.as_mut_slice()
//     }
// }

// #[derive(Debug)]
// pub struct Branch<RT, BT, LT> {
//     parent: ContainerPtr<RT, BT, LT>,
//     children: TreeVec<RT, BT, LT>,
//     data: BT,

// }

// impl<RT, BT, LT> Branch<RT, BT, LT> {
//     pub fn new(parent: ContainerPtr<RT, BT, LT>, data: BT) -> Self {
//         Self {
//             parent,
//             children: TreeVec::new(),
//             data,
//         }
//     }

//     pub fn update_parent_ptr(&mut self) {
//         let self_ptr = unsafe { ContainerPtr::new_unchecked_branch(self) };
//         for child in self.children.iter_mut() {
//             match child {
//                 Contained::Branch(b) => b.parent = self_ptr,
//                 Contained::Leaf(l) => l.parent = self_ptr,
//             }
//         }
//     }

//     pub fn data(&self) -> &BT {
//         &self.data
//     }

//     pub fn data_mut(&mut self) -> &mut BT {
//         &mut self.data
//     }

// }

// #[derive(Debug)]
// pub struct Leaf<RT, BT, LT> {
//     parent: ContainerPtr<RT, BT, LT>,
//     data: LT,
// }

// impl<RT, BT, LT> Leaf<RT, BT, LT> {
//     pub fn new(parent: ContainerPtr<RT, BT, LT>, data: LT) -> Self {
//         Self { parent, data }
//     }
// }

// #[derive(Debug)]
// pub enum Data<RT, BT, LT> {
//     Root(RT),
//     Branch(BT),
//     Leaf(LT),
// }

// pub struct DataIter<RT, BT, LT> {
//     ptr: ContainerPtr<RT, BT, LT>,
// }

// #[derive(Debug)]
// pub enum Contained<RT, BT, LT> {
//     Branch(Branch<RT, BT, LT>),
//     Leaf(Leaf<RT, BT, LT>),
// }

// impl<RT, BT, LT> Contained<RT, BT, LT> {
//     pub fn update_parent_ptr(&mut self) {
//         let Contained::Branch(parent) = self else {
//             return;
//         };
//         parent.update_parent_ptr();
//     }
// }

// impl<RT, BT, LT> From<Leaf<RT, BT, LT>> for Contained<RT, BT, LT> {
//     fn from(value: Leaf<RT, BT, LT>) -> Self {
//         Contained::Leaf(value)
//     }
// }

// impl<RT, BT, LT> From<Branch<RT, BT, LT>> for Contained<RT, BT, LT> {
//     fn from(value: Branch<RT, BT, LT>) -> Self {
//         Contained::Branch(value)
//     }
// }

// #[derive(Debug)]
// pub struct Ref<'a, RT, BT, LT> {
//     ptr: ContainerPtr<RT, BT, LT>,
//     _phantom_data: PhantomData<&'a ()>,
// }

// #[derive(Debug)]
// pub struct RefMut<'a, RT, BT, LT> {
//     ptr: ContainerPtr<RT, BT, LT>,
//     _phantom_data: PhantomData<&'a ()>,
// }

// #[derive(Debug)]
// pub enum ContainerPtr<RT, BT, LT> {
//     Root(NonNull<Root<RT, BT, LT>>),
//     Branch(NonNull<Branch<RT, BT, LT>>),
// }

// impl<RT, BT, LT> ContainerPtr<RT, BT, LT> {
//     pub unsafe fn new_unchecked_branch(ptr: *mut Branch<RT, BT, LT>) -> Self {
//         Self::Branch(NonNull::new_unchecked(ptr))
//     }

//     pub unsafe fn new_unchecked_root(ptr: *mut Root<RT, BT, LT>) -> Self {
//         Self::Root(NonNull::new_unchecked(ptr))
//     }
// }

// impl<RT, BT, LT> Clone for ContainerPtr<RT, BT, LT> {
//     fn clone(&self) -> Self {
//         *self
//     }
// }

// impl<RT, BT, LT> Copy for ContainerPtr<RT, BT, LT> {}

// #[derive(Debug)]
// pub enum BorrowState {
//     NoBorrow,
//     Borrow(usize),
//     MutBorrow,
// }

// #[derive(Debug)]
// struct TreeVec<RT, BT, LT> {
//     vec: Vec<Contained<RT, BT, LT>>,
// }

// impl<RT, BT, LT> TreeVec<RT, BT, LT> {
//     pub fn new() -> Self {
//         Self { vec: Vec::new() }
//     }

//     pub fn add_leaf(
//         &mut self,
//         parent: ContainerPtr<RT, BT, LT>,
//         leaf_data: LT,
//     ) -> &mut Leaf<RT, BT, LT> {
//         let prev_cap = self.vec.capacity();
//         self.vec.push(Leaf::new(parent, leaf_data).into());
//         if prev_cap != self.vec.capacity() {
//             for contained in self.vec.iter_mut() {
//                 contained.update_parent_ptr()
//             }
//         }
//         match self.vec.last_mut() {
//             Some(Contained::Leaf(l)) => l,
//             _ => unreachable!(),
//         }
//     }

//     pub fn add_branch(
//         &mut self,
//         parent: ContainerPtr<RT, BT, LT>,
//         branch_data: BT,
//     ) -> &mut Branch<RT, BT, LT> {
//         let prev_cap = self.vec.capacity();
//         self.vec.push(Branch::new(parent, branch_data).into());
//         if prev_cap != self.vec.capacity() {
//             for contained in self.vec.iter_mut() {
//                 contained.update_parent_ptr()
//             }
//         }
//         match self.vec.last_mut() {
//             Some(Contained::Branch(b)) => b,
//             _ => unreachable!(),
//         }
//     }

//     pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Contained<RT, BT, LT>> {
//         self.into_iter()
//     }

//     pub fn iter(&self) -> impl Iterator<Item = &Contained<RT, BT, LT>> {
//         self.into_iter()
//     }

//     pub fn as_slice(&self) -> &[Contained<RT, BT, LT>] {
//         &self.vec
//     }

//     pub fn as_mut_slice(&mut self) -> &mut [Contained<RT, BT, LT>] {
//         &mut self.vec
//     }
// }

// impl<'a, RT, BT, LT> IntoIterator for &'a mut TreeVec<RT, BT, LT> {
//     type Item = &'a mut Contained<RT, BT, LT>;

//     type IntoIter = IterMut<'a, Contained<RT, BT, LT>>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.vec.iter_mut()
//     }
// }

// impl<'a, RT, BT, LT> IntoIterator for &'a TreeVec<RT, BT, LT> {
//     type Item = &'a Contained<RT, BT, LT>;

//     type IntoIter = Iter<'a, Contained<RT, BT, LT>>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.vec.iter()
//     }
// }

// #[derive(Debug)]
// struct TreeBox<RT, BT, LT> {
//     boks: Box<Root<RT, BT, LT>>,
// }

// impl<RT, BT, LT> TreeBox<RT, BT, LT> {
//     pub fn new(root_data: RT) -> Self {
//         Self {
//             boks: Box::new(Root::new(root_data)),
//         }
//     }

//     pub fn downgrade(self, parent: &mut Root<RT, BT, LT>)
//     where
//         BT: From<RT>,
//     {
//         let parent_ptr = unsafe { ContainerPtr::new_unchecked_root(parent) };
//         let branch = parent
//             .children
//             .add_branch(parent_ptr, self.boks.data.into());
//         branch.children = self.boks.children;
//         branch.update_parent_ptr()
//     }
// }
