
#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    pub data: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.size+=1;
    }

    /// Removes the last element from a vector and returns it, or [`None`] if it
    /// is empty.
    pub fn pop(&mut self) -> Option<T> {
        if 0 == self.size {
            return None;
        }

        self.size-=1;
        self.data.pop()
    }

    /// Returns a reference to an element
    pub fn peek(&self) -> Option<&T> {
        if 0 == self.size {
            return None;
        }

        self.data.get(self.size-1)
    }

    /// Returns a mutable reference to an element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size {
            return None;
        }

        self.data.get_mut(self.size - 1)
    }

    fn into_iter(self) -> IntoIter<T> { // pass ownership
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> { // pass a reference or borrow
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }

        iterator
    }
}

struct IntoIter<T>(Stack<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size-=1;
            self.0.data.pop()
        } else {
            return None;
        }
    }
}

struct Iter<'a, T: 'a> {stack: Vec<&'a T>}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}


#[cfg(test)]
mod test_stack {
    use super::Stack;


    #[test]
    fn it_works() {
        let mut my_stack = Stack::new();
        my_stack.push(1);
        my_stack.push(2);
        my_stack.push(3);
        my_stack.push(4);

        for item in my_stack.into_iter() {
            println!("{}", item);
        }
    }

    #[test]
    fn it_ref_works(){
        let mut my_stack = Stack::new();
        my_stack.push(1);
        my_stack.push(2);
        my_stack.push(3);
        my_stack.push(4);


        for item in my_stack.iter() {
            println!("{}", item);
        }

        println!("{:?}", my_stack.pop());
    }
}
