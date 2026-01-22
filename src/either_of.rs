use crate::{
    abs::{TrAnyOf, TrInverseLR},
    AnyLR, AnyOf, SomeOf, SomeLR, TrSomeOf,
};

pub trait TrEitherOf {
    type Lt;
    type Rt;

    // Required methods

    fn as_ref<'a>(&'a self) -> impl TrEitherOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a;

    fn as_mut<'a>(&'a mut self) -> impl TrEitherOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a;

    fn into_either_of(self) -> EitherOf<Self::Lt, Self::Rt>;

    // Provided methods

    fn is_left(&self) -> bool {
        self.as_ref().pick_left().is_some()
    }

    fn is_right(&self) -> bool {
        self.as_ref().pick_right().is_some()
    }

    fn is_left_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&Self::Lt) -> bool,
    {
        match self.as_ref().pick_left() {
            Option::Some(l) => f(l),
            _ => false,
        }
    }

    fn is_right_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&Self::Rt) -> bool,
    {
        match self.as_ref().pick_right() {
            Option::Some(r) => f(r),
            _ => false,
        }
    }

    fn map_left<F, U>(self, f: F) -> EitherOf<U, Self::Rt>
    where
        Self: Sized,
        F: FnOnce(Self::Lt) -> U,
    {
        self.into_either_of().map_left(f)
    }

    fn map_right<F, U>(self, f: F) -> EitherOf<Self::Lt, U>
    where
        Self: Sized,
        F: FnOnce(Self::Rt) -> U,
    {
        self.into_either_of().map_right(f)
    }

    fn pick_left(self) -> Option<Self::Lt>
    where
        Self: Sized,
    {
        self.into_either_of().pick_left()
    }

    fn pick_right(self) -> Option<Self::Rt>
    where
        Self: Sized,
    {
        self.into_either_of().pick_right()
    }
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

    pub fn into_any_of(self) -> AnyOf<L, R> {
        match self {
            EitherOf::Left(l) => AnyOf::new_left(l),
            EitherOf::Right(r) => AnyOf::new_right(r),
        }
    }

    pub fn into_some_of(self) -> SomeOf<L, R> {
        match self {
            EitherOf::Left(l) => SomeOf::new_left(l),
            EitherOf::Right(r) => SomeOf::new_right(r),
        }
    }

    pub fn into_either_of(self) -> EitherOf<L, R> {
        self
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

    /// Maps EitherOf<L, R> to EitherOf<T, R>
    pub fn map_left<F, T>(self, f: F) -> EitherOf<T, R>
    where
        F: FnOnce(L) -> T,
    {
        match self {
            EitherOf::Left(l) => EitherOf::new_left(f(l)),
            EitherOf::Right(r) => EitherOf::new_right(r),
        }
    }

    /// Maps EitherOf<L, R> to EitherOf<L, T>
    pub fn map_right<F, T>(self, f: F) -> EitherOf<L, T>
    where
        F: FnOnce(R) -> T,
    {
        match self {
            EitherOf::Left(l) => EitherOf::<L, T>::Left(l),
            EitherOf::Right(r) => EitherOf::<L, T>::Right(f(r)),
        }
    }

    pub fn pick_left(self) -> Option<L> {
        if let EitherOf::Left(l) = self {
            Option::Some(l)
        } else {
            Option::None
        }
    }

    pub fn pick_right(self) -> Option<R> {
        if let EitherOf::Right(r) = self {
            Option::Some(r)
        } else {
            Option::None
        }
    }

    pub fn into_inversed(self) -> EitherOf<R, L> {
        match self {
            EitherOf::Left(x) => EitherOf::<R, L>::Right(x),
            EitherOf::Right(x) => EitherOf::<R, L>::Left(x),
        }
    }

    pub const fn is_left(&self) -> bool {
        matches!(self, EitherOf::Left(_))
    }

    pub const fn is_right(&self) -> bool {
        matches!(self, EitherOf::Right(_))
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

impl<L, R> TryFrom<SomeOf<L, R>> for EitherOf<L, R> {
    type Error = SomeOf<L, R>;

    fn try_from(value: SomeOf<L, R>) -> Result<Self, Self::Error> {
        match value.into_inner() {
            SomeLR::Left(l) => Result::Ok(EitherOf::Left(l)),
            SomeLR::Right(r) => Result::Ok(EitherOf::Right(r)),
            SomeLR::Both(l, r ) => Result::Err(SomeOf::new_both(l, r)   ),
        }
    }
}

impl<L, R> TryFrom<AnyOf<L, R>> for EitherOf<L, R> {
    type Error = AnyOf<L, R>;

    fn try_from(value: AnyOf<L, R>) -> Result<Self, Self::Error> {
        match value.into_inner() {
            AnyLR::Left(l) => Result::Ok(EitherOf::Left(l)),
            AnyLR::Right(r) => Result::Ok(EitherOf::Right(r)),
            AnyLR::Both(l, r ) => Result::Err(AnyOf::new_both(l, r)),
            AnyLR::Neither => Result::Err(AnyOf::new_neither()),
        }
    }
}

impl<L, R> TrInverseLR for EitherOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn into_inversed(self) -> impl TrInverseLR<Lt = Self::Rt, Rt = Self::Lt> {
        EitherOf::into_inversed(self)
    }
}

impl<L, R> TrEitherOf for EitherOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrEitherOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        EitherOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrEitherOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        EitherOf::as_mut(self)
    }

    #[inline]
    fn into_either_of(self) -> EitherOf<Self::Lt, Self::Rt> {
        EitherOf::into_either_of(self)
    }
}

impl<L, R> TrSomeOf for EitherOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrSomeOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        EitherOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrSomeOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        EitherOf::as_mut(self)
    }

    #[inline]
    fn into_some_of(self) -> SomeOf<Self::Lt, Self::Rt> {
        EitherOf::into_some_of(self)
    }
}

impl<L, R> TrAnyOf for EitherOf<L, R> {
    type Lt = L;
    type Rt = R;

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
    fn into_any_of(self) -> crate::AnyOf<Self::Lt, Self::Rt> {
        match self {
            EitherOf::Left(l) => AnyOf::new_left(l),
            EitherOf::Right(r) => AnyOf::new_right(r),
        }
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