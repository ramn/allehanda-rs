pub enum Rec<T> {
    Done(T),
    Go(Box<Fn() -> Rec<T>>)
}

impl<T> Rec<T> {
    pub fn done(x: T) -> Rec<T> {
        Rec::Done(x)
    }

    pub fn go<F: 'static + Fn() -> Rec<T>>(f: F) -> Rec<T> {
        Rec::Go(Box::new(f))
    }

    pub fn run(self) -> T {
        let mut f = self;
        loop {
            match f {
                Rec::Go(next) => f = next(),
                Rec::Done(result) => return result
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::Rec;

    #[test]
    fn test_factorial_with_rec() {
        fn fact((count, acc): (usize, usize)) -> Rec<usize> {
            if count == 0 {
                Rec::done(acc)
            } else {
                Rec::go(move || fact((count - 1, acc * count)))
            }
        }
        assert_eq!(fact((12, 1)).run(), 479001600);
        assert_eq!(fact((13, 1)).run(), 6227020800);
        assert_eq!(fact((14, 1)).run(), 87178291200);
    }

    #[test]
    fn test_fib_with_rec() {
        fn fib(n: usize, a: usize, b: usize) -> Rec<usize> {
            if n == 1 {
                Rec::done(a)
            } else {
                Rec::go(move || fib(n - 1, b, a + b))
            }
        }
        assert_eq!(fib(12, 1, 1).run(), 144);
    }

    #[test]
    fn test_rec_stack() {
        fn take(i: usize, acc: usize) -> Rec<usize> {
            if i == 0 {
                Rec::done(acc)
            } else {
                Rec::go(move || take(i - 1, acc + 1))
            }
        }
        assert_eq!(take(1000000, 0).run(), 1000000);
    }
}
