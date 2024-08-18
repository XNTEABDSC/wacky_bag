// from https://nomicon.purewhite.io/vec/vec-raw.html

use std::alloc::{self, Layout};
use std::ops::{Deref, DerefMut};
use std::{cmp, mem};
use std::ptr::NonNull;

pub struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}

impl<T> RawVec<T> {
    pub const fn new() -> Self {
        // !0 等价于 usize::MAX， 这一段分支代码在编译期间就可以计算出结果返回的结果，返回给 cap
        let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };

        // `NonNull::dangling()` 有双重含义:
        // `未分配内存 (unallocated)`, `零尺寸 (zero-sized allocation)`
        RawVec {
            ptr: NonNull::dangling(),
            cap: cap,
        }
    }

    pub fn with_capacity(cap:usize) -> RawVec<T> {
        if mem::size_of::<T>() != 0 {
            let layout=Layout::array::<T>(cap).unwrap();
            RawVec {
                ptr: match NonNull::new(unsafe { alloc::alloc(layout) } as *mut T) {
                    Some (p)=>p,
                    None=> alloc::handle_alloc_error(layout),
                },
                cap,
            }
        }
        else{
            RawVec {
                ptr: NonNull::dangling(),
                cap: usize::MAX,
            }
        }
    }

    pub fn try_grow(&mut self,cap:usize){
        if self.cap<=cap {self.grow(cap);}
        
    }

    pub fn grow(&mut self,cap:usize) {
        // 因为当 T 的尺寸为 0 时，我们设置了 cap 为 usize::MAX，
        // 这一步成立便意味着 Vec 溢出了.
        assert!(mem::size_of::<T>() != 0, "capacity overflow");

        let (new_cap, new_layout) = if self.cap == 0 {
            (cap, Layout::array::<T>(cap).unwrap())
        } else {
            // 保证新申请的内存没有超出 `isize::MAX` 字节
            let new_cap = cmp::max(cap, 2*self.cap);

            // `Layout::array` 会检查申请的空间是否小于等于 usize::MAX，
            // 但是因为 old_layout.size() <= isize::MAX，
            // 所以这里的 unwrap 永远不可能失败
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // 保证新申请的内存没有超出 `isize::MAX` 字节
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // 如果分配失败，`new_ptr` 就会成为空指针，我们需要处理该意外情况
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();

        if self.cap != 0 && elem_size != 0 {
            unsafe {
                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::array::<T>(self.cap).unwrap(),
                );
            }
        }
    }
}

impl<T> Deref for RawVec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.cap)
        }
    }
}

impl<T> DerefMut for RawVec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.cap)
        }
    }
}