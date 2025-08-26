// struct Vec<T> {
//     ptr: *mut T,  // points to heap buffer
//     len: usize,   // how many T are initialized
//     cap: usize,   // how many T the buffer can hold
// }

use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, PartialEq)]
struct MyVec<T> {
    data: Box<[T]>,
}

impl<T> MyVec<T> {
    fn new(v: Vec<T>) -> Self {
        Self {
            data: v.into_boxed_slice(),
        }
    }

    // These methods would be redundant thanks to Deref/DerefMut:
    // With Deref implemented, MyVec automatically gains access to all slice methods
    // No need to manually implement len(), get(), chunks(), reverse(), etc.

    // fn as_slice(&self) -> &[T] {
    //     &self.data
    // }
    //
    // fn as_slice_mut(&mut self) -> &mut [T] {
    //     &mut self.data
    // }
    //
    // fn len(&self) -> usize {
    //     self.data.len()
    // }
}

// Implementing Deref allows MyVec to automatically "dereference" to a slice [A]
// This enables ergonomic access to all slice methods without manual implementation
impl<WhateverTypeThisIs> Deref for MyVec<WhateverTypeThisIs> {
    type Target = [WhateverTypeThisIs];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

// DerefMut allows mutable access to slice methods
// This enables calling mutating methods like reverse() directly on MyVec
impl<WhateverTypeThisIs> DerefMut for MyVec<WhateverTypeThisIs> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T> AsRef<[T]> for MyVec<T> {
    fn as_ref(&self) -> &[T] { &*self }
}

impl<T> AsMut<[T]> for MyVec<T> {
    fn as_mut(&mut self) -> &mut [T] { &mut *self }
}

fn deref_iterator() {
    println!("\n========> DEREF ITERATOR");
    let mut mv = MyVec::new(vec![3, 1, 2]);
    // slice method via DerefMut
    mv.sort();
    // indexing via DerefMut + IndexMut on [T]
    mv[0] += 10;
    // range indexing via Deref
    let head = &mv[..3];
    println!("{:?}", head);
}

fn main() {
    let mv = MyVec::new(vec![10, 20, 30]);

    // we can call slice methods directly on MyVec
    println!("slice: {:?}", *&mv);
    // [T]::len() through Deref
    println!("len: {}", mv.len());
    // [T]::get() through Deref
    println!("first: {:?}", mv.get(0));
    // [T]::chunks() through Deref
    println!("slice: {:?}", mv.chunks(2));

    // Thanks to DerefMut, we can call mutating slice methods:
    let mut vm = mv.clone();
    vm.reverse(); // Uses [T]::reverse() through DerefMut
    
    deref_iterator();
}
