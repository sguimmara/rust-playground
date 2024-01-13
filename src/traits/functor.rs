use crate::Pair;

pub trait Functor<T> {
    type Functor<U>;
    fn fmap<U>(&self, f: impl Fn(&T) -> U) -> Self::Functor<U>;
}

impl<T> Functor<T> for Option<T> {
    type Functor<U> = Option<U>;

    fn fmap<U>(&self, f: impl Fn(&T) -> U) -> Option<U> {
        match &self {
            Some(v) => Some(f(v)),
            None => None,
        }
    }
}

impl<T, E> Functor<T> for Result<T, E>
where
    E: Clone,
{
    type Functor<U> = Result<U, E>;

    fn fmap<U>(&self, f: impl Fn(&T) -> U) -> Result<U, E> {
        match &self {
            Ok(v) => Ok(f(v)),
            Err(e) => Err(e.clone()),
        }
    }
}

impl<T> Functor<T> for Vec<T> {
    type Functor<U> = Vec<U>;

    fn fmap<U>(&self, f: impl Fn(&T) -> U) -> Vec<U> {
        let mut result = Vec::with_capacity(self.len());
        for v in self {
            result.push(f(v));
        }
        return result;
    }
}

impl<T> Functor<T> for Pair<T> {
    type Functor<U> = Pair<U>;

    fn fmap<U>(&self, f: impl Fn(&T) -> U) -> Self::Functor<U> {
        (f(&self.0), f(&self.1))
    }
}

#[cfg(test)]
mod test {
    use super::Functor;

    #[test]
    fn pair() {
        let input = (1, 2);
        let mapped = input.fmap(|x| x % 2 == 0);
        assert_eq!(mapped.0, false);
        assert_eq!(mapped.1, true);
    }

    #[test]
    fn option_none() {
        let input: Option<i32> = None;
        let mapped = input.fmap(|_| true);
        assert_eq!(None, mapped);
    }

    #[test]
    fn option_some() {
        let input = Some(45);
        let mapped = input.fmap(|_| true);
        assert_eq!(Some(true), mapped);
    }

    #[test]
    fn option_result_err() {
        let input: Result<i32, bool> = Err(false);
        let mapped = input.fmap(|_| 5);
        assert_eq!(Err(false), mapped);
    }

    #[test]
    fn option_result_ok() {
        let input: Result<i32, bool> = Ok(45);
        let mapped = input.fmap(|x| 2 * x);
        assert_eq!(Ok(90), mapped);
    }

    #[test]
    fn option_vec_empty() {
        let input: Vec<i32> = vec![];
        let mapped = input.fmap(|x| 2 * x);
        assert_eq!(0, mapped.len());
    }

    #[test]
    fn option_vec() {
        let input: Vec<i32> = vec![1, 2, 3, 4];
        let mapped = input.fmap(|x| x % 2 == 0);
        assert_eq!(4, mapped.len());
        assert_eq!(false, mapped[0]);
        assert_eq!(true, mapped[1]);
        assert_eq!(false, mapped[2]);
        assert_eq!(true, mapped[3]);
    }
}
