use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T>
where
    T: Mul + Add + Sub + Div + Copy,
{
    x: T,
    y: T,
    z: T,
}

impl<T> Default for Vec3<T>
where
    T: Mul + Add + Sub + Div + Default + Copy,
{
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(&self) -> Self {
        f32::sqrt(*self)
    }
}

impl Sqrt for f64 {
    fn sqrt(&self) -> Self {
        f64::sqrt(*self)
    }
}

pub trait Component: Mul + Add + Sub + Div + Default + Copy + Sqrt {}

impl<T> Vec3<T>
where
    T: Component,
    <T as Mul<T>>::Output: Add<Output = T>,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn into<U>(self) -> Vec3<U> where U: From<T> + Component, <U as Mul<U>>::Output: Add<Output = U>, {
        Vec3::new(self.x.into(), self.y.into(), self.z.into())
    }

    pub fn copy(&mut self, other: &Vec3<T>) {
        self.x = other.x;
        self.y = other.y;
        self.z = other.z;
    }

    // pub fn magnitude(&self) -> T {
    //     self.square_magnitude().sqrt()
    // }

    // pub fn square_magnitude(&self) -> T {
    //     let xx = self.x * self.x;
    //     let yy = self.y * self.y;
    //     let zz = self.z * self.z;

    //     xx + yy + zz
    // }
}
