use crate::{
    any_of::AnyLR,
    commutative::{CommutativeVariant, TrCommutative},
    AnyOf, BothOf, EitherOf, TrAnyOf,
};

pub trait TrSomeOf
where
    Self: TrCommutative<Left = Self::Lt, Right = Self::Rt>,
{
    type Lt;
    type Rt;

    type Ref<'f>: TrSomeOf<Lt = &'f Self::Lt, Rt = &'f Self::Rt>
    where
        Self: 'f,
        Self::Lt: 'f,
        Self::Rt: 'f;

    type Mut<'f>: TrSomeOf<Lt = &'f mut Self::Lt, Rt = &'f mut Self::Rt>
    where
        Self: 'f,
        Self::Lt: 'f,
        Self::Rt: 'f;

    // Required methods

    fn as_ref<'f>(&'f self) -> Self::Ref<'f>
    where
        Self::Lt: 'f,
        Self::Rt: 'f;

    fn as_mut<'f>(&'f mut self) -> Self::Mut<'f>
    where
        Self::Lt: 'f,
        Self::Rt: 'f;

    fn into_some_of(self) -> SomeOf<Self::Lt, Self::Rt>;

    // Provided methods

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
        match self.as_ref().pick_left() {
            Option::Some(l) => f(l),
            _ => false,
        }
    }

    fn contains_right_and<F>(&self, f: F) -> bool
    where
        F: FnOnce(&Self::Rt) -> bool,
    {
        match self.as_ref().pick_right() {
            Option::Some(r) => f(r),
            _ => false,
        }
    }

    fn map_left<F, U>(self, f: F) -> SomeOf<U, Self::Rt>
    where
        Self: Sized,
        F: FnOnce(Self::Lt) -> U,
    {
        self.into_some_of().map_left(f)
    }

    fn map_right<F, U>(self, f: F) -> SomeOf<Self::Lt, U>
    where
        Self: Sized,
        F: FnOnce(Self::Rt) -> U,
    {
        self.into_some_of().map_right(f)
    }

    fn pick_left(self) -> Option<Self::Lt>
    where
        Self: Sized
    {
        self.into_some_of().pick_left()
    }

    fn pick_right(self) -> Option<Self::Rt>
    where
        Self: Sized
    {
        self.into_some_of().pick_right()
    }
}

/// An wrapper around `SomeLR<L, R>`. This is to avoid misunderstanding for
/// the semantic APIs like `TrAnyLeftRight::map_left`. In detail, the variant
/// of `SomeLR::Both` should returns true for both `is_left` and `is_right`.
#[derive(Clone, Debug)]
pub struct SomeOf<L, R>(SomeLR<L, R>);

impl<L, R> SomeOf<L, R> {
    pub const fn new(inner: SomeLR<L, R>) -> Self {
        SomeOf(inner)
    }

    pub const fn new_left(l: L) -> Self {
        SomeOf(SomeLR::Left(l))
    }

    pub const fn new_right(r: R) -> Self {
        SomeOf(SomeLR::Right(r))
    }

    pub const fn new_both(l: L, r: R) -> Self {
        SomeOf(SomeLR::Both(l, r))
    }

    pub fn into_some_of(self) -> SomeOf<L, R> {
        self
    }

    pub fn into_any_of(self) -> AnyOf<L, R> {
        match self.0 {
            SomeLR::Left(l) => AnyOf::new_left(l),
            SomeLR::Right(r) => AnyOf::new_right(r),
            SomeLR::Both(l, r) => AnyOf::new_both(l, r),
        }
    }

    pub fn into_commutated(self) -> SomeOf<R, L> {
        SomeOf(self.0.into_commutated())
    }

    pub fn as_ref(&self) -> SomeOf<&L, &R> {
        match &self.0 {
            SomeLR::Left(l) => SomeOf::new_left(l),
            SomeLR::Right(r) => SomeOf::new_right(r),
            SomeLR::Both(l, r) => SomeOf::new_both(l, r)
        }
    }

    pub fn as_mut(&mut self) -> SomeOf<&mut L, &mut R> {
        match &mut self.0 {
            SomeLR::Left(l) => SomeOf::new_left(l),
            SomeLR::Right(r) => SomeOf::new_right(r),
            SomeLR::Both(l, r) => SomeOf::new_both(l, r)
        }
    }

    /// The variant is `SomeOf::Left` or `SomeOf::Both`
    pub fn contains_left(&self) -> bool {
        self.0.contains_left()
    }

    /// The variant is `SomeOf::Right` or `SomeOf::Both`
    pub fn contains_right(&self) -> bool {
        self.0.contains_right()
    }

    /// The variant is just `SomeOf::Both`
    pub fn contains_both(&self) -> bool {
        self.0.contains_both()
    }

    pub fn map_left<F, U>(self, f: F) -> SomeOf<U, R>
    where
        F: FnOnce(L) -> U,
    {
        SomeOf(self.0.map_left(f))
    }

    pub fn map_right<F, U>(self, f: F) -> SomeOf<L, U>
    where
        F: FnOnce(R) -> U,
    {
        SomeOf(self.0.map_right(f))
    }

    pub fn pick_left(self) -> Option<L> {
        match self.0 {
            SomeLR::Left(l) => Option::Some(l),
            SomeLR::Both(l, _ ) => Option::Some(l),
            _ => Option::None,
        }
    }

    pub fn pick_right(self) -> Option<R> {
        match self.0 {
            SomeLR::Right(r) => Option::Some(r),
            SomeLR::Both(_, r) => Option::Some(r),
            _ => Option::None,
        }
    }

    pub fn into_inner(self) -> SomeLR<L, R> {
        self.0
    }
}

impl<L, R> From<SomeLR<L, R>> for SomeOf<L, R> {
    fn from(value: SomeLR<L, R>) -> Self {
        SomeOf::new(value)
    }
}

impl<L, R> From<EitherOf<L, R>> for SomeOf<L, R> {
    fn from(value: EitherOf<L, R>) -> Self {
        match value {
            EitherOf::Left(x) => SomeOf::new_left(x),
            EitherOf::Right(x) => SomeOf::new_right(x),
        }
    }
}

impl<L, R> From<BothOf<L, R>> for SomeOf<L, R> {
    fn from(value: BothOf<L, R>) -> Self {
        let (l, r) = value.split();
        SomeOf::new_both(l, r)
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

impl<L, R> TryFrom<AnyOf<L, R>> for SomeOf<L, R> {
    type Error = AnyOf<L, R>;

    fn try_from(value: AnyOf<L, R>) -> Result<Self, AnyOf<L, R>> {
        match value.into_inner() {
            AnyLR::Both(l, r) => Result::Ok(SomeOf::new_both(l, r)),
            AnyLR::Left(l) => Result::Ok(SomeOf::new_left(l)),
            AnyLR::Right(r) => Result::Ok(SomeOf::new_right(r)),
            _ => Result::Err(AnyOf::new_neither()),
        }
    }
}

impl<L, R> TrCommutative for SomeOf<L, R> {
    type Left = L;
    type Right = R;
    type Commutated = SomeOf<R, L>;

    #[inline]
    fn into_commutated(self) -> Self::Commutated {
        SomeOf::into_commutated(self)
    }

    #[inline]
    fn into_variant(self) -> CommutativeVariant<Self::Left, Self::Right> {
        CommutativeVariant::Some(self)
    }
}

impl<L, R> TrSomeOf for SomeOf<L, R> {
    type Lt = L;
    type Rt = R;

    type Ref<'f> = SomeOf<&'f Self::Lt, &'f Self::Rt>
    where
        Self: 'f,
        Self::Lt: 'f,
        Self::Rt: 'f;

    type Mut<'f> = SomeOf<&'f mut Self::Lt, &'f mut Self::Rt>
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
        SomeOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'f>(&'f mut self) -> Self::Mut<'f>
    where
        Self::Lt: 'f,
        Self::Rt: 'f,
    {
        SomeOf::as_mut(self)
    }

    #[inline]
    fn into_some_of(self) -> SomeOf<Self::Lt, Self::Rt> {
        SomeOf::into_some_of(self)
    }
}

impl<L, R> TrAnyOf for SomeOf<L, R> {
    type Lt = L;
    type Rt = R;

    type Ref<'f> = SomeOf<&'f Self::Lt, &'f Self::Rt>
    where
        Self: 'f,
        Self::Lt: 'f,
        Self::Rt: 'f;

    type Mut<'f> = SomeOf<&'f mut Self::Lt, &'f mut Self::Rt>
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
        SomeOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'f>(&'f mut self) -> Self::Mut<'f>
    where
        Self::Lt: 'f,
        Self::Rt: 'f,
    {
        SomeOf::as_mut(self)
    }

    #[inline]
    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt> {
        SomeOf::into_any_of(self)
    }
}

impl<L: Copy, R: Copy> Copy for SomeOf<L, R>
{}

/// The raw value of `SomeOf<L, R>`.
#[derive(Clone, Debug)]
pub enum SomeLR<L, R> {
    Left(L),
    Right(R),
    Both(L, R),
}

impl<L, R> SomeLR<L, R> {
    pub fn into_any_of(self) -> AnyOf<L, R> {
        match self {
            SomeLR::Left(l) => AnyOf::new_left(l),
            SomeLR::Right(r) => AnyOf::new_right(r),
            SomeLR::Both(l, r) => AnyOf::new_both(l, r),
        }
    }

    pub fn into_some_of(self) -> SomeOf<L, R> {
        SomeOf(self)
    }

    pub fn into_commutated(self) -> SomeLR<R, L> {
        match self {
            SomeLR::Left(l) => SomeLR::Right(l),
            SomeLR::Right(r) => SomeLR::Left(r),
            SomeLR::Both(l, r) => SomeLR::Both(r, l),
        }
    }

    pub const fn as_ref(&self) -> SomeLR<&L, &R> {
        match self {
            SomeLR::Left(l) => SomeLR::Left(l),
            SomeLR::Right(r) => SomeLR::Right(r),
            SomeLR::Both(l, r ) => SomeLR::Both(l, r),
        }
    }

    pub const fn as_mut(&mut self) -> SomeLR<&mut L, &mut R> {
        match self {
            SomeLR::Left(l) => SomeLR::Left(l),
            SomeLR::Right(r) => SomeLR::Right(r),
            SomeLR::Both(l, r ) => SomeLR::Both(l, r),
        }
    }

    pub(crate) fn map_left<F, T>(self, f: F) -> SomeLR<T, R>
    where
        F: FnOnce(L) -> T,
    {
        match self {
            SomeLR::Left(l) => SomeLR::Left(f(l)),
            SomeLR::Right(r) => SomeLR::Right(r),
            SomeLR::Both(l, r) => SomeLR::Both(f(l), r),
        }
    }

    pub(crate) fn map_right<F, T>(self, f: F) -> SomeLR<L, T>
    where
        F: FnOnce(R) -> T,
    {
        match self {
            SomeLR::Left(l) => SomeLR::Left(l),
            SomeLR::Right(r) => SomeLR::Right(f(r)),
            SomeLR::Both(l, r) => SomeLR::Both(l, f(r)),
        }
    }

    /// The variant is `SomeOf::Left` or `SomeOf::Both`
    pub(crate) fn contains_left(&self) -> bool {
        matches!(self, SomeLR::Left(_) | SomeLR::Both(_, _))
    }

    /// The variant is `SomeOf::Right` or `SomeOf::Both`
    pub(crate) fn contains_right(&self) -> bool {
        matches!(self, SomeLR::Right(_) | SomeLR::Both(_, _))
    }

    /// The variant is just `SomeOf::Both`
    pub(crate) fn contains_both(&self) -> bool {
        matches!(self, SomeLR::Both(_, _))
    }
}

impl<L: Copy, R: Copy> Copy for SomeLR<L, R>
{ }

impl<L, R> TrCommutative for SomeLR<L, R> {
    type Left = L;
    type Right = R;
    type Commutated = SomeLR<R, L>;

    #[inline]
    fn into_commutated(self) -> Self::Commutated {
        SomeLR::into_commutated(self)
    }

    #[inline]
    fn into_variant(self) -> CommutativeVariant<L, R> {
        CommutativeVariant::Some(self.into_some_of())
    }
}
