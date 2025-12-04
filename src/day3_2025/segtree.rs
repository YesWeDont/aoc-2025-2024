// simple segtree implementation
pub(crate) struct Segtree<T, U: Fn() -> T, F: Fn(&T, &T) -> T> {
    pub len: usize,
    storage: Vec<T>,
    combiner: F,
    blanker: U,
}

impl<T, U: Fn() -> T, F: Fn(&T, &T) -> T> Segtree<T, U, F> {
    pub fn new(len: usize, combiner: F, blanker: U) -> Self {
        let mut storage = Vec::with_capacity(len * 2);
        storage.resize_with(len * 2, || blanker());
        Self {
            len,
            storage,
            combiner,
            blanker,
        }
    }
    pub fn data(&mut self) -> &mut [T] {
        let len = self.storage.len();
        &mut self.storage[self.len..len]
    }
    pub fn build(&mut self) -> &mut Self {
        for i in (0..self.len).rev() {
            self.storage[i] = (self.combiner)(&self.storage[i * 2], &self.storage[i * 2 + 1]);
        }
        self
    }
    pub fn query(&self, std::ops::Range { mut start, mut end }: std::ops::Range<usize>) -> T {
        let mut result_left = (self.blanker)();
        let mut result_right = (self.blanker)();
        println!("Query {start}..{end}");
        if end > self.len {
            panic!("End query exceeded capacity {start}..{end}")
        }
        if start > self.len {
            panic!("Start query exceeded capacity {start}..{end}")
        }
        if start >= end {
            panic!("Blank or non-existent query range {start}..{end}",)
        }
        start += self.len;
        end += self.len;
        loop {
            if start >= end {
                return (self.combiner)(&result_left, &result_right);
            } else {
                if start % 2 == 1 {
                    result_left = (self.combiner)(&result_left, &self.storage[start]);
                    start += 1;
                }
                if end % 2 == 1 {
                    end -= 1;
                    result_right = (self.combiner)(&self.storage[end], &result_right);
                }
                start /= 2;
                end /= 2;
            }
        }
    }
}
