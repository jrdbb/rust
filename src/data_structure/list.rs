pub enum List<T> {
    Nil,
    Elem { value: T, next: Box<List<T>>},
}

impl<T> List<T> {
    pub fn new() -> Self {
        List::Nil
    }

    pub fn empty(&self) -> bool {
        match self {
            List::Nil => true,
            List::Elem{value: _, next: _} => false
        }
    }

    pub fn insert(&mut self, value: T) {
        match self {
            List::Nil => { *self = List::Elem {value: value, next: Box::new(List::Nil)} },
            List::Elem{value:_, next} => {
                // intend to move the field out of a reference...this is an unsafe action
                *next = Box::new(List::Elem{value: value, next: std::mem::take(next)});
            }
        }
    }

    pub fn unwarp(&self) -> &T
    {
        match self {
            List::Nil => panic!(""),
            List::Elem{value, next: _} => &value
        }
    }

    pub fn next(&self) -> &List<T>
    {
        match self {
            List::Nil => panic!(""),
            List::Elem{value:_, next} => &next
        }
    }

    pub fn remove(&mut self)
    {
        match self {
            List::Nil => panic!(""),
            List::Elem{value: _, next} => {
                match &**next {
                    List::Nil => {
                        *self = List::Nil;
                    },
                    List::Elem{value:_, next: nn} => {

                    }
                }
            }
        }
    }
}

impl<T> Default for List<T> {
    fn default() -> Self { List::Nil }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct() {
        let mut l : List<i32> = List::new();
        assert_eq!(l.empty(), true);
        l.insert(12);
        assert_eq!(l.empty(), false);
        assert_eq!(l.unwarp(), &12);
    }
}