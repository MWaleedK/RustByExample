#[derive(Debug)]
pub struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }
    pub fn is_empty(&self) -> bool {
        0 == self.data.len()
    }
    pub fn is_full(&self) -> bool {
        self.data.len() == self.cap
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }
    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.len() == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }
    pub fn dequeue(&mut self) -> Option<T> {
        if self.len() > 0 {
            self.data.pop()
        } else {
            None
        }
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { queue: Vec::new() };
        for item in self.data.iter() {
            iterator.queue.push(item);
        }
        iterator
    }
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { queue: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.queue.push(item);
        }
        iterator
    }
}

pub struct IntoIter<T>(Queue<T>);
impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            Some(self.0.data.remove(0))
        } else {
            None
        }
    }
}
pub struct Iter<'a, T: 'a> {
    queue: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.queue.len() {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }
}
pub struct IterMut<'a, T: 'a> {
    queue: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if 0 != self.queue.len() {
            Some(self.queue.remove(0))
        } else {
            None
        }
    }
}
