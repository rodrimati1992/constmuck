/// For constructing types that implement [`Infer`], this includes `Is*` types.
#[macro_export]
macro_rules! infer {
    () => {
        $crate::Infer::INFER
    };
    ($ty:ty) => {
        <$ty as $crate::Infer>::INFER
    };
}

/// For constructing `Is*` types (values that represent trait bounds),
/// and tuples of them.
pub trait Infer: Sized + Copy {
    /// Constructs this type.
    const INFER: Self;
}

macro_rules! impl_tuple {
    ($($ty:ident),*) => {
        impl<$($ty),*> Infer for ($($ty,)*)
        where $($ty: Infer,)*
        {
            const INFER: Self = ($($ty::INFER,)*);
        }
    };
}

impl_tuple! {}
impl_tuple! {A}
impl_tuple! {A,B}
impl_tuple! {A,B,C}
impl_tuple! {A,B,C,D}
impl_tuple! {A,B,C,D,E}
impl_tuple! {A,B,C,D,E,F}
impl_tuple! {A,B,C,D,E,F,G}
impl_tuple! {A,B,C,D,E,F,G,H}
