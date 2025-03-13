/// A trait for types that you can reverse left to right, or the opposite.
pub trait TrReverseLeftRight {
    type LeftType;
    type RightType;

    fn reverse(self) -> impl TrReverseLeftRight<LeftType = Self::RightType, RightType = Self::LeftType>;
}

/// Trait for types that may contain zero or more variants among left type and right type.
pub trait TrAnyLeftRight {
    type LeftType;
    type RightType;

    fn map_left<F, T>(self, f: F) -> impl TrAnyLeftRight<LeftType = T, RightType = Self::RightType >
    where
        F: FnOnce(Self::LeftType) -> T;

    fn map_right<F, T>(self, f: F) -> impl TrAnyLeftRight<LeftType = Self::LeftType, RightType = T>
    where
        F: FnOnce(Self::RightType) -> T;

    fn as_ref<'a>(&'a self) -> impl TrAnyLeftRight<LeftType = &'a Self::LeftType, RightType = &'a Self::RightType>
    where
        Self::LeftType: 'a,
        Self::RightType: 'a;

    fn as_mut<'a>(&'a mut self) -> impl TrAnyLeftRight<LeftType = &'a mut Self::LeftType, RightType = &'a mut Self::RightType>
    where
        Self::LeftType: 'a,
        Self::RightType: 'a;

    fn left(self) -> Option<Self::LeftType>;

    fn right(self) -> Option<Self::RightType>;

    // Provided methods

    fn is_left(&self) -> bool {
        self.as_ref().left().is_some()
    }

    fn is_right(&self) -> bool {
        self.as_ref().right().is_some()
    }
}

impl<X, E> TrAnyLeftRight for Result<X, E> {
    type LeftType = X;
    type RightType = E;

    #[inline]
    fn map_left<F, T>(self, f: F) -> impl TrAnyLeftRight<LeftType = T, RightType = Self::RightType >
    where
        F: FnOnce(Self::LeftType) -> T,
    {
        match self {
            Result::Ok(x) => Result::Ok(f(x)),
            Result::Err(e) => Result::Err(e),
        }
    }

    #[inline]
    fn map_right<F, T>(self, f: F) -> impl TrAnyLeftRight<LeftType = Self::LeftType, RightType = T>
    where
        F: FnOnce(Self::RightType) -> T,
    {
        match self {
            Result::Ok(x) => Result::Ok(x),
            Result::Err(e) => Result::Err(f(e)),
        }
    }

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyLeftRight<LeftType = &'a Self::LeftType, RightType = &'a Self::RightType>
    where
        Self::LeftType: 'a,
        Self::RightType: 'a,
    {
        self.as_ref()
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyLeftRight<LeftType = &'a mut Self::LeftType, RightType = &'a mut Self::RightType>
    where
        Self::LeftType: 'a,
        Self::RightType: 'a,
    {
        self.as_mut()
    }

    #[inline]
    fn left(self) -> Option<Self::LeftType> {
        self.ok()
    }

    #[inline]
    fn right(self) -> Option<Self::RightType> {
        self.err()
    }

    #[inline]
    fn is_left(&self) -> bool {
        self.is_ok()
    }

    #[inline]
    fn is_right(&self) -> bool {
        self.is_err()
    }
}
