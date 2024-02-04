use std::marker::PhantomData;
use std::ops::Index;
use {
    crate::{
        nondet::{
            Nondet, nondet, nondet_pointer
        },
    }
};
use crate::{CVT_assume};

////////////////////////////////////////////////////////////////////////
// Adapted from Kani.

// NoDataVec implements an abstraction of the Vec library which tracks
// only the length of the vector. It does not contain a backing store
// which implies that writes only increment the length and all reads
// return a non-deterministic value.
////////////////////////////////////////////////////////////////////////

const DEFAULT_CAPACITY: usize = 1073741824;
const MAX_MALLOC_SIZE: usize = 18014398509481984;

macro_rules! assert {
    ($cond:expr) => {{ /*cvt::CVT_assert($cond)*/}};
}

// We have observed that resizing might cause collapses in the pointer analysis.
// So we have this macro to enable/disable resizing.
macro_rules! allow_resizing {
    () => { false }
}


// The NoDataVec structure here models the length and the capacity.
pub struct NoDataVec<T> {
    len: usize,
    capacity: usize,
    // We use a _marker variable since we want the Vector to be generic over type
    // T. It is a zero-sized type which is used to mark things such that they act
    // like they own a T.
    _marker: PhantomData<T>,
}

impl<T: Nondet> NoDataVec<T> {
    // The standard library NoDataVec implementation calls reserve() to reserve
    // space for an additional element -> self.reserve(1). However, the
    // semantics of reserve() are ambiguous. reserve(num) allocates space for
    // "at least num more elements of the containing type". The operation can
    // be found in function `grow_amortized()` in raw_vec.rs in the standard
    // library. The logic for choosing a new value is:
    // self.cap = max(self.cap * 2, self.len + additional)
    // We try to implement similar semantics here.
    #[cfg_attr(feature = "certora-debug", inline(never))]
    fn grow(&mut self, additional: usize) {
        let new_len = self.len + additional;
        let grow_cap = self.capacity * 2;
        let new_capacity = if new_len > grow_cap { new_len } else { grow_cap };

        if new_capacity > MAX_MALLOC_SIZE {
            panic!("Malloc failed to allocate enough memory");
        }

        self.capacity = new_capacity;
    }
}

impl<T: Nondet> NoDataVec<T> {

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn new() -> NoDataVec<T> {
        // By default, we create a vector with a high default capacity. An
        // important callout to make here is that it prevents us from discovering
        // buffer-overflow bugs since we will (most-likely) always have enough
        // space allocated additional to the required vec capacity.
        // NOTE: This is however not a concern for this abstraction.
        NoDataVec { len: 0, capacity: DEFAULT_CAPACITY, _marker: Default::default() }
    }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn with_len(len: usize) -> Self {
        NoDataVec { len, capacity: DEFAULT_CAPACITY, _marker: Default::default()}
    }

    // Even though we dont model any memory, we can soundly model the capacity
    // of the allocation.
    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn with_capacity(capacity: usize) -> Self {
        NoDataVec { len: 0, capacity, _marker: Default::default() }
    }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn clone(&self) -> Self {
        NoDataVec { len: self.len, capacity: self.capacity, _marker: Default::default()}
    }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn iter(&self) -> NoDataIter<T> {
        NoDataIter::new(self.len)
    }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn is_empty(&mut self) -> bool {
        self.len == 0
    }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn push(&mut self, _elem: T) {

        if allow_resizing!() {
            // Please refer to grow() for better understanding the semantics of reserve().
            if self.capacity == self.len {
                self.reserve(1);
            }
        }

        assert!(self.capacity >= self.len);
        // We only increment the length of the vector disregarding the actual
        // element added to the Vector.
        self.len += 1;
    }

    // We check if there are any elements in the Vector. If not, we return a None
    // otherwise we return a nondeterministic value since we dont track any concrete
    // values in the Vector.
    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
	    Some(nondet::<T>())
        }
    }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn append(&mut self, other: &mut NoDataVec<T>) {
        let new_len = self.len + other.len;

        if allow_resizing!() {
            // Please refer to grow() for better understanding the semantics of grow().
            if self.capacity < new_len {
                self.reserve(other.len);
            }
        }

        assert!(self.capacity >= new_len);
        // Drop all writes, increment the length of the Vector with the size
        // of the Vector which is appended.
        self.len = new_len;
    }

    // At whichever position we insert the new element into, the overall effect on
    // the abstraction is that the length increases by 1.
    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn insert(&mut self, index: usize, _elem: T) {
        assert!(index <= self.len);

        self.len += 1;
    }

    // We only care that the index we are removing from lies somewhere as part of
    // the length of the Vector. The only effect on the abstraction is that the
    // length decreases by 1. In the case that it is a valid removal, we return a
    // nondeterministic value.
    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);

        self.len -= 1;
	    nondet::<T>()
    }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn extend<I: Iterator>(&mut self, iter: I)
    where
        I: Iterator<Item = T>,
    {
        // We first compute the length of the iterator.
        let mut iter_len = 0;
        for _value in iter {
            iter_len += 1;
        }

        // Please refer to grow() for better understanding the semantics of grow().
        self.reserve(iter_len);
        self.len += iter_len;
    }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn len(&self) -> usize {
        self.len
    }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // Please refer to grow() for better understanding the semantics of reserve().
    pub fn reserve(&mut self, additional: usize) {
        self.grow(additional);
    }

    /// These methods are from Deref<Target=[T]>
    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn binary_search_by_key<'a, B, F>(&'a self, _b: &B, mut _f: F) -> Result<usize, usize>
        where
            F: FnMut(&'a T) -> B,
            B: Ord,
    {
        let index: usize = nondet();
        CVT_assume(index < self.len);
        let tmp: i8 = nondet();
        if tmp > 0 {
            Ok(index)
        } else {
            Err(index)
        }
    }

    pub fn sort_by_key<K, F>(&mut self, f: F) where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        // do nothing
    }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn get(&self, index: usize) -> Option<&T> {
        assert!(index < self.len);
        // It's not ideal to allocate a pointer here but don't know a better solution
        // except returning a C pointer. The problem with returning a C pointer is that
        // we need to declare all the C functions at front.
        let ptr: *mut T = nondet_pointer::<T>();
        unsafe {
            let ref_ptr = &*ptr;
            Some(ref_ptr)
        }
    }
}

impl<T: Nondet> Index<usize> for NoDataVec<T> {
    type Output = T;
    #[cfg_attr(feature = "certora-debug", inline(never))]
    fn index(&self, index: usize) -> &Self::Output {
        let val = NoDataVec::get(self, index);
        match val {
            Some(v) => v,
            None => panic!(),
        }
    }
}


// NoDataIter is a structure which implements Iterator suitable for NoDataVec. We
// only track the index values to the start and end of the iterator.
pub struct NoDataIter<T> {
    start: usize,
    end: usize,
    // Please refer to the NoDataVec definition to understand why PhantomData is used
    // here.
    _marker: PhantomData<T>,
}

impl<T> NoDataIter<T> {
    #[cfg_attr(feature = "certora-debug", inline(never))]
    pub fn new(len: usize) -> Self {
        // By default, initialize the start to index 0 and end to the last index
        // of the Vector.
        NoDataIter { start: 0, end: len, _marker: Default::default() }
    }
}

impl<T: Nondet> Iterator for NoDataIter<T> {
    type Item = T;

    // Unless we are at the end of the array, return a nondeterministic value
    // wrapped around a Some.
    #[cfg_attr(feature = "certora-debug", inline(never))]
    fn next(&mut self) -> Option<T> {
        let res = if self.start == self.end { None } else { Some(nondet::<T>())};
        self.start = self.start + 1;
        return res
    }

    #[cfg_attr(feature = "certora-debug", inline(never))]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end - self.start;
        (len, Some(len))
    }
}

impl<T: Nondet> IntoIterator for NoDataVec<T> {
    type Item = T;
    type IntoIter = NoDataIter<T>;

    #[cfg_attr(feature = "certora-debug", inline(never))]
    fn into_iter(self) -> NoDataIter<T> {
        NoDataIter::new(self.len())
    }
}

impl<T: Nondet> Clone for NoDataVec<T> {
    fn clone(&self) -> Self {
        NoDataVec::clone(self)
    }
}

impl<T: Nondet> Nondet for NoDataVec<T> {
    #[cfg_attr(feature = "certora-debug", inline(never))]
    fn nondet() -> Self {
        NoDataVec::with_len(nondet())
    }
}

// Here, we define the cvt_no_data_vec! macro which behaves similar to the vec! macro
// found in the std prelude. If we try to override the vec! macro, we get error:
//
//     = note: `vec` could refer to a macro from prelude
//     note: `vec` could also refer to the macro defined here
//
// Relevant Zulip stream:
// https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/Override.20prelude.20macro
//
// The workaround for now is to define a new macro. cvt_no_data_vec! will initialize a new
// Vec based on its definition in this file. We support two types of initialization
// expressions:
//
// [ elem; count] -  initialize a Vector with element value `elem` occurring count times.
// [ elem1, elem2, ...] - initialize a Vector with elements elem1, elem2...
#[macro_export]
macro_rules! cvt_no_data_vec {
  ( $val:expr ; $count:expr ) =>
    ({
      let mut result = NoDataVec::new();
      let mut i: usize = 0;
      while i < $count {
        result.push($val);
        i += 1;
      }
      result
    });
  ( $( $xs:expr ),* ) => {
    {
      let mut result = NoDataVec::new();
      $(
        result.push($xs);
      )*
      result
    }
  };
}

