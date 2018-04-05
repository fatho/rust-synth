//! Provides some data structures that are useful for audio programming
use std::iter::FromIterator;
use std::collections::VecDeque;

pub struct SlidingWindowMax<T> {
    elements: VecDeque<T>,
    max_elements: VecDeque<T>
}

impl<T> SlidingWindowMax<T> where
    T: Ord + Copy,
{
    pub fn new() -> Self {
        SlidingWindowMax {
            elements: VecDeque::new(),
            max_elements: VecDeque::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        SlidingWindowMax {
            elements: VecDeque::with_capacity(capacity),
            max_elements: VecDeque::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        self.max_elements.clear();
    }

    pub fn enqueue(&mut self, value: T) {
        // remove all elements smaller than value from the front of the max queue
        while self.max_elements.front().map_or(false, |x| *x < value) {
            self.max_elements.pop_front();
        }
        // push value
        self.elements.push_front(value);
        self.max_elements.push_front(value);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let result = self.elements.pop_back();
        // remove current maximum if it's equal to the value being dequeued
        if self.max_elements.back() == result.as_ref() {
            self.max_elements.pop_back();
        }
        result
    }

    pub fn maximum(&self) -> Option<T> {
        self.max_elements.back().map(|x| *x)
    }
}

impl<T: Ord + Copy> FromIterator<T> for SlidingWindowMax<T> {
    fn from_iter<I>(iter: I) -> Self where
        I: IntoIterator<Item = T>
    {
        let mut real_iter = iter.into_iter();
        let mut window = SlidingWindowMax::with_capacity(real_iter.size_hint().0);
        real_iter.for_each(|x| window.enqueue(x));
        window
    }
}

#[test]
fn test_sliding_window_max() {
    let mut w = SlidingWindowMax::<i32>::new();
    w.enqueue(1); assert_eq!(w.maximum(), Some(1));
    w.enqueue(3); assert_eq!(w.maximum(), Some(3));
    w.enqueue(2); assert_eq!(w.maximum(), Some(3));
    assert_eq!(w.dequeue(), Some(1)); assert_eq!(w.maximum(), Some(3));
    assert_eq!(w.dequeue(), Some(3)); assert_eq!(w.maximum(), Some(2));
    w.enqueue(5); assert_eq!(w.maximum(), Some(5));
    w.enqueue(7); assert_eq!(w.maximum(), Some(7));
    assert_eq!(w.dequeue(), Some(2)); assert_eq!(w.maximum(), Some(7));
    assert_eq!(w.dequeue(), Some(5)); assert_eq!(w.maximum(), Some(7));
    assert_eq!(w.dequeue(), Some(7)); assert_eq!(w.maximum(), None);
}
