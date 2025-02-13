use crate::{
    abs::{TrAnyLeftRight, TrReverseLeftRight},
    any_of::AnyOf, both::Both, either::Either,
};

pub enum SomeOf<L, R> {
    Left(L),
    Right(R),
    Both(Both<L, R>),
}

impl<L, R> SomeOf<L, R> {
    pub fn map_left<F, T>(self, f: F) -> AnyOf<T, R>
    where
        F: FnOnce(L) -> T,
    {
        match self {
            SomeOf::Left(x) => AnyOf::Left(f(x)),
            SomeOf::Both(x) => AnyOf::Both(x.map_left(f)),
            _ => AnyOf::Neither,
        }
    }

    pub fn map_right<F, T>(self, f: F) -> AnyOf<L, T>
    where
        F: FnOnce(R) -> T,
    {
        match self {
            SomeOf::Right(x) => AnyOf::Right(f(x)),
            SomeOf::Both(x) => AnyOf::Both(x.map_right(f)),
            _ => AnyOf::Neither,
        }
    }

    pub fn as_ref(&self) -> SomeOf<&L, &R> {
        match self {
            SomeOf::Left(x) => SomeOf::Left(x),
            SomeOf::Right(x) => SomeOf::Right(x),
            SomeOf::Both(x) => SomeOf::Both(x.as_ref()),
        }
    }

    pub fn as_mut(&mut self) -> SomeOf<&mut L, &mut R> {
        match self {
            SomeOf::Left(x) => SomeOf::Left(x),
            SomeOf::Right(x) => SomeOf::Right(x),
            SomeOf::Both(x) => SomeOf::Both(x.as_mut()),
        }
    }

    pub fn reverse(self) -> SomeOf<R, L> {
        match self {
            SomeOf::Left(x) => SomeOf::Right(x),
            SomeOf::Right(x) => SomeOf::Left(x),
            SomeOf::Both(x) => SomeOf::Both(x.reverse()),
        }
    }

    pub fn left(self) -> Option<L> {
        match self {
            SomeOf::Left(x) => Option::Some(x),
            SomeOf::Both(x) => Option::Some(x.left),
            _ => Option::None,
        }
    }

    pub fn right(self) -> Option<R> {
        match self {
            SomeOf::Right(x) => Option::Some(x),
            SomeOf::Both(x) => Option::Some(x.right),
            _ => Option::None,
        }
    }

    pub fn is_left(&self) -> bool {
        match self {
            SomeOf::Left(_) => true,
            SomeOf::Both(_) => true,
            _ => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            SomeOf::Right(_) => true,
            SomeOf::Both(_) => true,
            _ => false,
        }
    }
}

impl<L, R> From<Either<L, R>> for SomeOf<L, R> {
    fn from(value: Either<L, R>) -> Self {
        match value {
            Either::Left(x) => SomeOf::Left(x),
            Either::Right(x) => SomeOf::Right(x),
        }
    }
}

impl<L, R> From<Both<L, R>> for SomeOf<L, R> {
    fn from(value: Both<L, R>) -> Self {
        SomeOf::Both(value)
    }
}

impl<L, R> TrReverseLeftRight for SomeOf<L, R> {
    type LeftType = L;
    type RightType = R;

    #[inline]
    fn reverse(self) -> impl TrReverseLeftRight<LeftType = Self::RightType, RightType = Self::LeftType> {
        SomeOf::reverse(self)
    }
}

impl<L, R> TrAnyLeftRight for SomeOf<L, R> {
    type LeftType = L;
    type RightType = R;

    #[inline]
    fn map_left<F, T>(self, f: F) -> impl TrAnyLeftRight<LeftType = T, RightType = Self::RightType >
    where
        F: FnOnce(Self::LeftType) -> T,
    {
        SomeOf::map_left(self, f)
    }

    #[inline]
    fn map_right<F, T>(self, f: F) -> impl TrAnyLeftRight<LeftType = Self::LeftType, RightType = T>
    where
        F: FnOnce(Self::RightType) -> T,
    {
        SomeOf::map_right(self, f)
    }

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyLeftRight<LeftType = &'a Self::LeftType, RightType = &'a Self::RightType>
    where
        Self::LeftType: 'a,
        Self::RightType: 'a,
    {
        SomeOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyLeftRight<LeftType = &'a mut Self::LeftType, RightType = &'a mut Self::RightType>
    where
        Self::LeftType: 'a,
        Self::RightType: 'a,
    {
        SomeOf::as_mut(self)
    }

    #[inline]
    fn is_left(&self) -> bool {
        SomeOf::is_left(self)
    }

    #[inline]
    fn is_right(&self) -> bool {
        SomeOf::is_right(self)
    }

    #[inline]
    fn left(self) -> Option<Self::LeftType> {
        SomeOf::left(self)
    }

    #[inline]
    fn right(self) -> Option<Self::RightType> {
        SomeOf::right(self)
    }
}
