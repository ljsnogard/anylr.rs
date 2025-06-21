use crate::{
    abs::{TrAnyOf, TrInverseLR},
    AnyOf, SomeOf,
};

pub type BothOf<L, R> = (L, R);

impl<L, R> TrInverseLR for (L, R) {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn into_inversed(self) -> impl TrInverseLR<Lt = Self::Rt, Rt = Self::Lt> {
        (self.1, self.0)
    }
}

impl<L, R> TrAnyOf for BothOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt> {
        AnyOf::new_both(self.0, self.1)
    }

    #[inline]
    fn map_left<F, T>(self, f: F) -> impl TrAnyOf<Lt = T, Rt = Self::Rt >
    where
        F: FnOnce(Self::Lt) -> T,
    {
        let (l, r) = self;
        (f(l), r)
    }

    #[inline]
    fn map_right<F, T>(self, f: F) -> impl TrAnyOf<Lt = Self::Lt, Rt = T>
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
    fn as_ref<'a>(&'a self) -> impl TrAnyOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        (&self.0, &self.1)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        (&mut self.0, &mut self.1)
    }
}
