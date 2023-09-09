use num_traits::Float;

pub trait Coord<T = f64>
where
    T: Float,
{
    fn distance_to(&self, rhs: &Self) -> T;
}

macro_rules! impl_coord {
    ($($t:ty),*) => {
        $(
            impl Coord<$t> for $t {
                fn distance_to(&self, rhs: &Self) -> $t {
                    (*rhs - *self).abs()
                }
            }

            impl Coord<$t> for ($t, $t) {
                fn distance_to(&self, rhs: &Self) -> $t {
                    let dx = rhs.0 - self.0;
                    let dy = rhs.1 - self.1;
                    (dx * dx + dy * dy).sqrt()
                }
            }

            impl Coord<$t> for ($t, $t, $t) {
                fn distance_to(&self, rhs: &Self) -> $t {
                    let dx = rhs.0 - self.0;
                    let dy = rhs.1 - self.1;
                    let dz = rhs.2 - self.2;
                    (dx * dx + dy * dy + dz * dz).sqrt()
                }
            }
        )*
    };
}

impl_coord!(f32, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to_1d() {
        assert_eq!(3.0.distance_to(&4.0), 1.0);
        assert_eq!((-3.0).distance_to(&4.0), 7.0);
        assert_eq!((-3.0).distance_to(&(-4.0)), 1.0);
        assert_eq!(3.0.distance_to(&(-4.0)), 7.0);
        assert_eq!(3.5.distance_to(&4.5), 1.0);
    }

    #[test]
    fn test_distance_to_2d() {
        assert_eq!((0.0, 0.0).distance_to(&(3.0, 4.0)), 5.0);
        assert_eq!((1.0, 1.0).distance_to(&(4.0, 5.0)), 5.0);
        assert_eq!((-1.0, -1.0).distance_to(&(-4.0, -5.0)), 5.0);
        assert_eq!((-1.0, -1.0).distance_to(&(2.0, 3.0)), 5.0);
        assert_eq!((1.5, 1.5).distance_to(&(4.5, 5.5)), 5.0);
    }

    #[test]
    fn test_distance_to_3d() {
        assert_eq!((0.0, 0.0, 0.0).distance_to(&(1.0, 2.0, 2.0),), 3.0);
        assert_eq!((-1.0, -2.0, -2.0).distance_to(&(-2.0, -4.0, -4.0),), 3.0);
        assert_eq!((1.0, 2.0, 2.0).distance_to(&(2.0, 4.0, 4.0),), 3.0);
        assert_eq!((1.0, 2.0, 2.0).distance_to(&(-1.0, -2.0, -2.0),), 6.0);
        assert_eq!((-1.0, -2.0, -2.0).distance_to(&(1.0, 2.0, 2.0),), 6.0);
    }
}
