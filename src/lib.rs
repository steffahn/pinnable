// reasonable clippy categories
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
// reasonable clippy::restriction lints
#![warn(
    clippy::as_conversions,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::else_if_without_else,
    clippy::exhaustive_enums,
    clippy::exhaustive_structs,
    clippy::exit,
    clippy::expect_used,
    clippy::filetype_is_file,
    clippy::float_arithmetic,
    clippy::float_cmp_const,
    clippy::get_unwrap,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::integer_division,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::map_err_ignore,
    clippy::mem_forget,
    clippy::missing_docs_in_private_items,
    clippy::modulo_arithmetic,
    clippy::multiple_inherent_impl,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::pattern_type_mismatch,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::rc_buffer,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unreachable,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::use_debug,
    clippy::verbose_file_reads,
    clippy::wildcard_enum_match_arm
)]
// reasonable rustc lints
#![warn(
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    semicolon_in_expressions_from_macros,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
// reasonable rustdoc lints
#![warn(
    rustdoc::missing_crate_level_docs,
    rustdoc::missing_doc_code_examples,
    rustdoc::private_doc_tests,
    rustdoc::invalid_html_tags
)]

//! [![crates.io]](https://crates.io/crates/pinnable)
//! [![github]](https://github.com/steffahn/pinnable)
//! [![MIT / Apache 2.0 licensed]](https://github.com/steffahn/pinnable#License)
//!
//! A wrapper for [`Mutex`](std::sync::Mutex "std::sync::Mutex")
//! that supports obtaining `Pin<&mut T>` references to the contained value.
//! It’s a trade-off though, because it can no longer be locked _without_ being pinned.
//!
//! [github]: https://img.shields.io/badge/github-steffahn/pinnable-yellowgreen.svg
//! [crates.io]: https://img.shields.io/crates/v/pinnable.svg
//! [MIT / Apache 2.0 licensed]: https://img.shields.io/crates/l/pinnable.svg
//! [docs.rs]: https://docs.rs/pinnable/badge.svg

use std::{fmt, ops, pin::Pin, sync};

/// Documentation still incomplete. API similar to [`std::sync::Mutex`].
/// # Examples
/// ```
/// use std::future::Future;
/// use std::pin::Pin;
/// use std::sync::Arc;
/// use std::task::{Context, Poll};
/// 
/// fn poll_shared_future<F: Future>(
///     fut: &Pin<Arc<pinnable::Mutex<F>>>,
///     ctx: &mut Context<'_>,
/// ) -> Poll<F::Output> {
///     fut.as_ref().lock().unwrap().as_mut().poll(ctx)
/// }
/// ```
// FIXME: add docs
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub struct Mutex<T: ?Sized>(sync::Mutex<T>);

impl<T: ?Sized> fmt::Debug for Mutex<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

// FIXME: add docs
#[allow(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc
)]
impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        Self(sync::Mutex::new(t))
    }

    pub fn into_inner(self) -> sync::LockResult<T> {
        self.0.into_inner()
    }
}

// FIXME: add docs
#[allow(clippy::missing_docs_in_private_items)]
fn wrap_result<S, T>(f: impl FnOnce(S) -> T, r: sync::LockResult<S>) -> sync::LockResult<T> {
    match r {
        Ok(x) => Ok(f(x)),
        Err(e) => Err(sync::PoisonError::new(f(e.into_inner()))),
    }
}

// FIXME: add docs
#[allow(clippy::missing_docs_in_private_items)]
fn wrap_result_try<S, T>(
    f: impl FnOnce(S) -> T,
    r: sync::TryLockResult<S>,
) -> sync::TryLockResult<T> {
    use sync::TryLockError::{Poisoned, WouldBlock};
    match r {
        Ok(x) => Ok(f(x)),
        Err(Poisoned(e)) => Err(Poisoned(sync::PoisonError::new(f(e.into_inner())))),
        Err(WouldBlock) => Err(WouldBlock),
    }
}

// FIXME: add docs
#[allow(
    missing_docs,
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc
)]
impl<T: ?Sized> Mutex<T> {
    pub fn lock(self: Pin<&Self>) -> sync::LockResult<PinMutexGuard<'_, T>> {
        wrap_result(
            |x| unsafe { Pin::new_unchecked(x) },
            self.get_ref().0.lock(),
        )
    }

    pub fn lock_no_pin(&self) -> sync::LockResult<NoPinMutexGuard<'_, T>> {
        wrap_result(NoPinMutexGuard, self.0.lock())
    }

    pub fn try_lock(self: Pin<&Self>) -> sync::TryLockResult<PinMutexGuard<'_, T>> {
        wrap_result_try(
            |x| unsafe { Pin::new_unchecked(x) },
            self.get_ref().0.try_lock(),
        )
    }

    pub fn try_lock_no_pin(&self) -> sync::TryLockResult<NoPinMutexGuard<'_, T>> {
        wrap_result_try(NoPinMutexGuard, self.0.try_lock())
    }

    pub fn is_poisoned(&self) -> bool {
        self.0.is_poisoned()
    }

    pub fn get_mut(self: Pin<&mut Self>) -> sync::LockResult<Pin<&mut T>> {
        wrap_result(
            |x| unsafe { Pin::new_unchecked(x) },
            unsafe { Pin::into_inner_unchecked(self) }.0.get_mut(),
        )
    }

    pub fn get_mut_no_pin(&mut self) -> sync::LockResult<&mut T> {
        self.0.get_mut()
    }
}

// FIXME: add docs
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub type PinMutexGuard<'a, T> = Pin<sync::MutexGuard<'a, T>>;

// FIXME: add docs
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub struct NoPinMutexGuard<'a, T: ?Sized>(sync::MutexGuard<'a, T>);

impl<T: ?Sized> fmt::Debug for NoPinMutexGuard<'_, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized> fmt::Display for NoPinMutexGuard<'_, T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T: ?Sized> ops::Deref for NoPinMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T: ?Sized> ops::DerefMut for NoPinMutexGuard<'_, T>
where
    T: Unpin,
{
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
