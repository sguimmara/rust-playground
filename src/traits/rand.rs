use rand::{Rng, distributions::{Distribution, Standard}};

struct Angle(f64);

struct Wrapper<T>(T);

// Implementing rng for any custom type
// goes through the Distribution trait for Standard
impl Distribution<Angle> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Angle {
        Angle(rng.gen())
    }
}

// For generic types, we can add the where clause to constraint T to be also randomizable
impl<T> Distribution<Wrapper<T>> for Standard where Standard: Distribution<T> {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Wrapper<T> {
        Wrapper(rng.gen())
    }
}
