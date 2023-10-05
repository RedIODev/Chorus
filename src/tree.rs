use std::{marker::PhantomData, ptr::NonNull, mem::MaybeUninit, ops::{IndexMut, Index}};

pub struct Tree<RT,BT,LT> {
    root: RootBox<RT,BT,LT>
}

pub struct Root<RT, BT, LT> {
    children: ContainedVec<RT, BT, LT>,
    data: RT,
    borrow_state: BorrowState,
}

pub struct Branch<RT, BT, LT> {
    parent: ContainerPtr<RT, BT, LT>,
    children: ContainedVec<RT, BT, LT>,
    data: BT,
    borrow_state: BorrowState,
}

pub struct Leaf<RT, BT, LT> {
    parent: ContainerPtr<RT, BT, LT>,
    data: LT,
}

pub enum Data<RT,BT,LT> {
    Root(RT),
    Branch(BT),
    Leaf(LT)
}

pub enum Container<RT, BT, LT> {
    Root(Root<RT, BT, LT>),
    Branch(Branch<RT, BT, LT>),
}

pub enum Contained<RT, BT, LT> {
    Branch(Branch<RT, BT, LT>),
    Leaf(Leaf<RT, BT, LT>),
}

pub struct Ref<'a, RT,BT,LT> {
    ptr: NonNull<Container<RT,BT,LT>>,
    _phantom_data: PhantomData<&'a ()>,
}

pub struct RefMut<'a, RT,BT,LT> {
    ptr: NonNull<Container<RT,BT,LT>>,
    _phantom_data: PhantomData<&'a ()>,
}

pub struct ContainedVec<RT,BT,LT> {
    data:HeapArray<Contained<RT,BT,LT>>,
    lenght:usize
}

pub struct RootBox<RT,BT,LT> {
    data:Box<Root<RT,BT,LT>>
}

pub struct ContainerPtr<RT,BT,LT> {
    ptr:NonNull<Container<RT,BT,LT>>,
}

#[derive(Debug)]
pub enum BorrowState {
    NoBorrow,
    Borrow(usize),
    MutBorrow,
}

struct HeapArray<T> {
    memory:NonNull<T>,
    lenght:usize
}

/// 
/// 
/// Impls
/// 
/// 

impl<T> HeapArray<T> {
    pub fn empty() -> Self {
        HeapArray { memory: NonNull::dangling(), lenght: 0 }
    }

    pub fn new(lenght: usize) -> Self {
        use std::alloc::*;
        let layout = Layout::array::<T>(lenght).unwrap();
        let ptr = unsafe {alloc(layout)};
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        let memory = NonNull::new(ptr).unwrap().cast();
        HeapArray { memory , lenght }
    }

    pub fn resize(&mut self, new_lenght:usize) {
        use std::alloc::*;
        let new_layout = Layout::array::<T>(new_lenght).unwrap();
        let old_layout = Layout::array::<T>(self.lenght).unwrap();
        let ptr = self.memory.cast().as_ptr();
        let new_ptr = unsafe { realloc(ptr, old_layout, new_layout.size())};  
        if new_ptr.is_null() {
            handle_alloc_error(new_layout);
        }
        let new_memory = NonNull::new(ptr).unwrap().cast();
        self.memory = new_memory;
        self.lenght = new_lenght;
    }
}

impl<T> Drop for HeapArray<T> {
    fn drop(&mut self) {
        if self.lenght == 0 {
            return;
        }
        use std::alloc::*;
        let layout = Layout::array::<T>(self.lenght).unwrap();
        let ptr = self.memory.cast().as_ptr();
        unsafe { dealloc(ptr, layout) }
    }
}

impl<T> Index<usize> for HeapArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.lenght {
            panic!("index {} out of bounds", index)
        }

        unsafe { &*self.memory.as_ptr().add(index) }
    }
}

impl<T> IndexMut<usize> for HeapArray<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.lenght {
            panic!("index {} out of bounds", index)
        }

        unsafe { &mut *self.memory.as_ptr().add(index) }
    }
}

impl<RT,BT,LT> ContainedVec<RT,BT,LT> {

    pub fn new() -> Self {
        Self { data: HeapArray::empty(), lenght:0 }
    }   

    pub fn add() {
        
    }
}

//borrow state doesnt work rethink location for borrow state info.