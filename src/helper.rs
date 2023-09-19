

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
where I: Iterator {
    iter:I,
    paused_value:Option<I::Item>
}

impl<I> Iterator for PausableIter<I>
where I: Iterator {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.paused_value.is_some() {
            return self.paused_value.take();
        }
        self.iter.next()
    }
}

impl<I> PausableIter<I>
where I:Iterator {
    pub fn pause(&mut self, current_value:I::Item) {
        self.paused_value = Some(current_value)
    }
}

pub trait PausableIterAdapter: Iterator + Sized {
    
    fn pausable_iter(self) -> PausableIter<Self> {
        PausableIter { iter:self, paused_value: None }
    }
}

impl<I> PausableIterAdapter for I
where I:Iterator + Sized {
    
}

pub trait Strlen {
    fn strlen(&self) -> usize;
}

impl Strlen for &str {
    fn strlen(&self) -> usize {
        self.graphemes(true).count()
    }
}