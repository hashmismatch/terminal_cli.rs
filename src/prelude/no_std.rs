
pub use core::marker::PhantomData;
pub use core::iter;
pub use core::cell::RefCell;
pub use core::fmt;
pub use core::ops::Range;
pub use core::num::Wrapping;
pub use core::cmp::*;
pub use core::mem;
pub use core::intrinsics::write_bytes;

pub use alloc::rc::Rc;
pub use alloc::boxed::Box;

pub use collections::vec::Vec;
pub use collections::string::*;
pub use collections::slice::SliceConcatExt;
pub use collections::fmt::format as format_to_string;

use core::intrinsics;

pub trait LocalFloat: Sized {
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn floor(self) -> Self;
}

impl LocalFloat for f32 {
	#[inline]
    fn ceil(self) -> f32 {
        unsafe { intrinsics::ceilf32(self) }
    }

	#[inline]
    fn round(self) -> f32 {
        unsafe { intrinsics::roundf32(self) }
    }

    #[inline]
    fn floor(self) -> f32 {
        unsafe { intrinsics::floorf32(self) }
    }
}
