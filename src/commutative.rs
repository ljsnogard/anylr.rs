pub use crate::{AnyOf, BothOf, EitherOf, SomeOf};

/// A trait for types that commutate the position of left type and right type.
pub trait TrCommutative {
    type Left;
    type Right;
    type Commutated: TrCommutative<Left = Self::Right, Right = Self::Left>;

    /// Exchange the place of left and right.
    fn into_commutated(self) -> Self::Commutated;

    /// Tell who am I.
    fn into_variant(self) -> CommutativeVariant<Self::Left, Self::Right>;
}

pub enum CommutativeVariant<L, R> {
    Any(AnyOf<L, R>),
    Both(BothOf<L, R>),
    Either(EitherOf<L, R>),
    Some(SomeOf<L, R>),
}
