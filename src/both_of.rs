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

    type Ref<'f>: TrBothOf<Lt = &'f Self::Lt, Rt = &'f Self::Rt>
    where
        Self: 'f,
        Self::Lt: 'f,
        Self::Rt: 'f;

    type Mut<'f>: TrBothOf<Lt = &'f mut Self::Lt, Rt = &'f mut Self::Rt>
    where
        Self: 'f,
        Self::Lt: 'f,
        Self::Rt: 'f;

    fn as_ref<'f>(&'f self) -> Self::Ref<'f>
    where
        Self::Lt: 'f,
        Self::Rt: 'f;

    fn as_mut<'f>(&'f mut self) -> Self::Mut<'f>
    where
        Self::Lt: 'f,
        Self::Rt: 'f;

    fn split(self) -> (Self::Lt, Self::Rt);
}

pub struct BothOf<L, R>(L, R);

impl<L, R> BothOf<L, R> {
    pub const fn new(l: L, r: R) -> Self {
        BothOf(l, r)
    }

    #[inline]
    pub fn into_commutated(self) -> BothOf<R, L> {
        (self.1, self.0).into()
    }

    #[inline]
    pub fn into_any_of(self) -> AnyOf<L, R> {
        AnyOf::new_both(self.0, self.1)
    }

    #[inline]
    pub fn into_some_of(self) -> SomeOf<L, R> {
        SomeOf::new_both(self.0, self.1)
    }

    #[inline]
    pub const fn as_ref(&self) -> BothOf<&L, &R> {
        BothOf::new(&self.0, &self.1)
    }

    #[inline]
    pub const fn as_mut(&mut self) -> BothOf<&mut L, &mut R> {
        BothOf::new(&mut self.0, &mut self.1)
    }

    #[inline]
    pub fn split(self) -> (L, R) {
        (self.0, self.1)
    }
}

impl<L, R> TrCommutative for BothOf<L, R> {
    type Left = L;
    type Right = R;
    type Commutated = BothOf<R, L>;

    #[inline]
    fn into_commutated(self) -> Self::Commutated {
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

    type Ref<'f> = BothOf<&'f Self::Lt, &'f Self::Rt>
    where
        Self: 'f,
        Self::Lt: 'f,
        Self::Rt: 'f;

    type Mut<'f> = BothOf<&'f mut Self::Lt, &'f mut Self::Rt>
    where
        Self: 'f,
        Self::Lt: 'f,
        Self::Rt: 'f;

    #[inline]
    fn as_ref<'f>(&'f self) -> Self::Ref<'f>
    where
        Self::Lt: 'f,
        Self::Rt: 'f
    {
        BothOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'f>(&'f mut self) -> Self::Mut<'f>
    where
        Self::Lt: 'f,
        Self::Rt: 'f
    {
        BothOf::as_mut(self)
    }

    #[inline]
    fn split(self) -> (Self::Lt, Self::Rt) {
        BothOf::split(self)
    }
}

impl<L, R> From<(L, R)> for BothOf<L, R> {
    fn from(value: (L, R)) -> Self {
        BothOf::<L, R>::new(value.0, value.1)
    }
}

impl<L, R> TrSomeOf for BothOf<L, R> {
    type Lt = L;
    type Rt = R;

    type Ref<'f> = BothOf<&'f Self::Lt, &'f Self::Rt>
    where
        Self: 'f,
        Self::Lt: 'f,
        Self::Rt: 'f;

    type Mut<'f> = BothOf<&'f mut Self::Lt, &'f mut Self::Rt>
    where
        Self: 'f,
        Self::Lt: 'f,
        Self::Rt: 'f;

    #[inline]
    fn as_ref<'f>(&'f self) -> Self::Ref<'f>
    where
        Self::Lt: 'f,
        Self::Rt: 'f,
    {
        BothOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'f>(&'f mut self) -> Self::Mut<'f>
    where
        Self::Lt: 'f,
        Self::Rt: 'f,
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

    type Ref<'f> = BothOf<&'f L, &'f R>
    where
        Self: 'f,
        Self::Lt: 'f,
        Self::Rt: 'f;

    type Mut<'f> = BothOf<&'f mut L, &'f mut R>
    where
        Self: 'f,
        Self::Lt: 'f,
        Self::Rt: 'f;

    #[inline]
    fn as_ref<'f>(&'f self) -> Self::Ref<'f>
    where
        Self::Lt: 'f,
        Self::Rt: 'f,
    {
        BothOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'f>(&'f mut self) -> Self::Mut<'f>
    where
        Self::Lt: 'f,
        Self::Rt: 'f,
    {
        BothOf::as_mut(self)
    }

    #[inline]
    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt> {
        BothOf::into_any_of(self)
    }
}
