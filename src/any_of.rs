use crate::{
    commutative::{CommutativeVariant, TrCommutative},
    BothOf, EitherOf, SomeOf,
};

/// Trait for types that may contain zero or more variants among left type and right type.
pub trait TrAnyOf
where
    Self: TrCommutative<Left = Self::Lt, Right = Self::Rt>,
{
    type Lt;
    type Rt;

    // Required methods

    fn as_ref<'a>(&'a self) -> impl TrAnyOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a;

    fn as_mut<'a>(&'a mut self) -> impl TrAnyOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a;

    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt>;

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

    fn map_left<F, U>(self, f: F) -> AnyOf<U, Self::Rt>
    where
        Self: Sized,
        F: FnOnce(Self::Lt) -> U,
    {
        self.into_any_of().map_left(f)
    }

    fn map_right<F, U>(self, f: F) -> AnyOf<Self::Lt, U>
    where
        Self: Sized,
        F: FnOnce(Self::Rt) -> U,
    {
        self.into_any_of().map_right(f)
    }

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
}

/// A combination of zero or one, `L` and `R`
#[derive(Clone, Debug)]
pub struct AnyOf<L, R>(AnyLR<L, R>);

impl<L, R> AnyOf<L, R> {
    pub const fn new(inner: AnyLR<L, R>) -> Self {
        AnyOf(inner)
    }

    /// Wraps value of `L` with AnyOf<L, R>.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use anylr::AnyOf;
    /// 
    /// let a = AnyOf::<usize, f64>::new_left(0usize);
    /// assert!(a.contains_left());
    /// assert!(!a.contains_right());
    /// ```
    pub const fn new_left(l: L) -> Self {
        AnyOf(AnyLR::Left(l))
    }

    /// Wraps value of `R` with AnyOf<L, R>.
    ///
    /// # Examples
    /// 
    /// ```
    /// use anylr::AnyOf;
    ///
    /// let a = AnyOf::<usize, f64>::new_right(0.0f64);
    /// assert!(!a.contains_left());
    /// assert!(a.contains_right());
    /// ```
    pub const fn new_right(r: R) -> Self {
        AnyOf(AnyLR::Right(r))
    }

    /// Wraps a pair of values of type `L` and `R` with AnyOf<L, R>.
    ///
    /// # Examples
    /// 
    /// ```
    /// use anylr::AnyOf;
    ///
    /// let a = AnyOf::<usize, f64>::new_both(0usize, 0.0f64);
    /// assert!(a.contains_left());
    /// assert!(a.contains_right());
    /// ```
    pub const fn new_both(l: L, r: R) -> Self {
        AnyOf(AnyLR::Both(l, r))
    }

    /// Creates a value of `AnyOf<L, R>` that contains no values.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use anylr::AnyOf;
    ///
    /// let a = AnyOf::<usize, f64>::new_neither();
    /// assert!(!a.contains_left());
    /// assert!(!a.contains_right());
    /// ```
    pub const fn new_neither() -> Self {
        AnyOf(AnyLR::Neither)
    }

    /// Creates a tuple that the first element is the optional left value and
    /// the second element is the optional right value.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use anylr::AnyOf;
    ///
    /// let l = 0usize;
    /// let r = 0.0f64;
    /// let a = AnyOf::new_both(l, r);
    /// let t = a.split();
    /// assert_eq!(Option::Some(l), t.0);
    /// assert_eq!(Option::Some(r), t.1);
    /// ```
    #[inline]
    pub fn split(self) -> (Option<L>, Option<R>) {
        match self.0 {
            AnyLR::Neither => (Option::None, Option::None),
            AnyLR::Left(l) => (Option::Some(l), Option::None),
            AnyLR::Right(r) => (Option::None, Option::Some(r)),
            AnyLR::Both(l, r) => (Option::Some(l), Option::Some(r)),
        }
    }

    /// Makes an `AnyOf<L, R>` to `AnyOf<U, R>` by applying a function to a containing
    /// `Left` value of type `L`, leaving `Right` value of type `R` untouched.
    #[inline]
    pub fn map_left<F, U>(self, f: F) -> AnyOf<U, R>
    where
        F: FnOnce(L) -> U,
    {
        let inner = match self.0 {
            AnyLR::Left(l) => AnyLR::Left(f(l)),
            AnyLR::Both(l, r) => AnyLR::Both(f(l), r),
            _ => AnyLR::Neither,
        };
        AnyOf(inner)
    }

    /// Makes an `AnyOf<L, R>` to `AnyOf<L, U>` by applying a function to a containing
    /// `Right` value of type `R`, leaving `Left` value of type `L` untouched.
    #[inline]
    pub fn map_right<F, U>(self, f: F) -> AnyOf<L, U>
    where
        F: FnOnce(R) -> U,
    {
        let inner = match self.0 {
            AnyLR::Right(r) => AnyLR::Right(f(r)),
            AnyLR::Both(l, r) => AnyLR::Both(l, f(r)),
            _ => AnyLR::Neither,
        };
        AnyOf(inner)
    }

    /// Makes an `AnyOf<L, R>` into `Option<L>` as long as a value of `L` is contained.
    ///
    /// # Examples
    /// 
    /// ```
    /// use anylr::AnyOf;
    ///
    /// let l = 0usize;
    /// let r = 0.0f64;
    /// let a = AnyOf::new_both(l, r);
    /// assert_eq!(Option::Some(l), a.clone().pick_left());
    /// assert_eq!(Option::Some(r), a.clone().pick_right());
    /// 
    /// let a = AnyOf::<usize, f64>::new_left(l);
    /// assert_eq!(Option::Some(l), a.clone().pick_left());
    /// assert_eq!(Option::None, a.clone().pick_right());
    /// ```
    #[inline]
    pub fn pick_left(self) -> Option<L> {
        match self.0 {
            AnyLR::Both(l, _) => Option::Some(l),
            AnyLR::Left(l) => Option::Some(l),
            _ => Option::None,
        }
    }

    /// Makes an `AnyOf<L, R>` into `Option<R>` as long as a value of `R` is contained.
    ///
    /// # Examples
    /// 
    /// ```
    /// use anylr::AnyOf;
    ///
    /// let l = 0usize;
    /// let r = 0.0f64;
    /// let a = AnyOf::new_both(l, r);
    /// assert_eq!(Option::Some(r), a.clone().pick_right());
    /// assert_eq!(Option::Some(l), a.clone().pick_left());
    /// 
    /// let a = AnyOf::<usize, f64>::new_right(r);
    /// assert_eq!(Option::Some(r), a.clone().pick_right());
    /// assert_eq!(Option::None, a.clone().pick_left());
    /// ```
    #[inline]
    pub fn pick_right(self) -> Option<R> {
        match self.0 {
            AnyLR::Both(_, r) => Option::Some(r),
            AnyLR::Right(r) => Option::Some(r),
            _ => Option::None,
        }
    }

    pub fn into_commutated(self) -> AnyOf<R, L> {
        AnyOf(self.0.into_commutated())
    }

    pub fn into_variant(self) -> CommutativeVariant<L, R> {
        CommutativeVariant::Any(self)
    }

    pub const fn as_ref(&self) -> AnyOf<&L, &R> {
        match &self.0 {
            AnyLR::Neither => AnyOf::new_neither(),
            AnyLR::Left(l) => AnyOf::new_left(l),
            AnyLR::Right(r) => AnyOf::new_right(r),
            AnyLR::Both(l, r) => AnyOf::new_both(l, r),
        }
    }

    pub const fn as_mut(&mut self) -> AnyOf<&mut L, &mut R> {
        match &mut self.0 {
            AnyLR::Neither => AnyOf::new_neither(),
            AnyLR::Left(l) => AnyOf::new_left(l),
            AnyLR::Right(r) => AnyOf::new_right(r),
            AnyLR::Both(l, r) => AnyOf::new_both(l, r),
        }
    }

    pub const fn contains_left(&self) -> bool {
        matches!(self.0, AnyLR::Left(_) | AnyLR::Both(_, _))
    }

    pub const fn contains_right(&self) -> bool {
        matches!(self.0, AnyLR::Right(_) | AnyLR::Both(_, _))
    }

    pub const fn is_neither(&self) -> bool {
        matches!(self.0, AnyLR::Neither)
    }

    pub fn into_inner(self) -> AnyLR<L, R> {
        self.0
    }
}

impl<L, R> Default for AnyOf<L, R> {
    fn default() -> Self {
        AnyOf::new_neither()
    }
}

impl<L, R> From<AnyLR<L, R>> for AnyOf<L, R> {
    fn from(value: AnyLR<L, R>) -> Self {
        AnyOf::new(value)
    }
}

impl<L, R> From<EitherOf<L, R>> for AnyOf<L, R> {
    fn from(value: EitherOf<L, R>) -> Self {
        match value {
            EitherOf::Left(l) => AnyOf::new_left(l),
            EitherOf::Right(r) => AnyOf::new_right(r),
        }
    }
}

impl<L, R> From<BothOf<L, R>> for AnyOf<L, R> {
    fn from(value: BothOf<L, R>) -> Self {
        let (l, r) = value.split();
        AnyOf::new_both(l, r)
    }
}

impl<L, R> From<SomeOf<L, R>> for AnyOf<L, R> {
    fn from(value: SomeOf<L, R>) -> Self {
        value.into_any_of()
    }
}

impl<L, R> From<BothOf<Option<L>, Option<R>>> for AnyOf<L, R> {
    fn from(value: BothOf<Option<L>, Option<R>>) -> Self {
        match value.split() {
            (Option::Some(l), Option::Some(r)) => AnyOf::new_both(l, r),
            (Option::Some(l), Option::None) => AnyOf::new_left(l),
            (Option::None, Option::Some(r)) => AnyOf::new_right(r),
            (Option::None, Option::None) => AnyOf::new_neither(),
        }
    }
}

impl<L, R> TrCommutative for AnyOf<L, R> {
    type Left = L;
    type Right = R;

    #[inline]
    fn into_commutated(self) -> impl TrCommutative<Left = Self::Right, Right = Self::Left> {
        AnyOf::into_commutated(self)
    }

    #[inline]
    fn into_variant(self) -> CommutativeVariant<Self::Left, Self::Right> {
        AnyOf::into_variant(self)
    }
}

impl<L, R> TrAnyOf for AnyOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        AnyOf::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        AnyOf::as_mut(self)
    }

    #[inline]
    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt> {
        self
    }
}

impl<L: Copy, R: Copy> Copy for AnyOf<L, R>
{}

#[derive(Clone, Debug)]
pub enum AnyLR<L, R> {
    Neither,
    Left(L),
    Right(R),
    Both(L, R),
}

impl<L, R> AnyLR<L, R> {
    #[inline]
    pub fn into_any_of(self) -> AnyOf<L, R> {
        AnyOf(self)
    }

    pub fn into_commutated(self) -> AnyLR<R, L> {
        match self {
            AnyLR::Neither => AnyLR::Neither,
            AnyLR::Left(x) => AnyLR::Right(x),
            AnyLR::Right(x) => AnyLR::Left(x),
            AnyLR::Both(l, r) => AnyLR::Both(r, l,),
        }
    }

    #[inline]
    pub fn into_variant(self) -> CommutativeVariant<L, R> {
        CommutativeVariant::Any(self.into_any_of())
    }

    pub const fn as_ref(&self) -> AnyLR<&L, &R> {
        match self {
            AnyLR::Neither => AnyLR::Neither,
            AnyLR::Left(x) => AnyLR::Left(x),
            AnyLR::Right(x) => AnyLR::Right(x),
            AnyLR::Both(l, r) => AnyLR::Both(l, r),
        }
    }

    pub const fn as_mut(&mut self) -> AnyLR<&mut L, &mut R> {
        match self {
            AnyLR::Neither => AnyLR::Neither,
            AnyLR::Left(x) => AnyLR::Left(x),
            AnyLR::Right(x) => AnyLR::Right(x),
            AnyLR::Both(l, r) => AnyLR::Both(l , r),
        }
    }
}

impl<L: Copy, R: Copy> Copy for AnyLR<L, R>
{}

impl<L, R> TrCommutative for AnyLR<L, R> {
    type Left = L;
    type Right = R;

    #[inline]
    fn into_commutated(self) -> impl TrCommutative<Left = R, Right = L> {
        AnyLR::into_commutated(self)
    }

    #[inline]
    fn into_variant(self) -> CommutativeVariant<L, R> {
        AnyLR::into_variant(self)
    }
}

impl<L, R> TrAnyOf for AnyLR<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyOf<Lt = &'a Self::Lt, Rt = &'a Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        AnyLR::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyOf<Lt = &'a mut Self::Lt, Rt = &'a mut Self::Rt>
    where
        Self::Lt: 'a,
        Self::Rt: 'a,
    {
        AnyLR::as_mut(self)
    }

    #[inline]
    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt> {
        AnyLR::into_any_of(self)
    }
}
