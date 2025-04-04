use crate::abs::{TrAnyLeftRight, TrReverseLeftRight};

#[derive(Clone, Debug)]
pub struct Both<L, R> {
    pub left: L,
    pub right: R,
}

impl<L, R> Both<L, R> {
    pub const fn new(left: L, right: R) -> Self {
        Both { left, right }
    }

    /// Maps Both<L, R> to Both<T, R>
    pub fn map_left<F, T>(self, f: F) -> Both<T, R>
    where
        F: FnOnce(L) -> T,
    {
        Both { left: f(self.left), right: self.right }
    }

    /// Maps Both<L, R> to Both<L, T>
    pub fn map_right<F, T>(self, f: F) -> Both<L, T>
    where
        F: FnOnce(R) -> T,
    {
        Both { left: self.left, right: f(self.right) }
    }

    pub const fn as_ref(&self) -> Both<&L, &R> {
        Both { left: &self.left, right: &self.right }
    }

    pub const fn as_mut(&mut self) -> Both<&mut L, &mut R> {
        Both { left: &mut self.left, right: &mut self.right }
    }

    pub fn reverse(self) -> Both<R, L> {
        Both { left: self.right, right: self.left }
    }

    pub fn into_inner(self) -> (L, R) {
        (self.left, self.right)
    }
}

impl<L, R> Default for Both<L, R>
where
    L: Default,
    R: Default,
{
    fn default() -> Self {
        Both { left: L::default(), right: R::default() }
    }
}

impl<L, R> From<(L, R)> for Both<L, R> {
    fn from(value: (L, R)) -> Self {
        Both::new(value.0, value.1)
    }
}

impl<L, R> From<Both<L, R>> for (L, R) {
    fn from(value: Both<L, R>) -> Self {
        (value.left, value.right)
    }
}

impl<L, R> TrReverseLeftRight for Both<L, R> {
    type LeftType = L;
    type RightType = R;

    #[inline]
    fn reverse(self) -> impl TrReverseLeftRight<LeftType = Self::RightType, RightType = Self::LeftType> {
        Both::reverse(self)
    }
}

impl<L, R> TrAnyLeftRight for Both<L, R> {
    type LeftType = L;
    type RightType = R;

    #[inline]
    fn map_left<F, T>(self, f: F) -> impl TrAnyLeftRight<LeftType = T, RightType = Self::RightType >
    where
        F: FnOnce(Self::LeftType) -> T,
    {
        Both::map_left(self, f)
    }

    #[inline]
    fn map_right<F, T>(self, f: F) -> impl TrAnyLeftRight<LeftType = Self::LeftType, RightType = T>
    where
        F: FnOnce(Self::RightType) -> T,
    {
        Both::map_right(self, f)
    }

    #[inline]
    fn as_ref<'a>(&'a self) -> impl TrAnyLeftRight<LeftType = &'a Self::LeftType, RightType = &'a Self::RightType>
    where
        Self::LeftType: 'a,
        Self::RightType: 'a,
    {
        Both::as_ref(self)
    }

    #[inline]
    fn as_mut<'a>(&'a mut self) -> impl TrAnyLeftRight<LeftType = &'a mut Self::LeftType, RightType = &'a mut Self::RightType>
    where
        Self::LeftType: 'a,
        Self::RightType: 'a,
    {
        Both::as_mut(self)
    }

    #[inline]
    fn is_left(&self) -> bool {
        true
    }

    #[inline]
    fn is_right(&self) -> bool {
        true
    }

    #[inline]
    fn left(self) -> Option<Self::LeftType> {
        Option::Some(self.left)
    }

    #[inline]
    fn right(self) -> Option<Self::RightType> {
        Option::Some(self.right)
    }
}
