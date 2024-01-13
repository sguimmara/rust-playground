use crate::Pair;

pub trait Fold<T> {
    fn fold(&self, start: T, f: fn(&T, &T) -> T) -> T;
}

impl<T> Fold<T> for Pair<T> {
    fn fold(&self, start: T, f: fn(&T, &T) -> T) -> T {
        return f(&start, &f(&self.0, &self.1));
    }
}

impl<T> Fold<T> for Vec<T> {
    fn fold(&self, start: T, f: fn(&T, &T) -> T) -> T {
        if self.len() == 0 {
            return start;
        }

        let mut accum = start;

        for value in self {
            accum = f(&accum, &value);
        }

        accum
    }
}

#[cfg(test)]
mod test {
    use super::Fold;

    #[test]
    fn fold_empty_vec() {
        let v = vec![];

        let start = 321;

        let result = v.fold(start, |a, b| { a + b });

        assert_eq!(result, start);
    }

    #[test]
    fn fold_vec_add() {
        let v = vec![1, 2, 3];

        let result = v.fold(0, |a, b| { a + b });

        assert_eq!(result, 6);
    }

    #[test]
    fn fold_vec_mul() {
        let v = vec![3, 3, 4];

        let result = v.fold(1, |a, b| { a * b });

        assert_eq!(result, 36);
    }
}