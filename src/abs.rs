use core::convert::Infallible;

use crate::{AnyOf, SomeOf};

pub use crate::{
    either::TrEitherOf,
    some_of::TrSomeOf,
};

/// A trait for types that you can reverse left to right, or the opposite.
pub trait TrInverseLR {
    type Lt;
    type Rt;

    fn into_inversed(self) -> impl TrInverseLR<Lt = Self::Rt, Rt = Self::Lt>;
}

/// Trait for types that may contain zero or more variants among left type and right type.
pub trait TrAnyOf {
    type Lt;
    type Rt;

    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt>;

    fn map_left<F, U>(self, f: F) -> impl TrAnyOf<Lt = U, Rt = Self::Rt>
    where
        F: FnOnce(Self::Lt) -> U;

    fn map_right<F, U>(self, f: F) -> impl TrAnyOf<Lt = Self::Lt, Rt = U>
    where
        F: FnOnce(Self::Rt) -> U;

    fn take_left(self) -> SomeOf<Self::Lt, Self>
    where
        Self: Sized;

    fn take_right(self) -> SomeOf<Self::Rt, Self>
    where
        Self: Sized;

    fn as_ref<'a>(&'a self) -> impl TrAnyOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a;

    fn as_mut<'a>(&'a mut self) -> impl TrAnyOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a;

    // Provided methods

    fn pick_left(self) -> Option<Self::Lt>
    where
        Self: Sized
    {
        self.into_any_of().pick_left()
    }

    fn pick_right(self) -> Option<Self::Rt>
    where
        Self: Sized
    {
        self.into_any_of().pick_right()
    }

    fn contains_left(&self) -> bool {
        self.as_ref().pick_left().is_some()
    }

    fn contains_right(&self) -> bool {
        self.as_ref().pick_right().is_some()
    }

    fn contains_left_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&Self::Lt) -> bool,
    {
        if let Option::Some(l) = self.as_ref().pick_left() {
            f(l)
        } else {
            false
        }
    }

    fn contains_right_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&Self::Rt) -> bool,
    {
        if let Option::Some(r) = self.as_ref().pick_right() {
            f(r)
        } else {
            false
        }
    }
}

impl<T, E> TrAnyOf for Result<T, E> {
    type Lt = T;
    type Rt = E;

    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt> {
        match self {
            Result::Ok(x) => AnyOf::new_left(x),
            Result::Err(e) => AnyOf::new_right(e),
        }
    }

    fn map_left<F, U>(self, f: F) -> impl TrAnyOf<Lt = U, Rt = Self::Rt >
    where
        F: FnOnce(Self::Lt) -> U,
    {
        match self {
            Result::Ok(x) => Result::Ok(f(x)),
            Result::Err(e) => Result::Err(e),
        }
    }

    fn map_right<F, U>(self, f: F) -> impl TrAnyOf<Lt = Self::Lt, Rt = U>
    where
        F: FnOnce(Self::Rt) -> U,
    {
        match self {
            Result::Ok(x) => Result::Ok(x),
            Result::Err(e) => Result::Err(f(e)),
        }
    }

    fn take_left(self) -> SomeOf<T, Self> {
        match self {
            Result::Ok(t) => SomeOf::new_left(t),
            Result::Err(e) => SomeOf::new_right(Result::Err(e)),
        }
    }

    fn take_right(self) -> SomeOf<E, Self> {
        match self {
            Result::Ok(t) => SomeOf::new_right(Result::Ok(t)),
            Result::Err(e) => SomeOf::new_left(e),
        }
    }

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        self.as_ref()
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        self.as_mut()
    }
}

impl<T> TrAnyOf for Option<T> {
    type Lt = T;
    type Rt = Infallible;

    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt> {
        match self {
            Option::Some(x) => AnyOf::new_left(x),
            _ => AnyOf::new_neither(),
        }
    }

    fn map_left<F, U>(self, f: F) -> impl TrAnyOf<Lt = U, Rt = Self::Rt>
    where
        F: FnOnce(Self::Lt) -> U,
    {
        self.map(f)
    }

    fn map_right<F, U>(self, _: F) -> impl TrAnyOf<Lt = Self::Lt, Rt = U>
    where
        F: FnOnce(Self::Rt) -> U
    {
        AnyOf::<Self::Lt, U>::new_neither()
    }

    fn take_left(self) -> SomeOf<Self::Lt, Self>
    where
        Self: Sized
    {
        let Option::Some(t) = self else {
            return SomeOf::new_right(Option::None)
        };
        SomeOf::new_left(t)
    }

    fn take_right(self) -> SomeOf<Self::Rt, Self>
    where
        Self: Sized
    {
        SomeOf::new_right(Option::None)
    }

    fn as_ref<'a>(&'a self) -> impl TrAnyOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        if let Option::Some(t) = self.as_ref() {
            AnyOf::new_left(t)
        } else {
            AnyOf::new_neither()
        }
    }

    fn as_mut<'a>(&'a mut self) -> impl TrAnyOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a
    {
        if let Option::Some(t) = self.as_mut() {
            AnyOf::new_left(t)
        } else {
            AnyOf::new_neither()
        }
    }
}
