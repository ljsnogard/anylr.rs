use core::convert::Infallible;

use crate::{Any, SomeOf};

/// A trait for types that you can reverse left to right, or the opposite.
pub trait TrReverseLeftRight {
    type Lt;
    type Rt;

    fn reverse(self) -> impl TrReverseLeftRight<Lt = Self::Rt, Rt = Self::Lt>;
}

/// Trait for types that may contain zero or more variants among left type and right type.
pub trait TrAnyLeftRight {
    type Lt;
    type Rt;

    fn split(self) -> (Option<Self::Lt>, Option<Self::Rt>);

    fn map_left<F, U>(self, f: F) -> impl TrAnyLeftRight<Lt = U, Rt = Self::Rt>
    where
        F: FnOnce(Self::Lt) -> U;

    fn map_right<F, U>(self, f: F) -> impl TrAnyLeftRight<Lt = Self::Lt, Rt = U>
    where
        F: FnOnce(Self::Rt) -> U;

    fn take_left(self) -> SomeOf<Self::Lt, Self>
    where
        Self: Sized;

    fn take_right(self) -> SomeOf<Self::Rt, Self>
    where
        Self: Sized;

    fn as_ref<'a>(&'a self) -> impl TrAnyLeftRight<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a;

    fn as_mut<'a>(&'a mut self) -> impl TrAnyLeftRight<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a;

    // Provided methods

    fn pick_left(self) -> Option<Self::Lt>
    where
        Self: Sized
    {
        self.split().0
    }

    fn pick_right(self) -> Option<Self::Rt>
    where
        Self: Sized
    {
        self.split().1
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

impl<T, E> TrAnyLeftRight for Result<T, E> {
    type Lt = T;
    type Rt = E;

    fn split(self) -> (Option<Self::Lt>, Option<Self::Rt>) {
        match self {
            Result::Ok(t) => (Option::Some(t), Option::None),
            Result::Err(e) => (Option::None, Option::Some(e)),
        }
    }

    fn map_left<F, U>(self, f: F) -> impl TrAnyLeftRight<Lt = U, Rt = Self::Rt >
    where
        F: FnOnce(Self::Lt) -> U,
    {
        match self {
            Result::Ok(x) => Result::Ok(f(x)),
            Result::Err(e) => Result::Err(e),
        }
    }

    fn map_right<F, U>(self, f: F) -> impl TrAnyLeftRight<Lt = Self::Lt, Rt = U>
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
    fn as_ref<'a>(&'a self) -> impl TrAnyLeftRight<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        self.as_ref()
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyLeftRight<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        self.as_mut()
    }
}

impl<T> TrAnyLeftRight for Option<T> {
    type Lt = T;
    type Rt = Infallible;

    fn split(self) -> (Option<Self::Lt>, Option<Self::Rt>) {
        (self, Option::None)
    }

    fn map_left<F, U>(self, f: F) -> impl TrAnyLeftRight<Lt = U, Rt = Self::Rt>
    where
        F: FnOnce(Self::Lt) -> U,
    {
        self.map(f)
    }

    fn map_right<F, U>(self, _: F) -> impl TrAnyLeftRight<Lt = Self::Lt, Rt = U>
    where
        F: FnOnce(Self::Rt) -> U
    {
        Any::<Self::Lt, U>::new_neither()
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

    fn as_ref<'a>(&'a self) -> impl TrAnyLeftRight<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        if let Option::Some(t) = self.as_ref() {
            Any::new_left(t)
        } else {
            Any::new_neither()
        }
    }

    fn as_mut<'a>(&'a mut self) -> impl TrAnyLeftRight<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a
    {
        if let Option::Some(t) = self.as_mut() {
            Any::new_left(t)
        } else {
            Any::new_neither()
        }
    }
}
