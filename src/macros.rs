#[macro_export]
macro_rules! infer {
    () => {
        $crate::Infer::INFER
    };
    ($ty:ty) => {
        <$ty as $crate::Infer>::INFER
    };
}
