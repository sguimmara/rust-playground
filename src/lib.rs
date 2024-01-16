#[macro_use]
pub mod data_structures;
pub mod encryption;
pub mod serde;
pub mod traits;
pub mod redux;
pub mod cache;
pub mod graphics;

type Pair<T> = (T, T);

pub fn compose<I, M, O, F, G>(f: F, g: G) -> impl Fn(I) -> O
where
    F: Fn(I) -> M,
    G: Fn(M) -> O,
{
    move |x| g(f(x))
}
