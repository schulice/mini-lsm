#![allow(unused_variables)] // TODO(you): remove this lint after implementing this mod
#![allow(dead_code)] // TODO(you): remove this lint after implementing this mod

use std::cmp::{self};
use std::collections::binary_heap::PeekMut;
use std::collections::BinaryHeap;

use anyhow::{Error, Ok, Result};

use crate::key::KeySlice;

use super::StorageIterator;

struct HeapWrapper<I: StorageIterator>(pub usize, pub Box<I>);

impl<I: StorageIterator> PartialEq for HeapWrapper<I> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == cmp::Ordering::Equal
    }
}

impl<I: StorageIterator> Eq for HeapWrapper<I> {}

impl<I: StorageIterator> PartialOrd for HeapWrapper<I> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<I: StorageIterator> Ord for HeapWrapper<I> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.1
            .key()
            .cmp(&other.1.key())
            .then(self.0.cmp(&other.0))
            .reverse()
    }
}

/// Merge multiple iterators of the same type. If the same key occurs multiple times in some
/// iterators, prefer the one with smaller index.
pub struct MergeIterator<I: StorageIterator> {
    iters: BinaryHeap<HeapWrapper<I>>,
    current: Option<HeapWrapper<I>>,
}

impl<I: StorageIterator> MergeIterator<I> {
    pub fn create(iters: Vec<Box<I>>) -> Self {
        unimplemented!()
    }
}

impl<I: 'static + for<'a> StorageIterator<KeyType<'a> = KeySlice<'a>>> StorageIterator
    for MergeIterator<I>
{
    type KeyType<'a> = KeySlice<'a>;

    fn key(&self) -> KeySlice {
        match self.current.as_ref() {
            Some(x) => x.1.key(),
            None => KeySlice::from_slice(&[]),
        }
    }

    fn value(&self) -> &[u8] {
        match self.current.as_ref() {
            Some(x) => x.1.value(),
            None => &[],
        }
    }

    fn is_valid(&self) -> bool {
        match self.current.as_ref() {
            Some(x) => x.1.is_valid(),
            None => false,
        }
    }

    fn next(&mut self) -> Result<()> {
        let current = self.current.as_mut().unwrap();
        while let Some(mut inner) = self.iters.peek_mut() {
            if current.1.key() == inner.1.key() {
                let e = inner.1.next();
                if let e @ Err(_) = e {
                    PeekMut::pop(inner);
                    return e;
                }
                if !inner.1.is_valid() {
                    PeekMut::pop(inner);
                }
            }
        }
        current.1.next()?;
        if !current.1.is_valid() {
            *current = self.iters.pop().unwrap();
        }

        if let Some(mut x) = self.iters.peek_mut() {
            if *current < *x {
                std::mem::swap(&mut *x, current);
            }
        }
        Ok(())
    }
}
