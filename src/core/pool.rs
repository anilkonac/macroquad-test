pub struct ObjectPool<T> {
    objects: Vec<T>,
    index_next: usize,
}

impl<T: Default> ObjectPool<T> {
    pub fn new(size: usize) -> ObjectPool<T> {
        let mut objects = Vec::with_capacity(size);
        for _ in 0..size {
            objects.push(T::default());
        }

        ObjectPool {
            objects,
            index_next: 0,
        }
    }
}

impl<T> ObjectPool<T> {
    pub fn get_next_mut(&mut self) -> &mut T {
        let index_cur = self.index_next;

        self.index_next += 1;
        if self.index_next >= self.objects.len() {
            self.index_next = 0;
        }

        &mut self.objects[index_cur]
    }

    pub fn for_each<F>(&self, f: F)
    where
        F: Fn(&T),
    {
        for object in &self.objects {
            f(object);
        }
    }

    pub fn for_each_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T),
    {
        for object in &mut self.objects {
            f(object);
        }
    }
}
