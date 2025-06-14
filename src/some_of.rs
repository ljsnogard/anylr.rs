use crate::{
    abs::{TrAnyLeftRight, TrReverseLeftRight},
    Either, Any,
};

/// An wrapper around `SomeLR<L, R>`. This is to avoid misunderstanding for
/// the semantic APIs like `TrAnyLeftRight::map_left`. In detail, the variant
/// of `SomeLR::Both` should returns true for both `is_left` and `is_right`.
#[derive(Clone, Debug)]
pub struct SomeOf<L, R>(SomeLR<L, R>);

impl<L, R> SomeOf<L, R> {
    pub const fn new_left(l: L) -> Self {
        SomeOf(SomeLR::Left(l))
    }

    pub const fn new_right(r: R) -> Self {
        SomeOf(SomeLR::Right(r))
    }

    pub const fn new_both(l: L, r: R) -> Self {
        SomeOf(SomeLR::Both((l, r)))
    }

    pub fn split(self) -> (Option<L>, Option<R>) {
        SomeLR::split(self.0)
    }

    pub fn map_left<F, T>(self, f: F) -> SomeOf<T, R>
    where
        F: FnOnce(L) -> T,
    {
        SomeOf(self.0.map_left(f))
    }

    pub fn map_right<F, T>(self, f: F) -> SomeOf<L, T>
    where
        F: FnOnce(R) -> T,
    {
        SomeOf(self.0.map_right(f))
    }

    pub fn take_left(self) -> SomeOf<L, Self> {
        match self.0 {
            SomeLR::Left(l) => SomeOf::new_left(l),
            SomeLR::Right(r) => SomeOf::new_right(SomeOf::new_right(r)),
            SomeLR::Both((l, r,)) => SomeOf::new_both(l, SomeOf::new_right(r)),
        }
    }

    pub fn take_right(self) -> SomeOf<R, Self> {
        match self.0 {
            SomeLR::Left(l) => SomeOf::new_right(SomeOf::new_left(l)),
            SomeLR::Right(r) => SomeOf::new_left(r),
            SomeLR::Both((l, r,)) => SomeOf::new_both(r, SomeOf::new_left(l)),
        }
    }

    pub fn as_ref(&self) -> SomeOf<&L, &R> {
        match &self.0 {
            SomeLR::Left(l) => SomeOf::new_left(l),
            SomeLR::Right(r) => SomeOf::new_right(r),
            SomeLR::Both((l, r)) => SomeOf::new_both(l, r)
        }
    }

    pub fn as_mut(&mut self) -> SomeOf<&mut L, &mut R> {
        match &mut self.0 {
            SomeLR::Left(l) => SomeOf::new_left(l),
            SomeLR::Right(r) => SomeOf::new_right(r),
            SomeLR::Both((l, r)) => SomeOf::new_both(l, r)
        }
    }

    pub fn reverse(self) -> SomeOf<R, L> {
        SomeOf(self.0.reverse())
    }

    /// The variant is `SomeOf::Left` or `SomeOf::Both`
    pub fn is_left(&self) -> bool {
        self.0.is_left()
    }

    /// The variant is `SomeOf::Right` or `SomeOf::Both`
    pub fn is_right(&self) -> bool {
        self.0.is_right()
    }

    /// The variant is just `SomeOf::Both`
    pub fn is_both(&self) -> bool {
        self.0.is_both()
    }

    pub fn into_inner(self) -> SomeLR<L, R> {
        self.0
    }
}

impl<L, R> From<Either<L, R>> for SomeOf<L, R> {
    fn from(value: Either<L, R>) -> Self {
        match value {
            Either::Left(x) => SomeOf::new_left(x),
            Either::Right(x) => SomeOf::new_right(x),
        }
    }
}

impl<L, R> From<(L, R,)> for SomeOf<L, R> {
    fn from(value: (L, R,)) -> Self {
        SomeOf::new_both(value.0, value.1)
    }
}

impl<T, E> From<Result<T, E>> for SomeOf<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Result::Ok(t) => SomeOf::new_left(t),
            Result::Err(e) => SomeOf::new_right(e),
        }
    }
}

impl<L, R> TryFrom<Any<L, R>> for SomeOf<L, R> {
    type Error = Any<L, R>;

    fn try_from(value: Any<L, R>) -> Result<Self, Any<L, R>> {
        match value.split() {
            (Option::Some(l), Option::Some(r)) => Result::Ok(SomeOf::new_both(l, r)),
            (Option::Some(l), Option::None) => Result::Ok(SomeOf::new_left(l)),
            (Option::None, Option::Some(r)) => Result::Ok(SomeOf::new_right(r)),
            _ => Result::Err(Any::new_neither()),
        }
    }
}

impl<L, R> TrReverseLeftRight for SomeOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn reverse(self) -> impl TrReverseLeftRight<Lt = Self::Rt, Rt = Self::Lt> {
        SomeOf::reverse(self)
    }
}

impl<L, R> TrAnyLeftRight for SomeOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn split(self) -> (Option<Self::Lt>, Option<Self::Rt>) {
        SomeOf::split(self)
    }

    #[inline]
    fn map_left<F, T>(self, f: F) -> impl TrAnyLeftRight<Lt = T, Rt = Self::Rt >
    where
        F: FnOnce(Self::Lt) -> T,
    {
        SomeOf::map_left(self, f)
    }

    #[inline]
    fn map_right<F, T>(self, f: F) -> impl TrAnyLeftRight<Lt = Self::Lt, Rt = T>
    where
        F: FnOnce(Self::Rt) -> T,
    {
        SomeOf::map_right(self, f)
    }

    #[inline]
    fn take_left(self) -> SomeOf<Self::Lt, Self>
    where
        Self: Sized
    {
        SomeOf::take_left(self)
    }

    #[inline]
    fn take_right(self) -> SomeOf<Self::Rt, Self>
    where
        Self: Sized
    {
        SomeOf::take_right(self)
    }

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyLeftRight<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        SomeOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyLeftRight<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        SomeOf::as_mut(self)
    }
}

/// At least one value of type `L` or `R`, or both.
#[derive(Clone, Debug)]
pub enum SomeLR<L, R> {
    Left(L),
    Right(R),
    Both((L, R,)),
}

impl<L, R> SomeLR<L, R> {
    pub fn split(self) -> (Option<L>, Option<R>) {
        match self {
            SomeLR::Left(l) => (Option::Some(l), Option::None),
            SomeLR::Right(r) => (Option::None, Option::Some(r)),
            SomeLR::Both((l, r,)) => (Option::Some(l), Option::Some(r)),
        }
    }

    pub fn reverse(self) -> SomeLR<R, L> {
        match self {
            SomeLR::Left(l) => SomeLR::Right(l),
            SomeLR::Right(r) => SomeLR::Left(r),
            SomeLR::Both((l, r,)) => SomeLR::Both((r, l,)),
        }
    }

    pub(crate) fn map_left<F, T>(self, f: F) -> SomeLR<T, R>
    where
        F: FnOnce(L) -> T,
    {
        match self {
            SomeLR::Left(l) => SomeLR::Left(f(l)),
            SomeLR::Right(r) => SomeLR::Right(r),
            SomeLR::Both((l, r,)) => SomeLR::Both((f(l), r,)),
        }
    }

    pub(crate) fn map_right<F, T>(self, f: F) -> SomeLR<L, T>
    where
        F: FnOnce(R) -> T,
    {
        match self {
            SomeLR::Left(l) => SomeLR::Left(l),
            SomeLR::Right(r) => SomeLR::Right(f(r)),
            SomeLR::Both((l, r,)) => SomeLR::Both((l, f(r),)),
        }
    }

    /// The variant is `SomeOf::Left` or `SomeOf::Both`
    pub(crate) fn is_left(&self) -> bool {
        match self {
            SomeLR::Left(_) => true,
            SomeLR::Both(_) => true,
            _ => false,
        }
    }

    /// The variant is `SomeOf::Right` or `SomeOf::Both`
    pub(crate) fn is_right(&self) -> bool {
        match self {
            SomeLR::Right(_) => true,
            SomeLR::Both(_) => true,
            _ => false,
        }
    }

    /// The variant is just `SomeOf::Both`
    pub(crate) fn is_both(&self) -> bool {
        matches!(self, SomeLR::Both(_))
    }
}