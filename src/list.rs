use std::rc::Rc;


#[derive(Debug, Eq, PartialEq)]
pub enum List<T> {
    Cons(Rc<T>, Rc<List<T>>),
    Nil,
}

pub fn cons<T>(x: T, xs: Rc<List<T>>) -> Rc<List<T>> {
    Rc::new(List::Cons(Rc::new(x), xs))
}

pub fn nil<T>() -> Rc<List<T>> {
    Rc::new(List::Nil)
}

pub fn iter<T>(xs: Rc<List<T>>) -> Iter<T> {
    Iter(xs)
}

pub struct Iter<T>(Rc<List<T>>);

impl<T> Iterator for Iter<T> {
    type Item = Rc<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (tail, result) = match &*self.0 {
            &List::Nil => {
                (self.0.clone(), None)
            }
            &List::Cons(ref x, ref xs) => {
                (xs.clone(), Some(x.clone()))
            }
        };
        self.0 = tail;
        result
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use super::{nil, cons, List, iter};

    #[test]
    fn test_cons() {
        let mut my_list = vec![nil(); 4];
        my_list[2] = cons(99, my_list[1].clone());
        my_list[3] = cons(100, my_list[2].clone());
        println!("{:?}", my_list[3]);
        assert_eq!(my_list[3], cons(100, cons(99, nil())));
    }

    #[test]
    fn test_collect() {
        let xs: Rc<List<u32>> = cons(100, cons(99, nil()));
        let v = iter(xs).collect::<Vec<_>>();
        println!("{:?}", v);
        assert_eq!(v, vec![Rc::new(100), Rc::new(99)]);
    }
}
