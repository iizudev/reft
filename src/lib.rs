//! `Ref` & `Mut` — Universal `AsRef`/`AsMut` bridges for _any_ reference.
//!
//! Turns `&T` into `AsRef<T>` and `&mut T` into `AsMut<T>` with zero cost.
//! Perfect for generics that need trait bounds on foreign types.

use core::{clone::Clone, marker::PhantomData};

/// Immutable reference with guaranteed `AsRef<T>` impl.  
/// Works on _any_ type, including those you don't control.
pub struct Ref<'a, T: ?Sized>(*const T, PhantomData<&'a T>);

/// Mutable reference with `AsRef<T>` + `AsMut<T>` impls.  
/// Like `&mut T`, but plays nice with trait bounds.
pub struct Mut<'a, T: ?Sized>(*mut T, PhantomData<&'a T>);

/// Owned data with `AsRef<T>` + `AsMut<T>` impls.
#[derive(Clone, Copy)]
pub struct Own<T: Sized>(T);

impl<'a, T: ?Sized> Ref<'a, T> {
    /// Creates a new `Ref` from the provided reference.
    #[inline(always)]
    pub const fn new(inner: &'a T) -> Self {
        Self(inner as *const T, PhantomData)
    }

    /// Returns the inner reference.
    #[inline(always)]
    pub const fn get(&self) -> &'a T {
        unsafe { &*self.0 }
    }

    /// Returns the inner reference, consuming the wrapper.
    #[inline(always)]
    pub const fn into_inner(self) -> &'a T {
        self.get()
    }
}

impl<'a, T: ?Sized> Mut<'a, T> {
    /// Creates a new `Mut` from the provided **mutable** reference.
    #[inline(always)]
    pub const fn new(inner: &'a mut T) -> Self {
        Self(inner as *mut T, PhantomData)
    }

    /// Returns the inner reference.
    #[inline(always)]
    pub const fn get(&self) -> &'a T {
        unsafe { &*self.0 }
    }

    /// Returns the inner **mutable** reference.
    #[inline(always)]
    pub const fn get_mut(&mut self) -> &'a mut T {
        unsafe { &mut *self.0 }
    }

    /// Returns the inner **mutable** reference, consuming the wrapper.
    #[inline(always)]
    pub const fn into_inner(mut self) -> &'a mut T {
        self.get_mut()
    }
}

impl<T: Sized> Own<T> {
    /// Creates a `Own` with the provided inner.
    #[inline(always)]
    pub const fn new(inner: T) -> Self {
        Self(inner)
    }

    /// Returns an immutable reference to the inner data.
    #[inline(always)]
    pub const fn get(&self) -> &T {
        &self.0
    }

    /// Returns a **mutable** reference to the inner data.
    #[inline(always)]
    pub const fn get_mut(&mut self) -> &mut T {
        &mut self.0
    }

    /// Returns the inner data, consuming the wrapper.
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<'a, T: ?Sized> AsRef<T> for Ref<'a, T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<'a, T: ?Sized> Clone for Ref<'a, T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self::new(self.get())
    }
}

impl<'a, T: ?Sized> AsRef<T> for Mut<'a, T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<'a, T: ?Sized> AsMut<T> for Mut<'a, T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T: Sized> AsRef<T> for Own<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T: Sized> AsMut<T> for Own<T> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}
