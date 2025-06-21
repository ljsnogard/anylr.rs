use crate::{
    abs::{TrAnyOf, TrInverseLR}, AnyOf, SomeOf
};

pub trait TrEitherOf {
    type Lt;
    type Rt;

    fn map_left<F, U>(self, f: F) -> impl TrEitherOf<Lt = U, Rt = Self::Rt>
    where
        F: FnOnce(Self::Lt) -> U;

    fn map_right<F, U>(self, f: F) -> impl TrEitherOf<Lt = Self::Lt, Rt = U>
    where
        F: FnOnce(Self::Rt) -> U;

    fn as_ref<'a>(&'a self) -> impl TrEitherOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a;

    fn as_mut<'a>(&'a mut self) -> impl TrEitherOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a;
}

#[derive(Clone, Debug)]
pub enum EitherOf<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> EitherOf<L, R> {
    pub const fn new_left(l: L) -> Self {
        EitherOf::Left(l)
    }

    pub const fn new_right(r: R) -> Self {
        EitherOf::Right(r)
    }

    pub fn split(self) -> (Option<L>, Option<R>) {
        match self {
            EitherOf::Left(l) => (Option::Some(l), Option::None),
            EitherOf::Right(r) => (Option::None, Option::Some(r)),
        }
    }

    /// Maps Either<L, R> to Either<T, R>
    pub fn map_left<F, T>(self, f: F) -> EitherOf<T, R>
    where
        F: FnOnce(L) -> T,
    {
        match self {
            EitherOf::Left(l) => EitherOf::new_left(f(l)),
            EitherOf::Right(r) => EitherOf::new_right(r),
        }
    }

    /// Maps Either<L, R> to Either<L, T>
    pub fn map_right<F, T>(self, f: F) -> EitherOf<L, T>
    where
        F: FnOnce(R) -> T,
    {
        match self {
            EitherOf::Left(l) => EitherOf::<L, T>::Left(l),
            EitherOf::Right(r) => EitherOf::<L, T>::Right(f(r)),
        }
    }

    pub fn take_left(self) -> EitherOf<L, Self> {
        match self {
            EitherOf::Left(l) => EitherOf::new_left(l),
            EitherOf::Right(r) => EitherOf::new_right(EitherOf::new_right(r)),
        }
    }

    pub fn take_right(self) -> EitherOf<R, Self> {
        match self {
            EitherOf::Left(l) => EitherOf::Right(EitherOf::new_left(l)),
            EitherOf::Right(r) => EitherOf::new_left(r),
        }
    }

    pub const fn as_ref(&self) -> EitherOf<&L, &R> {
        match self {
            EitherOf::Left(l) => EitherOf::Left(l),
            EitherOf::Right(r) => EitherOf::Right(r)
        }
    }

    pub const fn as_mut(&mut self) -> EitherOf<&mut L, &mut R> {
        match self {
            EitherOf::Left(l) => EitherOf::Left(l),
            EitherOf::Right(r) => EitherOf::Right(r)
        }
    }

    pub fn left(self) -> Option<L> {
        if let EitherOf::Left(l) = self {
            Option::Some(l)
        } else {
            Option::None
        }
    }

    pub fn right(self) -> Option<R> {
        if let EitherOf::Right(r) = self {
            Option::Some(r)
        } else {
            Option::None
        }
    }

    pub fn reverse(self) -> EitherOf<R, L> {
        match self {
            EitherOf::Left(x) => EitherOf::<R, L>::Right(x),
            EitherOf::Right(x) => EitherOf::<R, L>::Left(x),
        }
    }

    pub const fn is_left(&self) -> bool {
        match self {
            EitherOf::Left(_) => true,
            _ => false,
        }
    }

    pub const fn is_right(&self) -> bool {
        match self {
            EitherOf::Right(_) => true,
            _ => false,
        }
    }

    pub fn is_left_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&L) -> bool,
    {
        if let EitherOf::Left(x) = self {
            f(x)
        } else {
            false
        }
    }

    pub fn is_right_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&R) -> bool,
    {
        if let EitherOf::Right(x) = self {
            f(x)
        } else {
            false
        }
    }
}

impl<T> EitherOf<T, T> {
    pub fn into_inner(self) -> T {
        match self {
            EitherOf::Left(x) => x,
            EitherOf::Right(x) => x,
        }
    }
}

impl<L> From<Option<L>> for EitherOf<L, ()> {
    fn from(value: Option<L>) -> Self {
        match value {
            Option::Some(left) => EitherOf::Left(left),
            Option::None => EitherOf::Right(()),
        }
    }
}

impl<T, E> From<Result<T, E>> for EitherOf<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Result::Ok(t) => EitherOf::Left(t),
            Result::Err(e) => EitherOf::Right(e),
        }
    }
}

impl<L, R> TrInverseLR for EitherOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn into_inversed(self) -> impl TrInverseLR<Lt = Self::Rt, Rt = Self::Lt> {
        EitherOf::reverse(self)
    }
}

impl<L, R> TrAnyOf for EitherOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn into_any_of(self) -> crate::AnyOf<Self::Lt, Self::Rt> {
        match self {
            EitherOf::Left(l) => AnyOf::new_left(l),
            EitherOf::Right(r) => AnyOf::new_right(r),
        }
    }

    #[inline]
    fn map_left<F, T>(self, f: F) -> impl TrAnyOf<Lt = T, Rt = Self::Rt >
    where
        F: FnOnce(Self::Lt) -> T,
    {
        EitherOf::map_left(self, f)
    }

    #[inline]
    fn map_right<F, T>(self, f: F) -> impl TrAnyOf<Lt = Self::Lt, Rt = T>
    where
        F: FnOnce(Self::Rt) -> T,
    {
        EitherOf::map_right(self, f)
    }

    #[inline]
    fn take_left(self) -> SomeOf<Self::Lt, Self>
    where
        Self: Sized
    {
        EitherOf::take_left(self).into()
    }

    #[inline]
    fn take_right(self) -> SomeOf<Self::Rt, Self>
    where
        Self: Sized
    {
        EitherOf::take_right(self).into()
    }

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        EitherOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        EitherOf::as_mut(self)
    }

    #[inline]
    fn contains_left(&self) -> bool {
        EitherOf::is_left(self)
    }

    #[inline]
    fn contains_right(&self) -> bool {
        EitherOf::is_right(self)
    }
}

impl<L: Copy, R: Copy> Copy for EitherOf<L, R>
{ }