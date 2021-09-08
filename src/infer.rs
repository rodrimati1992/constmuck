/// For constructing `Impls*` types (values that represent trait bounds),
/// and tuples of them.
///
/// For a more concise way to write to `Infer::INFER`, there's the [`infer`] macro.
///
/// # Example
///
/// ```rust
/// use constmuck::cast;
/// use constmuck::{Infer, ImplsPod};
///
/// const ARR: [i8; 3] = cast([3u8, 5, u8::MAX - 1], Infer::INFER);
/// assert_eq!(ARR, [3, 5, -2]);
///
/// const fn requires_pod<T>(_bounds: ImplsPod<T>) {}
/// requires_pod::<u32>(Infer::INFER);
///
/// const fn requires_2_pods<T, U>(_bounds: (ImplsPod<T>, ImplsPod<U>)) {}
/// requires_2_pods::<u32, u64>(Infer::INFER);
///  
/// ```
pub trait Infer: Sized + Copy {
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
