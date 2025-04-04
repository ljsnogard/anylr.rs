use crate::{
    abs::{TrAnyLeftRight, TrReverseLeftRight},
    both::Both, either::Either, some_of::SomeOf,
};

#[derive(Clone, Debug)]
pub enum AnyOf<L, R> {
    Neither,
    Left(L),
    Right(R),
    Both(Both<L, R>),
}

impl<L, R> AnyOf<L, R> {
    pub fn map_left<F, T>(self, f: F) -> AnyOf<T, R>
    where
        F: FnOnce(L) -> T,
    {
        match self {
            AnyOf::Left(x) => AnyOf::Left(f(x)),
            AnyOf::Both(x) => AnyOf::Both(x.map_left(f)),
            _ => AnyOf::Neither,
        }
    }

    pub fn map_right<F, T>(self, f: F) -> AnyOf<L, T>
    where
        F: FnOnce(R) -> T,
    {
        match self {
            AnyOf::Right(x) => AnyOf::Right(f(x)),
            AnyOf::Both(x) => AnyOf::Both(x.map_right(f)),
            _ => AnyOf::Neither,
        }
    }

    pub fn as_ref(&self) -> AnyOf<&L, &R> {
        match self {
            AnyOf::Neither => AnyOf::Neither,
            AnyOf::Left(x) => AnyOf::Left(x),
            AnyOf::Right(x) => AnyOf::Right(x),
            AnyOf::Both(x) => AnyOf::Both(x.as_ref()),
        }
    }

    pub fn as_mut(&mut self) -> AnyOf<&mut L, &mut R> {
        match self {
            AnyOf::Neither => AnyOf::Neither,
            AnyOf::Left(x) => AnyOf::Left(x),
            AnyOf::Right(x) => AnyOf::Right(x),
            AnyOf::Both(x) => AnyOf::Both(x.as_mut()),
        }
    }

    pub fn reverse(self) -> AnyOf<R, L> {
        match self {
            AnyOf::Neither => AnyOf::Neither,
            AnyOf::Left(x) => AnyOf::Right(x),
            AnyOf::Right(x) => AnyOf::Left(x),
            AnyOf::Both(x) => AnyOf::Both(x.reverse()),
        }
    }

    pub fn is_left(&self) -> bool {
        match self {
            AnyOf::Left(_) => true,
            AnyOf::Both(_) => true,
            _ => false,
        }
    }

    pub fn is_right(&self) -> bool {
        match self {
            AnyOf::Right(_) => true,
            AnyOf::Both(_) => true,
            _ => false,
        }
    }
}

impl<L, R> Default for AnyOf<L, R> {
    fn default() -> Self {
        AnyOf::Neither
    }
}

impl<L, R> From<Either<L, R>> for AnyOf<L, R> {
    fn from(value: Either<L, R>) -> Self {
        match value {
            Either::Left(x) => AnyOf::Left(x),
            Either::Right(x) => AnyOf::Right(x),
        }
    }
}

impl<L, R> From<Both<L, R>> for AnyOf<L, R> {
    fn from(value: Both<L, R>) -> Self {
        AnyOf::Both(value)
    }
}

impl<L, R> From<SomeOf<L, R>> for AnyOf<L, R> {
    fn from(value: SomeOf<L, R>) -> Self {
        match value {
            SomeOf::Left(x) => AnyOf::Left(x),
            SomeOf::Right(x) => AnyOf::Right(x),
            SomeOf::Both(x) => AnyOf::Both(x),
        }
    }
}

impl<L, R> From<Option<SomeOf<L, R>>> for AnyOf<L, R> {
    fn from(value: Option<SomeOf<L, R>>) -> Self {
        if let Option::Some(x) = value {
            x.into()
        } else {
            AnyOf::Neither
        }
    }
}

impl<L, R> From<(Option<L>, Option<R>)> for AnyOf<L, R> {
    fn from(value: (Option<L>, Option<R>)) -> Self {
        match value {
            (Option::Some(l), Option::Some(r)) => AnyOf::Both(Both::new(l, r)),
            (Option::Some(l), Option::None) => AnyOf::Left(l),
            (Option::None, Option::Some(r)) => AnyOf::Right(r),
            (Option::None, Option::None) => AnyOf::Neither,
        }
    }
}

impl<L, R> TrReverseLeftRight for AnyOf<L, R> {
    type LeftType = L;
    type RightType = R;

    #[inline]
    fn reverse(self) -> impl TrReverseLeftRight<LeftType = Self::RightType, RightType = Self::LeftType> {
        AnyOf::reverse(self)
    }
}

impl<L, R> TrAnyLeftRight for AnyOf<L, R> {
    type LeftType = L;
    type RightType = R;

    #[inline]
    fn map_left<F, T>(self, f: F) -> impl TrAnyLeftRight<LeftType = T, RightType = Self::RightType >
    where
        F: FnOnce(Self::LeftType) -> T,
    {
        AnyOf::map_left(self, f)
    }

    #[inline]
    fn map_right<F, T>(self, f: F) -> impl TrAnyLeftRight<LeftType = Self::LeftType, RightType = T>
    where
        F: FnOnce(Self::RightType) -> T,
    {
        AnyOf::map_right(self, f)
    }

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyLeftRight<LeftType = &'a Self::LeftType, RightType = &'a Self::RightType>
    where
        Self::LeftType: 'a,
        Self::RightType: 'a,
    {
        AnyOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyLeftRight<LeftType = &'a mut Self::LeftType, RightType = &'a mut Self::RightType>
    where
        Self::LeftType: 'a,
        Self::RightType: 'a,
    {
        AnyOf::as_mut(self)
    }

    #[inline]
    fn is_left(&self) -> bool {
        AnyOf::is_left(self)
    }

    #[inline]
    fn is_right(&self) -> bool {
        AnyOf::is_right(self)
    }

    fn left(self) -> Option<Self::LeftType> {
        match self {
            AnyOf::Left(x) => Option::Some(x),
            AnyOf::Both(x) => Option::Some(x.left),
            _ => Option::None,
        }
    }

    fn right(self) -> Option<Self::RightType> {
        match self {
            AnyOf::Right(x) => Option::Some(x),
            AnyOf::Both(x) => Option::Some(x.right),
            _ => Option::None,
        }
    }
}
