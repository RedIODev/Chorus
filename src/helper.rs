// pub trait PrevPeekableAdapter:Sized + Iterator
// where Self: Sized + Iterator,
//     Self::Item: Clone {
//     fn prev_iter(self) -> PrevPeekable<Self> {
//         PrevPeekable::new(self)
//     }
// }

// impl<T: Iterator> PrevPeekableAdapter for T
// where T: Iterator,
//     T::Item: Clone {}

// pub struct CurrentIter<I>
// where I: Iterator,
// I::Item: Copy {
//     iter: I,
//     current: Option<I::Item>,
//     unstarted:bool
// }

// impl<I> Iterator for CurrentIter<I>
// where I: Iterator,
// I::Item: Copy {
//     type Item = I::Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.unstarted {
//             self.unstarted = false;
//         }
//         let next = self.iter.next();
//         if next.is_some() {
//             self.current = next.clone();
//         }
//         next
//     }
// }

// impl<I> CurrentIter<I>
// where I: Iterator,
// I::Item: Copy {
//     pub fn current(&self) -> Option<I::Item> {
//         self.current
//     }

use unicode_segmentation::UnicodeSegmentation;

//     pub fn unstarted(&self) -> bool {
//         self.unstarted
//     }
// }
pub struct PausableIter<I>
where
    I: Iterator,
{
    iter: I,
    paused_value: Option<I::Item>,
}

impl<I> Iterator for PausableIter<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.paused_value.is_some() {
            return self.paused_value.take();
        }
        self.iter.next()
    }
}

impl<I> PausableIter<I>
where
    I: Iterator,
{
    pub fn pause(&mut self, current_value: I::Item) {
        self.paused_value = Some(current_value)
    }
}

pub trait PausableIterAdapter: Iterator + Sized {
    fn pausable_iter(self) -> PausableIter<Self> {
        PausableIter {
            iter: self,
            paused_value: None,
        }
    }
}

impl<I> PausableIterAdapter for I where I: Iterator + Sized {}

pub trait Strlen {
    fn strlen(&self) -> usize;
}

impl Strlen for &str {
    fn strlen(&self) -> usize {
        self.graphemes(true).count()
    }
}

// pub mod tree {
//     use std::{
//         marker::PhantomData,
//         ops::{Deref, DerefMut},
//     };

//     use crate::error::BorrowError;

//     pub mod single {
//         pub type Root<T> = super::Root<T, T, T>;
//         pub type Branch<T> = super::Branch<T, T, T>;
//         pub type Leaf<T> = super::Leaf<T, T, T>;
//         pub type Container<T> = super::Container<T, T, T>;
//         pub type Contained<T> = super::Contained<T, T, T>;
//     }

//     pub struct Root<RT, BT, LT> {
//         children: Vec<Contained<RT, BT, LT>>,
//         data: RT,
//         borrow_state: BorrowState,
//     }

//     pub struct Branch<RT, BT, LT> {
//         parent: *mut Container<RT, BT, LT>,
//         children: Vec<Contained<RT, BT, LT>>,
//         data: BT,
//         borrow_state: BorrowState,
//     }

//     pub struct Leaf<RT, BT, LT> {
//         parent: *mut Container<RT, BT, LT>,
//         data: LT,
//     }

//     pub enum Container<RT, BT, LT> {
//         Root(Root<RT, BT, LT>),
//         Branch(Branch<RT, BT, LT>),
//     }

//     pub enum Contained<RT, BT, LT> {
//         Branch(Branch<RT, BT, LT>),
//         Leaf(Leaf<RT, BT, LT>),
//     }

//     pub struct Ref<'a, T>
//     where
//         T: SafeBorrow,
//     {
//         ptr: *mut T,
//         _phantom_data: PhantomData<&'a T>,
//     }

//     pub struct RefMut<'a, T>
//     where
//         T: SafeBorrow,
//     {
//         ptr: *mut T,
//         _phantom_data: PhantomData<&'a T>,
//     }

//     pub trait SafeBorrow {
//         fn borrow_state(&mut self) -> &mut BorrowState;
//     }

//     impl<'a, T> Ref<'a, T>
//     where
//         T: SafeBorrow,
//     {
//         fn new(ptr: *mut T) -> Self {
//             Ref::try_new(ptr).unwrap()
//         }

//         fn try_new(ptr: *mut T) -> Result<Self, BorrowError> {
//             let borrow_state = unsafe { (&mut *ptr).borrow_state() };
//             match borrow_state {
//                 BorrowState::NoBorrow => *borrow_state = BorrowState::Borrow(1),
//                 BorrowState::Borrow(i) => *i += 1,
//                 BorrowState::MutBorrow => return Err(BorrowError::InvalidImutableBorrow),
//             }

//             Ok(Ref {
//                 ptr,
//                 _phantom_data: PhantomData,
//             })
//         }
//     }

//     impl<'a, T> Deref for Ref<'a, T>
//     where
//         T: SafeBorrow,
//     {
//         type Target = T;

//         fn deref(&self) -> &Self::Target {
//             unsafe { &*self.ptr }
//         }
//     }

//     impl<'a, T> Drop for Ref<'a, T>
//     where
//         T: SafeBorrow,
//     {
//         fn drop(&mut self) {
//             let borrow_state = unsafe { (&mut *self.ptr).borrow_state() };
//             match borrow_state {
//                 BorrowState::NoBorrow => unreachable!(),
//                 BorrowState::Borrow(i) if *i > 1 => *i -= 1,
//                 BorrowState::Borrow(i) if *i == 1 => *borrow_state = BorrowState::NoBorrow,
//                 BorrowState::Borrow(i) if *i == 0 => unreachable!(),
//                 BorrowState::Borrow(_) => unreachable!(),
//                 BorrowState::MutBorrow => unreachable!(),
//             }
//         }
//     }

//     impl<'a, T> RefMut<'a, T>
//     where
//         T: SafeBorrow,
//     {
//         fn new(ptr: *mut T) -> Self {
//             RefMut::try_new(ptr).unwrap()
//         }

//         fn try_new(ptr: *mut T) -> Result<Self, BorrowError> {
//             let borrow_state = unsafe { (&mut *ptr).borrow_state() };
//             match borrow_state {
//                 BorrowState::NoBorrow => *borrow_state = BorrowState::MutBorrow,
//                 BorrowState::Borrow(_) => return Err(BorrowError::InvalidMutableBorrow),
//                 BorrowState::MutBorrow => return Err(BorrowError::InvalidSecondMutableBorrow),
//             }

//             Ok(RefMut {
//                 ptr,
//                 _phantom_data: PhantomData,
//             })
//         }
//     }

//     impl<'a, T> Deref for RefMut<'a, T>
//     where
//         T: SafeBorrow,
//     {
//         type Target = T;

//         fn deref(&self) -> &Self::Target {
//             unsafe { &*self.ptr }
//         }
//     }

//     impl<'a, T> DerefMut for RefMut<'a, T>
//     where
//         T: SafeBorrow,
//     {
//         fn deref_mut(&mut self) -> &mut Self::Target {
//             unsafe { &mut *self.ptr }
//         }
//     }

//     impl<'a, T> Drop for RefMut<'a, T>
//     where
//         T: SafeBorrow,
//     {
//         fn drop(&mut self) {
//             let borrow_state = unsafe { (&mut *self.ptr).borrow_state() };
//             match borrow_state {
//                 BorrowState::NoBorrow => unreachable!(),
//                 BorrowState::Borrow(_) => unreachable!(),
//                 BorrowState::MutBorrow => *borrow_state = BorrowState::NoBorrow,
//             }
//         }
//     }

//     impl<RT, BT, LT> From<Root<RT, BT, LT>> for Container<RT, BT, LT> {
//         fn from(value: Root<RT, BT, LT>) -> Self {
//             Container::Root(value)
//         }
//     }

//     impl<RT, BT, LT> From<Branch<RT, BT, LT>> for Container<RT, BT, LT> {
//         fn from(value: Branch<RT, BT, LT>) -> Self {
//             Container::Branch(value)
//         }
//     }

//     impl<RT, BT, LT> From<Branch<RT, BT, LT>> for Contained<RT, BT, LT> {
//         fn from(value: Branch<RT, BT, LT>) -> Self {
//             Contained::Branch(value)
//         }
//     }

//     impl<RT, BT, LT> From<Leaf<RT, BT, LT>> for Contained<RT, BT, LT> {
//         fn from(value: Leaf<RT, BT, LT>) -> Self {
//             Contained::Leaf(value)
//         }
//     }

//     impl<RT, BT, LT> SafeBorrow for Container<RT, BT, LT> {
//         fn borrow_state(&mut self) -> &mut BorrowState {
//             match self {
//                 Container::Root(r) => &mut r.borrow_state,
//                 Container::Branch(b) => &mut b.borrow_state,
//             }
//         }
//     }

//     impl<RT, BT, LT> Container<RT, BT, LT> {
//         pub fn add_branch(&mut self, data: BT) {
//             let branch = unsafe { Branch::new(self, data) };
//             self.children_mut().push(branch.into());
//         }

//         pub fn add_leaf(&mut self, data: LT) {
//             let leaf = unsafe { Leaf::new(self, data) };
//             self.children_mut().push(leaf.into());
//         }

//         pub fn children(&self) -> &Vec<Contained<RT, BT, LT>> {
//             match self {
//                 Container::Root(r) => &r.children,
//                 Container::Branch(b) => &b.children,
//             }
//         }

//         pub fn children_mut(&mut self) -> &mut Vec<Contained<RT, BT, LT>> {
//             match self {
//                 Container::Root(r) => &mut r.children,
//                 Container::Branch(b) => &mut b.children,
//             }
//         }
//     }

//     impl<RT, BT, LT> Contained<RT, BT, LT> {
//         pub fn parent(&self) -> Ref<'_, Container<RT, BT, LT>> {
//             //unsafe: unchecked multiple borrow from children
//             let parent = match self {
//                 Contained::Branch(b) => b.parent,
//                 Contained::Leaf(l) => l.parent,
//             };
//             Ref::new(parent)
//         }

//         pub fn parent_mut(&mut self) -> RefMut<'_, Container<RT, BT, LT>> {
//             let parent = match self {
//                 Contained::Branch(b) => b.parent,
//                 Contained::Leaf(l) => l.parent,
//             };
//             RefMut::new(parent)
//         }

//         pub fn upgrade(&mut self)
//         where
//             LT: Into<BT>,
//         {
//             let Contained::Leaf(leaf) = self else {
//                 return;
//             };
//             let mut branch = unsafe {
//                 let mut leaf_data = std::mem::zeroed();
//                 std::mem::swap(&mut leaf_data, &mut leaf.data);
//                 Branch::new(leaf.parent, leaf_data.into()).into()
//             };
//             std::mem::swap(&mut branch, self);
//         }
//     }

//     impl<RT, BT, LT> Root<RT, BT, LT> {
//         pub fn new(data: RT) -> Self {
//             Self {
//                 children: Vec::new(),
//                 data,
//                 borrow_state: BorrowState::NoBorrow,
//             }
//         }

//         pub fn downgrade(self, parent: &mut Container<RT, BT, LT>)
//         where
//             RT: Into<BT>,
//         {
//             if self.borrow_state != BorrowState::NoBorrow {
//                 panic!("cannot downgrade a borrowed root")
//             }
//             let branch = Branch {
//                 parent,
//                 children: self.children,
//                 data: self.data.into(),
//                 borrow_state: BorrowState::NoBorrow,
//             };
//             parent.children_mut().push(branch.into())
//         }
//     }

//     impl<RT, BT, LT> Branch<RT, BT, LT> {
//         pub unsafe fn new(parent: *mut Container<RT, BT, LT>, data: BT) -> Self {
//             Self {
//                 parent,
//                 children: Vec::new(),
//                 data,
//                 borrow_state: BorrowState::NoBorrow,
//             }
//         }
//     }

//     impl<RT, BT, LT> Leaf<RT, BT, LT> {
//         pub unsafe fn new(parent: *mut Container<RT, BT, LT>, data: LT) -> Self {
//             Self { parent, data }
//         }
//     }
// }

// pub mod parent_container {

// }
