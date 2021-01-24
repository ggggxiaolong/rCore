//! 定义地址类型和地址常量
//!
//! 我们为虚拟地址和物理地址设立两种类型， 利用编译器检查来防止混淆

use crate::memory::config::{PAGE_SIZE, KERNEL_MAP_OFFSET};
use bit_field::BitField;

/// 物理地址(0x080..)
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalAddress(pub usize);

/// 物理页号(page 1, 2, 3...)
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalPageNumber(pub usize);

/// 虚拟页号
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VirtualPageNumber(pub usize);

/// 虚拟地址
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct VirtualAddress(pub usize);

/// 从指针转换为虚拟地址
impl<T> From<*const T> for VirtualAddress {
    fn from(pointer: *const T) -> Self {
        Self(pointer as usize)
    }
}

/// 从指针转换为虚拟地址
impl<T> From<*mut T> for VirtualAddress {
    fn from(pointer: (*mut T)) -> Self {
        Self(pointer as usize)
    }
}

/// 物理页转换为虚拟页
impl From<PhysicalPageNumber> for VirtualPageNumber {
    fn from(ppn: PhysicalPageNumber) -> Self {
        Self(ppn.0 + KERNEL_MAP_OFFSET / PAGE_SIZE)
    }
}

/// 虚拟页转换为物理页
impl From<VirtualPageNumber> for PhysicalPageNumber {
    fn from(vpn: VirtualPageNumber) -> Self {
        Self(vpn.0 - KERNEL_MAP_OFFSET / PAGE_SIZE)
    }
}

/// 物理地址转换为虚拟地址
impl From<PhysicalAddress> for VirtualAddress {
    fn from(pa: PhysicalAddress) -> Self {
        Self(pa.0 + KERNEL_MAP_OFFSET)
    }
}

/// 虚拟地址转换为物理地址
impl From<VirtualAddress> for PhysicalAddress {
    fn from(va: VirtualAddress) -> Self {
        Self(va.0 - KERNEL_MAP_OFFSET)
    }
}

impl VirtualAddress {
    /// 从虚拟地址取得某类型的 &mut T 引用
    pub fn deref<T>(self) ->&'static mut T {
        unsafe {&mut *(self.0 as * mut T)}
    }
    /// 取得页内偏移
    pub fn page_offset(&self) -> usize {
        self.0 % PAGE_SIZE
    }
}

impl PhysicalAddress {
    /// 从物理地址经过线性映射取得 &mut 引用
    pub fn deref<T>(self) ->&'static mut T {
        VirtualAddress::from(self).deref()
    }
    /// 取得页内偏移
    pub fn page_offset(&self) -> usize {
        self.0 % PAGE_SIZE
    }
}

impl VirtualPageNumber {
    /// 从虚拟地址取得页面
    pub fn deref(self) -> &'static mut [u8; PAGE_SIZE] {
        VirtualAddress::from(self).deref()
    }
}

impl PhysicalPageNumber {
    pub fn deref_kernel(self) -> &'static mut [u8; PAGE_SIZE] {
        PhysicalAddress::from(self).deref()
    }
}

impl VirtualPageNumber {
    /// 得到一.二.三级页号
    pub fn levels(self) -> [usize; 3] {
        [
            self.0.get_bits(18..27),
            self.0.get_bits(9..18),
            self.0.get_bits(0..9),
        ]
    }
}

macro_rules! implement_address_to_page_number {
    // 这里面的类型转换实现 ['From'] trait, 会自动实现相反的 ['Into'] trait
    ($address_type: ty, $page_number_type: ty) => {
        impl From<$page_number_type> for $address_type {
            fn from(page_number: $page_number_type) -> Self {
                Self(page_number.0 * PAGE_SIZE)
            }
        }

        impl From<$address_type> for $page_number_type {
            fn from(address: $address_type) -> Self {
                assert!(address.0 % PAGE_SIZE == 0);
                Self(address.0 / PAGE_SIZE)
            }
        }

        impl $page_number_type {
            /// 将地址转换为页号， 向下取整
            pub const fn floor(address: $address_type) -> Self {
                Self(address.0 / PAGE_SIZE)
            }
            /// 将地址转换为页号, 向上取整
            pub const fn ceil(address: $address_type) -> Self {
                Self((address.0 + PAGE_SIZE -1) / PAGE_SIZE)
            }
        }
    };
}

implement_address_to_page_number! {PhysicalAddress, PhysicalPageNumber}
implement_address_to_page_number! {VirtualAddress, VirtualPageNumber}

macro_rules! implement_usize_operations {
    ($type_name: ty) => {
        /// 操作符 '+'
        impl core::ops::Add<usize> for $type_name {
            type Output = Self;
            fn add(self, other: usize) -> Self::Output {
                Self(self.0 + other)
            }
        }

        /// 操作符 `+=`
        impl core::ops::AddAssign<usize> for $type_name {
            fn add_assign(&mut self, rhs: usize) {
                self.0 += rhs;
            }
        }

        /// 操作符 ‘-’
        impl core::ops::Sub<usize> for $type_name {
            type Output = Self;
            fn sub(self, other: usize) -> Self::Output {
                Self(self.0 - other)
            }
        }

        /// 操作符 ‘-’
        impl core::ops::Sub<$type_name> for $type_name {
            type Output = usize;
            fn sub(self, other: $type_name) -> Self::Output {
                self.0 - other.0
            }
        }

        /// 操作符 ‘-=’
        impl core::ops::SubAssign<usize> for $type_name {
            fn sub_assign(&mut self, rhs: usize) {
                self.0 -= rhs;
            }
        }

        /// 实现From<usize>
        impl From<usize> for $type_name {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }

        impl From<$type_name> for usize {
            fn from(value: $type_name) -> Self {
                value.0
            }
        }

        impl $type_name {
            /// 是否有效(0 为无效)
            pub fn valid(&self) -> bool {
                self.0 != 0
            }
        }

        /// {} 输出
        impl core::fmt::Display for $type_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}(0x{:x})", stringify!($type_name), self.0)
            }
        }
    };
}

implement_usize_operations! {PhysicalAddress}
implement_usize_operations! {PhysicalPageNumber}
implement_usize_operations! {VirtualAddress}
implement_usize_operations! {VirtualPageNumber}
