// struct Vec<T> {
//     ptr: *mut T,  // points to heap buffer
//     len: usize,   // how many T are initialized
//     cap: usize,   // how many T the buffer can hold
// }

struct MyVec<T> {
    data: Box<[T]>
}

impl<T> MyVec<T> {
    fn new(v: Vec<T>) -> Self {
        Self { data: v.into_boxed_slice() }
    }

    fn as_slice(&self) -> &[T] {
        &self.data
    }

    fn as_slice_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

fn main() {
    //let mv = MyVec::new(vec![10, 20, 30]);
    //
    //let mut slice = mv.as_slice();
    //println!("slice: {:?}", slice);
    //println!("len: {}", slice.len());
    //println!("first: {:?}", slice.get(0));
    //
    //slice[0] += 1
}
