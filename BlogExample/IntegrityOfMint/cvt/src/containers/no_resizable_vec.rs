extern crate alloc;
use alloc::alloc::{Layout, alloc, dealloc, handle_alloc_error};
use core::mem;
use core::ops::{Deref, DerefMut, Index, IndexMut, Range};
use core::ptr::{self, NonNull};
use std::cmp::Ordering;
use crate::{CVT_assume};

////////////////////////////////////////////////////////////////////////
// Adapted from SeaHorn.
//
// This is a more precise implementation of the Vector class than
// NoDataVec but the vector is not resizable so its capacity must be
// fixed a priori.
////////////////////////////////////////////////////////////////////////

trait IsZST { const IS_ZST: bool; }
impl<T> IsZST for T { const IS_ZST: bool = mem::size_of::<T>() == 0; }

macro_rules! assert {
        ($cond:expr) => {{ /*cvt::CVT_assert($cond)*/}};
}

struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}

impl<T> RawVec<T> {
    // We assume that there is no errors during allocation.
    fn new(capacity: usize) -> Self {
        if T::IS_ZST || capacity == 0 {
            return RawVec {
                ptr: NonNull::dangling(),
                cap: capacity,
            };
        }

        let layout: Layout = Layout::array::<T>(capacity).unwrap();
	    //CVT_assume(layout.size() <= isize::MAX as usize);

        let pointer: *mut u8 = unsafe { alloc(layout) };
        let pointer: NonNull<T> = match NonNull::new(pointer as *mut T) {
            Some(p) => p,
            None => {
                CVT_assume(false);
                handle_alloc_error(layout)
            },
        };

        RawVec {
            ptr: pointer,
            cap: capacity,
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 && !T::IS_ZST {
            unsafe {
                dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::array::<T>(self.cap).unwrap(),
                );
            }
        }
    }
}

pub struct NoResizableVec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> NoResizableVec<T> {
    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    pub fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn len(&self) -> usize { self.len }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn new(capacity: usize) -> Self {
        NoResizableVec {
            buf: RawVec::new(capacity),
            len: 0,
        }
    }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn push(&mut self, elem: T) {
        assert!(self.len < self.cap());
        unsafe { ptr::write(self.ptr().add(self.len), elem); }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
        }
    }

    pub fn find(& self, elem: &T) -> Option<usize>
        where  T: Ord, {
        let mut i = 0;
        while i < self.len {
            if self.index(i).cmp(elem) ==  Ordering::Equal {
                return Some(i)
            }
            i += 1;
        }
        return None
    }

    pub fn clone(&self) -> Self {
        let vec = RawVec::new(self.cap());
        let res = NoResizableVec {
            buf: vec,
            len: self.len,
        };
        unsafe {
            ptr::copy_nonoverlapping(
                self.ptr(),
                res.ptr() ,
                self.len
            )
        }
        res
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index < self.len);
        assert!(self.len < self.cap());
        unsafe {
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr().add(index), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );
            result
        }
    }

    pub fn drain(&mut self, range: Range<usize>) -> Drain<T> {
        unsafe {
            let slice: &[T] = &*self;
            let slice: &[T] = &slice[range.clone()];
            let iter: RawValIter<T> = RawValIter::new(slice);

            Drain {
                const_range: range.clone(),
                iter: iter,
                // vec: PhantomData,
                vec: self,
            }
        }
    }
    pub fn drop(&mut self) {
        // for i in 0..10 {
        //     if i > self.cap { break; }
        //     self.pop();
        // }
    }
}


impl<T> Drop for NoResizableVec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() { }
    }
}

impl<T> Deref for NoResizableVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T> DerefMut for NoResizableVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

impl<T> Index<usize> for NoResizableVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len());
        unsafe {
            if T::IS_ZST { NonNull::<T>::dangling().as_ref() }
            else { & *self.ptr().add(index) }
        }
    }
}

impl<T> IndexMut<usize> for NoResizableVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len());
        unsafe {
            if T::IS_ZST { NonNull::<T>::dangling().as_mut() }
            else { &mut *self.ptr().add(index) }
        }
    }
}

impl<T> IntoIterator for NoResizableVec<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    #[cfg_attr(feature = "certora-debug", inline(never))]
    fn into_iter(self) -> IntoIter<T> {
        unsafe {
            let iter: RawValIter<T> = RawValIter::new(&self);
            let buf: RawVec<T> = ptr::read(&self.buf);
            mem::forget(self);

            IntoIter {
                iter,
                _buf: buf,
            }
        }
    }
}

struct RawValIter<T> {
    start: *const T,
    end: *const T,
}

impl<T> RawValIter<T> {
    unsafe fn new(slice: &[T]) -> Self {
        RawValIter {
            start: slice.as_ptr(),
            end: if T::IS_ZST {
                ((slice.as_ptr() as usize) + slice.len()*mem::align_of::<T>()) as *const _
            } else if slice.len() == 0 {
                slice.as_ptr()
            } else {
                slice.as_ptr().add(slice.len())
            },
        }
    }
}

impl<T> Iterator for RawValIter<T> {
    type Item = T;

    #[cfg_attr(feature = "certora-debug", inline(never))]
    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if T::IS_ZST {
                    self.start = (self.start as usize + 1*mem::align_of::<T>()) as *const _;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    let old_ptr = self.start;
                    self.start = self.start.offset(1);
                    Some(ptr::read(old_ptr))
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // let elem_size: usize = mem::size_of::<T>();
        let len: usize = (self.end as usize - self.start as usize)
                  / if T::IS_ZST { 1*mem::align_of::<T>() } else { mem::size_of::<T>() };
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for RawValIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                if T::IS_ZST {
                    self.end = (self.end as usize - 1*mem::align_of::<T>()) as *const _;
                    Some(ptr::read(NonNull::<T>::dangling().as_ptr()))
                } else {
                    self.end = self.end.offset(-1);
                    Some(ptr::read(self.end))
                }
            }
        }
    }
}

pub struct IntoIter<T> {
    _buf: RawVec<T>,
    iter: RawValIter<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> Drop for IntoIter<T> {
    fn drop(&mut self) {
        for _ in &mut *self {}
    }
}

pub struct Drain<'a, T: 'a> {
    const_range: Range<usize>,
    vec: &'a mut NoResizableVec<T>,
    iter: RawValIter<T>,
}

impl<'a, T> Iterator for Drain<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a, T> DoubleEndedIterator for Drain<'a, T> {
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<'a, T> Drop for Drain<'a, T> {
    fn drop(&mut self) {
        for _ in &mut *self { }

        if T::IS_ZST || self.const_range.len() == 0 {
            self.vec.len -= self.const_range.len();
        } else {
            unsafe {
                ptr::copy_nonoverlapping(
                    self.vec.ptr().add(self.const_range.end),
                    self.vec.ptr().add(self.const_range.start),
                    self.vec.len() - self.const_range.end,
                );
            }
            self.vec.len -= self.const_range.len();
        }
    }
}

impl<T> Clone for NoResizableVec<T> {
    fn clone(&self) -> Self {
        NoResizableVec::clone(self)
    }
}

#[macro_export]
macro_rules! cvt_no_resizable_vec {
    // cvt_no_resizable_vec![1, 3, 2, 7]; => [1, 3, 2, 7], cap = 4*2 = 8
   ($($values:expr),+ $(,)?) => (
        {
            let ARG_COUNT: usize = 0 $(+ { _ = $values; 1 })*;
            let mut v = cvt::NoResizableVec::new(ARG_COUNT*2);
            $(v.push($values);)*
            v
        }
    );

     // cvt_no_resizable_vec!([1, 3, 4]; 12); => [1, 3, 4], cap = 12
    ([$($values:expr),* $(,)?]; $cap:expr) => {
        {
            let ARG_COUNT: usize = 0 $(+ { _ = $values; 1 })*;
            assert!(ARG_COUNT <= $cap);
            let mut v = cvt::NoResizableVec::new($cap);
            $(v.push($values);)*
            v
        }
    };
}



