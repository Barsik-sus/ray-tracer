use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec3<T> {
    val: [T; 3],
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { val: [x, y, z] }
    }
}

impl Vec3<f32> {
    pub fn length(&self) -> f32 {
        f32::sqrt(self.val[0] * self.val[0] + self.val[1] * self.val[1] + self.val[2] * self.val[2])
    }

    pub fn squared_length(&self) -> f32 {
        self.val[0] * self.val[0] + self.val[1] * self.val[1] + self.val[2] * self.val[2]
    }

    pub fn as_unit(&self) -> Vec3<f32> {
        *self / (*self).length()
    }

    pub fn dot(v1: &Vec3<f32>, v2: &Vec3<f32>) -> f32 {
        v1[0] * v2[0] + v1[1] * v2[1] + v1[2] * v2[2]
    }
}

// Indexes
impl<T> Index<usize> for Vec3<T> {
    type Output = T;
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &self.val[index]
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut<'a>(&'a mut self, index: usize) -> &'a mut Self::Output {
        &mut self.val[index]
    }
}

macro_rules! impl_operaion {
    ("A", $Operation:path, $Requirement:path, $Function:ident, T, $Operator:tt) => {
        impl<T> $Operation for Vec3<T>
        where
            T: $Requirement + Copy,
        {
            fn $Function (&mut self, rhs: T) {
                for i in self.val.iter_mut() {
                    // значение мувается в блок, надеюсь на компилятор
                    // но вообще, вроде как, тут в любом случае было бы копирование
                    *i $Operator rhs.clone();
                }
            }
        }
    };
    ("A", $Operation:path, $Requirement:path, $Function:ident, Vec3<T>, $Operator:tt) => {
        impl<T> $Operation for Vec3<T>
        where
            T: $Requirement + Copy,
        {
            fn $Function (&mut self, rhs: Vec3<T>) {
                for (l, r) in self.val.iter_mut().zip(rhs.val.iter()) {
                    *l $Operator *r;
                }
            }
        }
    };
    ($Operation:path, $Requirement:path, $Function:ident, $RhsType:ty, $Operator:tt) => {
        impl<T> $Operation for Vec3<T>
        where
            T: $Requirement + Copy,
        {
            type Output = Vec3<T>;
            fn $Function (mut self, rhs: $RhsType) -> Self::Output {
                self $Operator rhs;
                self
            }
        }
    }
}
// Muls
impl_operaion!("A", std::ops::MulAssign<T>, std::ops::MulAssign, mul_assign, T, *=);
impl_operaion!("A", std::ops::MulAssign<Vec3<T>>, std::ops::MulAssign, mul_assign, Vec3<T>,*=);
impl_operaion!(std::ops::Mul<T>, std::ops::MulAssign, mul, T, *=);
impl_operaion!(std::ops::Mul<Vec3<T>>, std::ops::MulAssign, mul, Vec3<T>, *=);

// Divs
impl_operaion!("A", std::ops::DivAssign<T>, std::ops::DivAssign, div_assign, T, /=);
impl_operaion!("A", std::ops::DivAssign<Vec3<T>>, std::ops::DivAssign, div_assign, Vec3<T>,/=);
impl_operaion!(std::ops::Div<T>, std::ops::DivAssign, div, T, /=);
impl_operaion!(std::ops::Div<Vec3<T>>, std::ops::DivAssign, div, Vec3<T>, /=);

// Adds
impl_operaion!("A", std::ops::AddAssign<T>, std::ops::AddAssign, add_assign, T, +=);
impl_operaion!("A", std::ops::AddAssign<Vec3<T>>, std::ops::AddAssign, add_assign, Vec3<T>,+=);
impl_operaion!(std::ops::Add<T>, std::ops::AddAssign, add, T, +=);
impl_operaion!(std::ops::Add<Vec3<T>>, std::ops::AddAssign, add, Vec3<T>, +=);

// Subs
impl_operaion!("A", std::ops::SubAssign<T>, std::ops::SubAssign, sub_assign, T, -=);
impl_operaion!("A", std::ops::SubAssign<Vec3<T>>, std::ops::SubAssign, sub_assign, Vec3<T>,-=);
impl_operaion!(std::ops::Sub<T>, std::ops::SubAssign, sub, T, -=);
impl_operaion!(std::ops::Sub<Vec3<T>>, std::ops::SubAssign, sub, Vec3<T>, -=);

#[cfg(test)]
mod tests {
    use crate::Vec3;

    #[test]
    fn creator() {
        let result = Vec3::new(1, 2, 3);
        assert_eq!(result, Vec3 { val: [1, 2, 3] });
    }

    #[test]
    fn get_val() {
        let vec = Vec3::new(1, 2, 3);
        assert_eq!(vec[2], 3, "not mutable");
        let mut vec = Vec3::new(1, 2, 3);
        vec[2] = 89;
        assert_eq!(vec[2], 89, "matable");
    }

    #[test]
    fn mul() {
        let mut vec = Vec3::new(1, 2, 3);
        vec *= 3;
        assert_eq!(vec, Vec3 { val: [3, 6, 9] }, "*= 3");
        let vec2 = vec * 3;
        assert_eq!(vec2, Vec3 { val: [9, 18, 27] }, "vec * 3");
        let mut vec3 = vec.clone();
        vec3 *= vec2;
        assert_eq!(
            vec3,
            Vec3 {
                val: [27, 108, 243]
            },
            "*= vec2"
        );
        let vec3 = vec * vec2;
        assert_eq!(
            vec3,
            Vec3 {
                val: [27, 108, 243]
            },
            "vec * vec2"
        );
    }
}
