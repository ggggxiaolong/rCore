//! 定义地址类型和地址常量
//!
//! 我们为虚拟地址和物理地址设立两种类型， 利用编译器检查来防止混淆

use crate::memory::config::PAGE_SIZE;

/// 物理地址(0x080..)
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalAddress(pub usize);

/// 物理页号(page 1, 2, 3...)
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalPageNumber(pub usize);

impl PhysicalAddress {
    /// 取得页内偏移
    pub fn page_offset(&self) -> usize {
        self.0 % PAGE_SIZE
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

        impl core::fmt::Display for $type_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}(0x{:x})", stringify!($type_name), self.0)
            }
        }
    };
}

implement_usize_operations! {PhysicalAddress}
implement_usize_operations! {PhysicalPageNumber}