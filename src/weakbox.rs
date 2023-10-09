use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
    ptr::NonNull, sync::atomic::{AtomicUsize, Ordering},
};

use crate::error::BorrowError;

#[derive(Debug)]
pub struct Ref<'a, T> {
    ptr: NonNull<T>,
    state: *mut BorrowState,
    _pd: PhantomData<&'a ()>,
}

impl<'a, T> Ref<'a, T> {
    unsafe fn new(ptr: NonNull<T>, state: *mut BorrowState) -> Result<Self, BorrowError> {
        state.as_mut().unwrap().try_inc_ref_count()?;
        Ok(Self {
            ptr,
            state,
            _pd: PhantomData,
        })
    }
}

impl<'a, T> Drop for Ref<'a, T> {
    fn drop(&mut self) {
        unsafe { self.state.as_mut().unwrap().try_dec_ref_count().unwrap() }
    }
}

impl<'a, T> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

#[derive(Debug)]
pub struct RefMut<'a, T> {
    ptr: NonNull<T>,
    state: *mut BorrowState,
    _pd: PhantomData<&'a ()>,
}

impl<'a, T> RefMut<'a, T> {
    unsafe fn new(ptr: NonNull<T>, state: *mut BorrowState) -> Result<Self, BorrowError> {
        state.as_mut().unwrap().try_inc_mut_ref_count()?;
        Ok(Self {
            ptr,
            state,
            _pd: PhantomData,
        })
    }
}

impl<'a, T> Drop for RefMut<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.state
                .as_mut()
                .unwrap()
                .try_dec_mut_ref_count()
                .unwrap()
        }
    }
}

impl<'a, T> Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<'a, T> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

#[derive(Debug, Default)]
enum BorrowState {
    #[default]
    None,
    Borrow(usize),
    Mut,
}

impl BorrowState {
    fn try_inc_ref_count(&mut self) -> Result<(), BorrowError> {
        match self {
            BorrowState::None => *self = BorrowState::Borrow(1),
            BorrowState::Borrow(i) => *i += 1,
            BorrowState::Mut => Err(BorrowError::InvalidImutableBorrow)?,
        }
        Ok(())
    }

    fn try_dec_ref_count(&mut self) -> Result<(), BorrowError> {
        match self {
            BorrowState::None => Err(BorrowError::InvalidUnborrow)?,
            BorrowState::Borrow(i) if *i == 1 => *self = BorrowState::None,
            BorrowState::Borrow(i) => *i -= 1,
            BorrowState::Mut => Err(BorrowError::InvalidMutableUnborrow)?,
        }

        Ok(())
    }

    fn try_inc_mut_ref_count(&mut self) -> Result<(), BorrowError> {
        match self {
            BorrowState::None => *self = BorrowState::Mut,
            BorrowState::Borrow(_) => Err(BorrowError::InvalidMutableBorrow)?,
            BorrowState::Mut => Err(BorrowError::InvalidSecondMutableBorrow)?,
        }

        Ok(())
    }

    fn try_dec_mut_ref_count(&mut self) -> Result<(), BorrowError> {
        match self {
            BorrowState::None => Err(BorrowError::InvalidMutableUnborrow)?,
            BorrowState::Borrow(_) => Err(BorrowError::InvalidMutableUnborrow)?,
            BorrowState::Mut => *self = BorrowState::None,
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct RcBox<T> {
    manager: NonNull<StrongWeakManager<T>>,
}

//TODO: Replace Ref and Ref Mut with Weak that itself can give out Ref/RefMut

impl<T> RcBox<T> {
    pub fn new(data: T) -> Self {
        Self {
            manager: StrongWeakManager::create(data),
        }
    }

    pub fn try_borrow(&self) -> Result<Ref<T>, BorrowError> {
        unsafe {
            let manager = self.manager.as_ptr();
            let Some(data) = &(*manager).data else {
                unreachable!()
            };
            Ref::new(NonNull::from(&**data), &mut (*manager).state)
        }
    }

    pub fn try_borrow_mut(&self) -> Result<RefMut<T>, BorrowError> {
        unsafe {
            let manager = self.manager.as_ptr();
            let Some(data) = &(*manager).data else {
                unreachable!()
            };
            RefMut::new(NonNull::from(&**data), &mut (*manager).state)
        }
    }

    pub fn weak(&self) -> Weak<T> {
        unsafe { StrongWeakManager::inc_weak(self.manager) }
        Weak {
            manager: self.manager,
        }
    }
}

impl<T> Drop for RcBox<T> {
    fn drop(&mut self) {
        unsafe { StrongWeakManager::destroy_box(self.manager) }
    }
}

#[derive(Debug)]
pub struct Weak<T> {
    manager: NonNull<StrongWeakManager<T>>,
}

impl<T> Weak<T> {
    pub fn try_borrow(&self) -> Result<Option<Ref<T>>, BorrowError> {
        unsafe {
            let manager = self.manager.as_ptr();
            let Some(data) = &(*manager).data else {
                return Ok(None);
            };
            Ok(Some(Ref::new(
                NonNull::from(&**data),
                &mut (*manager).state,
            )?))
        }
    }

    pub fn try_borrow_mut(&self) -> Result<Option<RefMut<T>>, BorrowError> {
        unsafe {
            let manager = self.manager.as_ptr();
            let Some(data) = &(*manager).data else {
                return Ok(None);
            };
            Ok(Some(RefMut::new(
                NonNull::from(&**data),
                &mut (*manager).state,
            )?))
        }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        unsafe { StrongWeakManager::dec_weak(self.manager) }
    }
}

struct StrongWeakManager<T> {
    data: Option<Box<T>>,
    state: BorrowState,
    weak_count: AtomicUsize,
}

impl<T> StrongWeakManager<T> {
    pub fn create(data: T) -> NonNull<Self> {
        let ptr = Box::leak(Box::new(Self {
            data: Some(Box::new(data)),
            state: BorrowState::default(),
            weak_count: AtomicUsize::new(0),
        }));
        NonNull::from(ptr)
    }

    pub unsafe fn destroy_box(mut ptr: NonNull<Self>) {
        ptr.as_mut().data = None;
        if ptr.as_ref().weak_count.load(Ordering::Acquire) == 0 {
            StrongWeakManager::drop_manager(ptr)
        }
    }

    pub unsafe fn inc_weak(mut ptr: NonNull<Self>) {
        let count = ptr.as_ref().weak_count.fetch_add(1, Ordering::SeqCst);
        ptr.as_mut().weak_count.store(count, Ordering::Release);
    }

    pub unsafe fn dec_weak(mut ptr: NonNull<Self>) {
        let count = ptr.as_ref().weak_count.fetch_sub(1, Ordering::SeqCst);
        ptr.as_mut().weak_count.store(count, Ordering::Release);
        if ptr.as_ref().weak_count.load(Ordering::Acquire) == 0 && ptr.as_ref().data.is_none() {
            StrongWeakManager::drop_manager(ptr)
        }
    }

    unsafe fn drop_manager(ptr: NonNull<Self>) {
        std::mem::drop(Box::from_raw(ptr.as_ptr()))
    }
}

// #[derive(Debug)]
// struct WeakVec<T> {
//     data: Vec<T>,
//     state: RefCell<BorrowState>,
// }

// impl<T> WeakVec<T> {
//     pub fn new() -> Self {
//         Self {
//             data: Vec::new(),
//             state: RefCell::new(BorrowState::default()),
//         }
//     }

//     pub fn try_borrow(&self, index: usize) -> Result<Ref<T>, BorrowError> {
//         unsafe { Ref::new(NonNull::from(&self.data[index]), self.state.as_ptr()) }
//     }

//     pub fn try_borrow_mut(&mut self, index: usize) -> Result<MutRef<T>, BorrowError> {
//         unsafe { MutRef::new(NonNull::from(&self.data[index]), self.state.as_ptr()) }
//     }
// }
