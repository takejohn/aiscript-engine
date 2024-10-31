pub(crate) trait GetByF64<T> {
    fn get_by_f64(&self, index: f64) -> Option<&T>;
}

impl<T> GetByF64<T> for [T] {
    fn get_by_f64(&self, index: f64) -> Option<&T> {
        let index_int = index as usize;
        if index == index as f64 {
            self.get(index_int)
        } else {
            None
        }
    }
}
