#[derive(Debug)]
pub struct Pages<T> {
    first: T,
    pages: Vec<Option<T>>,
}

impl<T> Pages<T> {
    pub fn new(first: T) -> Self {
        Pages {
            first,
            pages: Vec::new(),
        }
    }

    pub fn add(&mut self, value: T, idx: usize) {
        if idx == 0 {
            panic!("Index can't be a zero");
        }

        self.resize_to(idx);

        self.pages[idx - 1] = Some(value);
    }

    fn resize_to(&mut self, new_len: usize) {
        if self.pages.len() < new_len {
            for _ in 0..new_len - self.pages.len() {
                self.pages.push(None);
            }
        }
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx == 0 {
            Some(&self.first)
        } else {
            self.pages
                .get(idx - 1)
                .unwrap_or(&None)
                .as_ref()
        }
    }

    #[allow(dead_code)]
    pub fn get_or_first(&self, idx: usize) -> &T { self.get(idx).unwrap_or(&self.first) }

    pub fn first(&self) -> &T { &self.first }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pages() {
        let mut p = Pages::new(());
        p.add((), 4);
        p.add((), 3);

        assert_eq!(p.get(0), Some(&()));
        assert_eq!(p.get(1), None);
        assert_eq!(p.get(2), None);
        assert_eq!(p.get(3), Some(&()));
        assert_eq!(p.get(4), Some(&()));
        assert_eq!(p.get(5), None);
        assert_eq!(p.pages.len(), 4);
    }

    #[test]
    fn get_or_first() {
        let mut p = Pages::new(1);
        p.add(0, 2);

        assert_eq!(p.get_or_first(0), &1);
        assert_eq!(p.get_or_first(1), &1);
        assert_eq!(p.get_or_first(2), &0);
        assert_eq!(p.get_or_first(3), &1);
    }
}
