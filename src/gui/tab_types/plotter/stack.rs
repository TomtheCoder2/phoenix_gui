use std::mem::MaybeUninit;

/// A stack with a fixed size
pub const STACK_SIZE: usize = 64;
/// IMPORTANT: This is a stack of f64s, not Values and is not garbage collected, so it is not safe to use with Phoenix
/// And its only as big as STACK_SIZE, so it can overflow (but shouldn't)
/// btw idk if its really faster in reality, but i think it should be, because where just always allocating the same amount of memory, rather than doing it dynamically
pub struct Stack {
    xs: [MaybeUninit<f64>; STACK_SIZE],
    sz: usize,
}

// From standard library
// https://doc.rust-lang.org/stable/src/core/mem/maybe_uninit.rs.html#350-353
#[must_use]
#[inline(always)]
pub const fn uninit_array<const N: usize, T>() -> [MaybeUninit<T>; N] {
    // SAFETY: An uninitialized `[MaybeUninit<_>; LEN]` is valid.
    unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self {
            xs: uninit_array(),
            sz: 0,
        }
    }

    pub fn push(&mut self, item: f64) -> bool {
        if (self.sz + 1) <= STACK_SIZE {
            self.xs[self.sz].write(item);
            self.sz += 1;
            true
        } else {
            false
        }
    }

    pub fn pop(&mut self) -> Option<f64> {
        (self.sz > 0).then(|| {
            self.sz -= 1;
            // Safety: The value has been initialized
            unsafe { self.xs[self.sz].assume_init() }
        })
    }

    pub fn pop_n(&mut self, n: usize) -> Vec<Option<f64>> {
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(self.pop());
        }
        v
    }
}
