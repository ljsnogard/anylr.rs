use crate::{
    abs::{TrAnyLeftRight, TrReverseLeftRight},
    SomeOf,
};

#[derive(Clone, Debug)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> Either<L, R> {
    pub const fn new_left(l: L) -> Self {
        Either::Left(l)
    }

    pub const fn new_right(r: R) -> Self {
        Either::Right(r)
    }

    pub fn split(self) -> (Option<L>, Option<R>) {
        match self {
            Either::Left(l) => (Option::Some(l), Option::None),
            Either::Right(r) => (Option::None, Option::Some(r)),
        }
    }

    /// Maps Either<L, R> to Either<T, R>
    pub fn map_left<F, T>(self, f: F) -> Either<T, R>
    where
        F: FnOnce(L) -> T,
    {
        match self {
            Either::Left(l) => Either::new_left(f(l)),
            Either::Right(r) => Either::new_right(r),
        }
    }

    /// Maps Either<L, R> to Either<L, T>
    pub fn map_right<F, T>(self, f: F) -> Either<L, T>
    where
        F: FnOnce(R) -> T,
    {
        match self {
            Either::Left(l) => Either::<L, T>::Left(l),
            Either::Right(r) => Either::<L, T>::Right(f(r)),
        }
    }

    pub fn take_left(self) -> Either<L, Self> {
        match self {
            Either::Left(l) => Either::new_left(l),
            Either::Right(r) => Either::new_right(Either::new_right(r)),
        }
    }

    pub fn take_right(self) -> Either<R, Self> {
        match self {
            Either::Left(l) => Either::Right(Either::new_left(l)),
            Either::Right(r) => Either::new_left(r),
        }
    }

    pub const fn as_ref(&self) -> Either<&L, &R> {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(r)
        }
    }

    pub const fn as_mut(&mut self) -> Either<&mut L, &mut R> {
        match self {
            Either::Left(l) => Either::Left(l),
            Either::Right(r) => Either::Right(r)
        }
    }

    pub fn left(self) -> Option<L> {
        if let Either::Left(l) = self {
            Option::Some(l)
        } else {
            Option::None
        }
    }

    pub fn right(self) -> Option<R> {
        if let Either::Right(r) = self {
            Option::Some(r)
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
    type Lt = L;
    type Rt = R;

    #[inline]
    fn reverse(self) -> impl TrReverseLeftRight<Lt = Self::Rt, Rt = Self::Lt> {
        Either::reverse(self)
    }
}

impl<L, R> TrAnyLeftRight for Either<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn split(self) -> (Option<Self::Lt>, Option<Self::Rt>) {
        Either::split(self)
    }

    #[inline]
    fn map_left<F, T>(self, f: F) -> impl TrAnyLeftRight<Lt = T, Rt = Self::Rt >
    where
        F: FnOnce(Self::Lt) -> T,
    {
        Either::map_left(self, f)
    }

    #[inline]
    fn map_right<F, T>(self, f: F) -> impl TrAnyLeftRight<Lt = Self::Lt, Rt = T>
    where
        F: FnOnce(Self::Rt) -> T,
    {
        Either::map_right(self, f)
    }

    #[inline]
    fn take_left(self) -> SomeOf<Self::Lt, Self>
    where
        Self: Sized
    {
        Either::take_left(self).into()
    }

    #[inline]
    fn take_right(self) -> SomeOf<Self::Rt, Self>
    where
        Self: Sized
    {
        Either::take_right(self).into()
    }

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyLeftRight<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        Either::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyLeftRight<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        Either::as_mut(self)
    }

    #[inline]
    fn contains_left(&self) -> bool {
        Either::is_left(self)
    }

    #[inline]
    fn contains_right(&self) -> bool {
        Either::is_right(self)
    }
}
