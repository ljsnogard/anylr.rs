use crate::{
    abs::{TrAnyOf, TrInverseLR},
    BothOf, EitherOf, SomeOf,
};

/// A combination of zero or one, `L` and `R`
#[derive(Clone, Debug)]
pub struct AnyOf<L, R>(AnyLR<L, R>);

impl<L, R> AnyOf<L, R> {
    /// Wraps value of `L` with Any<L, R>.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use anylr::Any;
    /// 
    /// let a = Any::<usize, f64>::new_left(0usize);
    /// assert!(a.contains_left());
    /// assert!(!a.contains_right());
    /// ```
    pub const fn new_left(l: L) -> Self {
        AnyOf(AnyLR::Left(l))
    }

    /// Wraps value of `R` with Any<L, R>.
    ///
    /// # Examples
    /// 
    /// ```
    /// use anylr::Any;
    ///
    /// let a = Any::<usize, f64>::new_right(0.0f64);
    /// assert!(!a.contains_left());
    /// assert!(a.contains_right());
    /// ```
    pub const fn new_right(r: R) -> Self {
        AnyOf(AnyLR::Right(r))
    }

    /// Wraps a pair of values of type `L` and `R` with Any<L, R>.
    ///
    /// # Examples
    /// 
    /// ```
    /// use anylr::Any;
    ///
    /// let a = Any::<usize, f64>::new_both(0usize, 0.0f64);
    /// assert!(a.contains_left());
    /// assert!(a.contains_right());
    /// ```
    pub const fn new_both(l: L, r: R) -> Self {
        AnyOf(AnyLR::Both((l, r,)))
    }

    /// Creates a value of `Any<L, R>` that contains no values.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use anylr::Any;
    ///
    /// let a = Any::<usize, f64>::new_neither();
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
    /// use anylr::Any;
    ///
    /// let l = 0usize;
    /// let r = 0.0f64;
    /// let a = Any::new_both(l, r);
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
            AnyLR::Both((l, r,)) => (Option::Some(l), Option::Some(r)),
        }
    }

    /// Makes a `Any<L, R>` to `Any<U, R>` by applying a function to a containing
    /// `Left` value of type `L`, leaving `Right` value of type `R` untouched.
    #[inline]
    pub fn map_left<F, U>(self, f: F) -> AnyOf<U, R>
    where
        F: FnOnce(L) -> U,
    {
        AnyOf(self.0.map_left(f))
    }

    /// Makes a `Any<L, R>` to `Any<L, U>` by applying a function to a containing
    /// `Right` value of type `R`, leaving `Left` value of type `L` untouched.
    #[inline]
    pub fn map_right<F, U>(self, f: F) -> AnyOf<L, U>
    where
        F: FnOnce(R) -> U,
    {
        AnyOf(self.0.map_right(f))
    }

    pub fn take_left(self) -> SomeOf<L, Self> {
        match self.0 {
            AnyLR::Neither => SomeOf::new_right(AnyOf::new_neither()),
            AnyLR::Left(l) => SomeOf::new_both(l, AnyOf::new_neither()),
            AnyLR::Right(r) => SomeOf::new_right(AnyOf::new_right(r)),
            AnyLR::Both((l, r,)) => SomeOf::new_both(l, AnyOf::new_right(r)),
        }
    }

    pub fn take_right(self) -> SomeOf<R, Self> {
        match self.0 {
            AnyLR::Neither => SomeOf::new_right(AnyOf::new_neither()),
            AnyLR::Left(l) => SomeOf::new_right(AnyOf::new_left(l)),
            AnyLR::Right(r) => SomeOf::new_left(r),
            AnyLR::Both((l, r,)) => SomeOf::new_both(r, AnyOf::new_left(l)),
        }
    }

    pub fn reverse(self) -> AnyOf<R, L> {
        AnyOf(self.0.reverse())
    }

    pub fn as_ref(&self) -> AnyOf<&L, &R> {
        match &self.0 {
            AnyLR::Neither => AnyOf::new_neither(),
            AnyLR::Left(l) => AnyOf::new_left(l),
            AnyLR::Right(r) => AnyOf::new_right(r),
            AnyLR::Both((l, r,)) => AnyOf::new_both(l, r),
        }
    }

    pub fn as_mut(&mut self) -> AnyOf<&mut L, &mut R> {
        match &mut self.0 {
            AnyLR::Neither => AnyOf::new_neither(),
            AnyLR::Left(l) => AnyOf::new_left(l),
            AnyLR::Right(r) => AnyOf::new_right(r),
            AnyLR::Both((l, r,)) => AnyOf::new_both(l, r),
        }
    }

    pub fn contains_left(&self) -> bool {
        match self.0 {
            AnyLR::Left(_) => true,
            AnyLR::Both(_) => true,
            _ => false,
        }
    }

    pub fn contains_right(&self) -> bool {
        match self.0 {
            AnyLR::Right(_) => true,
            AnyLR::Both(_) => true,
            _ => false,
        }
    }

    pub fn is_both(&self) -> bool {
        matches!(self.0, AnyLR::Both(_))
    }

    pub fn is_neither(&self) -> bool {
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
        AnyOf::new_both(value.0, value.1)
    }
}

impl<L, R> From<SomeOf<L, R>> for AnyOf<L, R> {
    fn from(value: SomeOf<L, R>) -> Self {
        value.into_any_of()
    }
}

impl<L, R> From<(Option<L>, Option<R>,)> for AnyOf<L, R> {
    fn from(value: (Option<L>, Option<R>)) -> Self {
        match value {
            (Option::Some(l), Option::Some(r)) => AnyOf::new_both(l, r,),
            (Option::Some(l), Option::None) => AnyOf::new_left(l),
            (Option::None, Option::Some(r)) => AnyOf::new_right(r),
            (Option::None, Option::None) => AnyOf::new_neither(),
        }
    }
}

impl<L, R> TrInverseLR for AnyOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn into_inversed(self) -> impl TrInverseLR<Lt = Self::Rt, Rt = Self::Lt> {
        AnyOf::reverse(self)
    }
}

impl<L, R> TrAnyOf for AnyOf<L, R> {
    type Lt = L;
    type Rt = R;

    #[inline]
    fn into_any_of(self) -> AnyOf<Self::Lt, Self::Rt> {
        self
    }

    #[inline]
    fn map_left<F, T>(self, f: F) -> impl TrAnyOf<Lt = T, Rt = Self::Rt >
    where
        F: FnOnce(Self::Lt) -> T,
    {
        AnyOf::map_left(self, f)
    }

    #[inline]
    fn map_right<F, T>(self, f: F) -> impl TrAnyOf<Lt = Self::Lt, Rt = T>
    where
        F: FnOnce(Self::Rt) -> T,
    {
        AnyOf::map_right(self, f)
    }

    #[inline]
    fn take_left(self) -> SomeOf<L, Self> {
        AnyOf::take_left(self)
    }

    #[inline]
    fn take_right(self) -> SomeOf<R, Self> {
        AnyOf::take_right(self)
    }

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
}

impl<L: Copy, R: Copy> Copy for AnyOf<L, R>
{}

#[derive(Clone, Debug)]
pub enum AnyLR<L, R> {
    Neither,
    Left(L),
    Right(R),
    Both(BothOf<L, R>),
}

impl<L, R> AnyLR<L, R> {
    pub fn into_any_of(self) -> AnyOf<L, R> {
        AnyOf(self)
    }

    pub fn reverse(self) -> AnyLR<R, L> {
        match self {
            AnyLR::Neither => AnyLR::Neither,
            AnyLR::Left(x) => AnyLR::Right(x),
            AnyLR::Right(x) => AnyLR::Left(x),
            AnyLR::Both((l, r,)) => AnyLR::Both((r, l,)),
        }
    }

    pub(crate) fn map_left<F, U>(self, f: F) -> AnyLR<U, R>
    where
        F: FnOnce(L) -> U,
    {
        match self {
            AnyLR::Left(l) => AnyLR::Left(f(l)),
            AnyLR::Both((l, r,)) => AnyLR::Both((f(l), r,)),
            _ => AnyLR::Neither,
        }
    }

    pub(crate) fn map_right<F, U>(self, f: F) -> AnyLR<L, U>
    where
        F: FnOnce(R) -> U,
    {
        match self {
            AnyLR::Right(r) => AnyLR::Right(f(r)),
            AnyLR::Both((l, r,)) => AnyLR::Both((l, f(r),)),
            _ => AnyLR::Neither,
        }
    }
}

impl<L: Copy, R: Copy> Copy for AnyLR<L, R>
{}