use crate::{
    commutative::{CommutativeVariant, TrCommutative},
    AnyOf, SomeOf, TrAnyOf, TrSomeOf,
};

pub trait TrBothOf
where
    Self: TrCommutative<Left = Self::Lt, Right = Self::Rt>,
{
    type Lt;
    type Rt;

    fn split(self) -> (Self::Lt, Self::Rt);
}

pub struct BothOf<L, R>(BothLR<L, R>);

impl<L, R> BothOf<L, R> {
    pub const fn new(l: L, r: R) -> Self {
        BothOf((l, r))
    }

    #[inline]
    pub fn into_commutated(self) -> BothOf<R, L> {
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
    pub fn split(self) -> BothLR<L, R> {
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

impl<L, R> TrCommutative for BothOf<L, R> {
    type Left = L;
    type Right = R;

    #[inline]
    fn into_commutated(self) -> impl TrCommutative<Left = Self::Right, Right = Self::Left> {
        BothOf::into_commutated(self)
    }

    #[inline]
    fn into_variant(self) -> CommutativeVariant<Self::Left, Self::Right> {
        CommutativeVariant::Both(self)
    }
}

impl<L, R> TrBothOf for BothOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn split(self) -> (Self::Lt, Self::Rt) {
        BothOf::split(self)
    }
}

impl<L, R> From<BothLR<L, R>> for BothOf<L, R> {
    fn from(value: (L, R)) -> Self {
        BothOf::<L, R>::new(value.0, value.1)
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

impl<L, R> TrCommutative for BothLR<L, R> {
    type Left = L;
    type Right = R;

    #[inline]
    fn into_commutated(self) -> impl TrCommutative<Left = R, Right = L> {
        (self.1, self.0)
    }

    #[inline]
    fn into_variant(self) -> crate::commutative::CommutativeVariant<Self::Left, Self::Right> {
        CommutativeVariant::Both(BothOf::new(self.0, self.1))
    }
}

impl<L, R> TrBothOf for BothLR<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn split(self) -> (Self::Lt, Self::Rt) {
        (self.0, self.1)
    }
}

impl<L, R> TrSomeOf for BothLR<L, R> {
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

impl<L, R> TrAnyOf for BothLR<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyOf<Lt = &'a L, Rt = &'a R>
    where
        Self::Left: 'a,
        Self::Right: 'a,
    {
        (&self.0, &self.1)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyOf<Lt = &'a mut L, Rt = &'a mut R>
    where
        Self::Left: 'a,
        Self::Right: 'a,
    {
        (&mut self.0, &mut self.1)
    }

    #[inline]
    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt> {
        AnyOf::new_both(self.0, self.1)
    }
}
