use crate::{
    abs::{TrAnyOf, TrInverseLR},
    AnyOf, SomeOf, TrSomeOf,
};

pub struct BothOf<L, R>(BothLR<L, R>);

impl<L, R> BothOf<L, R> {
    pub const fn new(l: L, r: R) -> Self {
        BothOf((l, r))
    }

    #[inline]
    pub fn into_inversed(self) -> BothOf<R, L> {
        (self.0.1, self.0.0).into()
    }

    #[inline]
    pub fn into_any_of(self) -> AnyOf<L, R> {
        let (l, r) = self.0;
        AnyOf::new_both(l, r)
    }

    #[inline]
    pub fn into_some_of(self) -> SomeOf<L, R> {
        SomeOf::new_both(self.0.0, self.0.1)
    }

    #[inline]
    pub fn into_inner(self) -> BothLR<L, R> {
        self.0
    }

    #[inline]
    pub const fn as_ref(&self) -> BothOf<&L, &R> {
        BothOf::new(&self.0.0, &self.0.1)
    }

    #[inline]
    pub const fn as_mut(&mut self) -> BothOf<&mut L, &mut R> {
        BothOf::new(&mut self.0.0, &mut self.0.1)
    }
}

impl<L, R> From<BothLR<L, R>> for BothOf<L, R> {
    fn from(value: (L, R)) -> Self {
        BothOf::<L, R>::new(value.0, value.1)
    }
}

impl<L, R> TrInverseLR for BothOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn into_inversed(self) -> impl TrInverseLR<Lt = Self::Rt, Rt = Self::Lt> {
        BothOf::into_inversed(self)
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
        BothOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrSomeOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        BothOf::as_mut(self)
    }

    #[inline]
    fn into_some_of(self) -> SomeOf<Self::Lt, Self::Rt> {
        BothOf::into_some_of(self)
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
        BothOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        BothOf::as_mut(self)
    }

    #[inline]
    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt> {
        BothOf::into_any_of(self)
    }
}

pub type BothLR<L, R> = (L, R);

impl<L, R> TrInverseLR for BothLR<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn into_inversed(self) -> impl TrInverseLR<Lt = Self::Rt, Rt = Self::Lt> {
        (self.1, self.0)
    }
}
