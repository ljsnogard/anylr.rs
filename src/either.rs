use crate::abs::{TrAnyLeftRight, TrReverseLeftRight};

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    /// Maps Either<L, R> to Either<T, R>
    pub fn map_left<F, T>(self, f: F) -> Either<T, R>
    where
        F: FnOnce(L) -> T,
    {
        match self {
            Either::Left(x) => Either::<T, R>::Left(f(x)),
            Either::Right(x) => Either::<T, R>::Right(x),
        }
    }

    /// Maps Either<L, R> to Either<L, T>
    pub fn map_right<F, T>(self, f: F) -> Either<L, T>
    where
        F: FnOnce(R) -> T,
    {
        match self {
            Either::Left(x) => Either::<L, T>::Left(x),
            Either::Right(x) => Either::<L, T>::Right(f(x)),
        }
    }

    pub const fn as_ref(&self) -> Either<&L, &R> {
        match self {
            Either::Left(x) => Either::Left(x),
            Either::Right(x) => Either::Right(x)
        }
    }

    pub const fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        match self {
            Either::Left(x) => Either::Left(x),
            Either::Right(x) => Either::Right(x)
        }
    }

    pub fn left(self) -> Option<L> {
        if let Either::Left(x) = self {
            Option::Some(x)
        } else {
            Option::None
        }
    }

    pub fn right(self) -> Option<R> {
        if let Either::Right(x) = self {
            Option::Some(x)
        } else {
            Option::None
        }
    }

    pub fn reverse(self) -> Either<R, L> {
        match self {
            Either::Left(x) => Either::<R, L>::Right(x),
            Either::Right(x) => Either::<R, L>::Left(x),
        }
    }

    pub const fn is_left(&self) -> bool {
        match self {
            Either::Left(_) => true,
            _ => false,
        }
    }

    pub const fn is_right(&self) -> bool {
        match self {
            Either::Right(_) => true,
            _ => false,
        }
    }

    pub fn is_left_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&L) -> bool,
    {
        if let Either::Left(x) = self {
            f(x)
        } else {
            false
        }
    }

    pub fn is_right_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&R) -> bool,
    {
        if let Either::Right(x) = self {
            f(x)
        } else {
            false
        }
    }
}

impl<T> Either<T, T> {
    pub fn into_inner(self) -> T {
        match self {
            Either::Left(x) => x,
            Either::Right(x) => x,
        }
    }
}

impl<L> From<Option<L>> for Either<L, ()> {
    fn from(value: Option<L>) -> Self {
        match value {
            Option::Some(left) => Either::Left(left),
            Option::None => Either::Right(()),
        }
    }
}

impl<T, E> From<Result<T, E>> for Either<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Result::Ok(t) => Either::Left(t),
            Result::Err(e) => Either::Right(e),
        }
    }
}

impl<L, R> TrReverseLeftRight for Either<L, R> {
    type LeftType = L;
    type RightType = R;

    #[inline]
    fn reverse(self) -> impl TrReverseLeftRight<LeftType = Self::RightType, RightType = Self::LeftType> {
        Either::reverse(self)
    }
}

impl<L, R> TrAnyLeftRight for Either<L, R> {
    type LeftType = L;
    type RightType = R;

    #[inline]
    fn map_left<F, T>(self, f: F) -> impl TrAnyLeftRight<LeftType = T, RightType = Self::RightType >
    where
        F: FnOnce(Self::LeftType) -> T,
    {
        Either::map_left(self, f)
    }

    #[inline]
    fn map_right<F, T>(self, f: F) -> impl TrAnyLeftRight<LeftType = Self::LeftType, RightType = T>
    where
        F: FnOnce(Self::RightType) -> T,
    {
        Either::map_right(self, f)
    }

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyLeftRight<LeftType = &'a Self::LeftType, RightType = &'a Self::RightType>
    where
        Self::LeftType: 'a,
        Self::RightType: 'a,
    {
        Either::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyLeftRight<LeftType = &'a mut Self::LeftType, RightType = &'a mut Self::RightType>
    where
        Self::LeftType: 'a,
        Self::RightType: 'a,
    {
        Either::as_mut(self)
    }

    #[inline]
    fn is_left(&self) -> bool {
        Either::is_left(self)
    }

    #[inline]
    fn is_right(&self) -> bool {
        Either::is_right(self)
    }

    #[inline]
    fn left(self) -> Option<Self::LeftType> {
        Either::left(self)
    }

    #[inline]
    fn right(self) -> Option<Self::RightType> {
        Either::right(self)
    }
}
