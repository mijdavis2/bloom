use pin_project::pin_project;
#[pin(__private())]
pub struct TupleStruct<T, U>(#[pin] pub T, pub U);
#[allow(box_pointers)]
#[allow(deprecated)]
#[allow(explicit_outlives_requirements)]
#[allow(single_use_lifetimes)]
#[allow(unreachable_pub)]
#[allow(clippy::unknown_clippy_lints)]
#[allow(clippy::pattern_type_mismatch)]
#[allow(clippy::redundant_pub_crate)]
#[allow(clippy::used_underscore_binding)]
const _: () = {
    #[allow(box_pointers)]
    #[allow(deprecated)]
    #[allow(explicit_outlives_requirements)]
    #[allow(single_use_lifetimes)]
    #[allow(unreachable_pub)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::pattern_type_mismatch)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(dead_code)]
    #[allow(clippy::mut_mut)]
    #[allow(clippy::type_repetition_in_bounds)]
    pub(crate) struct __TupleStructProjection<'pin, T, U>(
        pub ::pin_project::__private::Pin<&'pin mut (T)>,
        pub &'pin mut (U),
    )
    where
        TupleStruct<T, U>: 'pin;
    #[allow(box_pointers)]
    #[allow(deprecated)]
    #[allow(explicit_outlives_requirements)]
    #[allow(single_use_lifetimes)]
    #[allow(unreachable_pub)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(clippy::pattern_type_mismatch)]
    #[allow(clippy::redundant_pub_crate)]
    #[allow(dead_code)]
    #[allow(clippy::ref_option_ref)]
    #[allow(clippy::type_repetition_in_bounds)]
    pub(crate) struct __TupleStructProjectionRef<'pin, T, U>(
        pub ::pin_project::__private::Pin<&'pin (T)>,
        pub &'pin (U),
    )
    where
        TupleStruct<T, U>: 'pin;
    impl<T, U> TupleStruct<T, U> {
        pub(crate) fn project<'pin>(
            self: ::pin_project::__private::Pin<&'pin mut Self>,
        ) -> __TupleStructProjection<'pin, T, U> {
            unsafe {
                let Self(_0, _1) = self.get_unchecked_mut();
                __TupleStructProjection(::pin_project::__private::Pin::new_unchecked(_0), _1)
            }
        }
        #[allow(clippy::missing_const_for_fn)]
        pub(crate) fn project_ref<'pin>(
            self: ::pin_project::__private::Pin<&'pin Self>,
        ) -> __TupleStructProjectionRef<'pin, T, U> {
            unsafe {
                let Self(_0, _1) = self.get_ref();
                __TupleStructProjectionRef(::pin_project::__private::Pin::new_unchecked(_0), _1)
            }
        }
    }
    #[forbid(safe_packed_borrows)]
    fn __assert_not_repr_packed<T, U>(this: &TupleStruct<T, U>) {
        let _ = &this.0;
        let _ = &this.1;
    }
    #[allow(missing_debug_implementations)]
    pub struct __TupleStruct<'pin, T, U> {
        __pin_project_use_generics: ::pin_project::__private::AlwaysUnpin<
            'pin,
            (
                ::pin_project::__private::PhantomData<T>,
                ::pin_project::__private::PhantomData<U>,
            ),
        >,
        __field0: T,
    }
    impl<'pin, T, U> ::pin_project::__private::Unpin for TupleStruct<T, U> where
        __TupleStruct<'pin, T, U>: ::pin_project::__private::Unpin
    {
    }
    #[doc(hidden)]
    unsafe impl<'pin, T, U> ::pin_project::UnsafeUnpin for TupleStruct<T, U> where
        __TupleStruct<'pin, T, U>: ::pin_project::__private::Unpin
    {
    }
    trait TupleStructMustNotImplDrop {}
    #[allow(clippy::drop_bounds, drop_bounds)]
    impl<T: ::pin_project::__private::Drop> TupleStructMustNotImplDrop for T {}
    impl<T, U> TupleStructMustNotImplDrop for TupleStruct<T, U> {}
    #[doc(hidden)]
    impl<T, U> ::pin_project::__private::PinnedDrop for TupleStruct<T, U> {
        unsafe fn drop(self: ::pin_project::__private::Pin<&mut Self>) {}
    }
};
fn main() {}
