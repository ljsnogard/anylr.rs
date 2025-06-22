use crate::{
    abs::{TrAnyOf, TrInverseLR},
    AnyOf, SomeOf, TrSomeOf,
};

pub type BothOf<L, R> = (L, R);

impl<L, R> TrInverseLR for BothOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn into_inversed(self) -> impl TrInverseLR<Lt = Self::Rt, Rt = Self::Lt> {
        (self.1, self.0)
    }
}

impl<L, R> TrSomeOf for BothOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrSomeOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        (&self.0, &self.1)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrSomeOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        (&mut self.0, &mut self.1)
    }

    #[inline]
    fn into_some_of(self) -> SomeOf<Self::Lt, Self::Rt> {
        SomeOf::new_both(self.0, self.1)
    }
}

impl<L, R> TrAnyOf for BothOf<L, R> {
    type Lt = L;
    type Rt = R;

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

    #[inline]
    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt> {
        AnyOf::new_both(self.0, self.1)
    }
}
