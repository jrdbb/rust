struct List<T> {
    mValue: Option<T>,
    mPrev: Option<Box<List<T>>>,
    mNext: Option<Box<List<T>>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {mValue: None, mPrev: None, mNext: None }
    }

    pub fn empty(&self) -> bool {
        self.mNext.is_none()
    }

    pub fn insert(&self, value: T) {
        let currentNext = self.mNext;
        self.mNext = Some(Box::new(List {mValue: Some(value), mPrev: Some(Box::new(self)), mNext: currentNext}));
    }

    pub fn get(&self) -> &T
    {
        &self.mValue.unwrap()
    }

    pub fn remove(&self)
    {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let l : List<i32> = List::new();
    }
}