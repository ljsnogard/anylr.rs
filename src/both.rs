use crate::{
    abs::{TrAnyLeftRight, TrReverseLeftRight},
    SomeOf,
};

impl<L, R> TrReverseLeftRight for (L, R) {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn reverse(self) -> impl TrReverseLeftRight<Lt = Self::Rt, Rt = Self::Lt> {
        (self.1, self.0)
    }
}

impl<L, R> TrAnyLeftRight for (L, R) {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn split(self) -> (Option<Self::Lt>, Option<Self::Rt>) {
        (Option::Some(self.0), Option::Some(self.1))
    }

    #[inline]
    fn map_left<F, T>(self, f: F) -> impl TrAnyLeftRight<Lt = T, Rt = Self::Rt >
    where
        F: FnOnce(Self::Lt) -> T,
    {
        let (l, r) = self;
        (f(l), r)
    }

    #[inline]
    fn map_right<F, T>(self, f: F) -> impl TrAnyLeftRight<Lt = Self::Lt, Rt = T>
    where
        F: FnOnce(Self::Rt) -> T,
    {
        let (l, r) = self;
        (l, f(r))
    }

    fn take_left(self) -> SomeOf<L, Self>
    where
        Self: Sized
    {
        SomeOf::new_left(self.0)
    }

    fn take_right(self) -> SomeOf<R, Self>
    where
        Self: Sized
    {
        SomeOf::new_left(self.1)
    }

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyLeftRight<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        (&self.0, &self.1)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyLeftRight<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        (&mut self.0, &mut self.1)
    }
}
