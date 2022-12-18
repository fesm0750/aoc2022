use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    fmt::Debug,
    ops::{Add, AddAssign, Sub, SubAssign},
    str::FromStr,
};

/// Helper struct for representing 2d values, i.e: coordinates, indexes, etc.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Base2d<U> {
    pub x: U,
    pub y: U,
}

impl<U: Copy> Base2d<U> {
    /// Constructs a new Base2d
    pub fn new(x: U, y: U) -> Base2d<U> {
        Base2d { x, y }
    }

    /// Constructs a new Base2d from a tuple
    /// In some situation a tuple is a more handy alternative.
    pub fn from_tuple(t: (U, U)) -> Base2d<U> {
        Base2d { x: t.0, y: t.1 }
    }

    /// Returns a tuple `(x, y)`.
    pub fn tuple(&self) -> (U, U) {
        (self.x, self.y)
    }
}

impl<U: PartialEq> Base2d<U> {
    pub fn is_same_column(&self, rhs: &Self) -> bool {
        self.x == rhs.x
    }

    pub fn is_same_row(&self, rhs: &Self) -> bool {
        self.y == rhs.y
    }
}

//--------------------------------------------------------------------
// Operations
//--------------------------------------------------------------------

impl<U> Add for Base2d<U>
where
    U: Add<Output = U>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<U: AddAssign> AddAssign for Base2d<U> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<U> Sub for Base2d<U>
where
    U: Sub<Output = U>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<U: SubAssign> SubAssign for Base2d<U> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

//--------------------------------------------------------------------
// Conversion traits
//--------------------------------------------------------------------

/// Defines how to convert a tuple (U, U) to a Base2d<T>,`U` must implement `TryInto<T>`. An user case would be to
/// convert a tuple of `usize` to a Base2d of a smaller unsigned or an integer.
///
///  If the conversion is Infallible, the method `from_tuple` may be a more handy alternative.
impl<U, T> TryFrom<(U, U)> for Base2d<T>
where
    U: TryInto<T>,
    <U as TryInto<T>>::Error: std::error::Error + 'static,
{
    type Error = Box<dyn Error>;

    fn try_from(item: (U, U)) -> Result<Self, Self::Error> {
        Ok(Base2d {
            x: item.0.try_into()?,
            y: item.1.try_into()?,
        })
    }
}

impl<U> FromStr for Base2d<U>
where
    U: FromStr + Copy,
    <U as FromStr>::Err: std::error::Error + 'static,
{
    type Err = Box<dyn Error>;

    /// string needs to have two values separated by comma (','). Example: "15,21"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        let x = iter
            .next()
            .ok_or("Could not parse the number before the comma.")?
            .parse::<U>()?;
        let y = iter
            .next()
            .ok_or("Could not parse the number after the comma.")?
            .parse::<U>()?;
        Ok(Base2d::new(x, y))
    }
}
