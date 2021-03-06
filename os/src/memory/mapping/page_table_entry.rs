use crate::memory::address::{PhysicalPageNumber, PhysicalAddress};
use bit_field::BitField;
use bitflags::*;
use riscv::register::fcsr::Flag;
use bitflags::_core::fmt::Formatter;

/// Sv39 结构的页表项
/// 整体长度64位
/// 0-7 标志位, 8-9 拓展位, 10-54 物理页号(10-18 PPN[0] 19-27 PPN[1] 28-53 PPN[2]) 54-63 Reserved
#[derive(Copy, Clone, Default)]
pub struct PageTableEntry(usize);

/// Sv39 页表项中标志位的位置
const FLAG_RANGE: core::ops::Range<usize> = 0..8;
/// Sv39 页表项中物理页号的位置
const PAGE_NUMBER_RANGE: core::ops::Range<usize> = 10..54;

impl PageTableEntry {
    /// 将相应页号和标志写入一个页表项
    pub fn new(page_number: Option<PhysicalPageNumber>, mut flags: Flags) -> Self {
        // 标志位中是否包含 Valid 取决于 page_number 是否为 Some
        flags.set(Flags::VALID, page_number.is_some());
        Self(
            *0usize
                .set_bits(FLAG_RANGE, flags.bits() as usize)
                .set_bits(PAGE_NUMBER_RANGE, page_number.unwrap_or_default().into())
        )
    }

    /// 设置物理页号, 同时根据 ppn 是否为 Some 来设置 Valid 位
    pub fn update_page_number(&mut self, ppn: Option<PhysicalPageNumber>) {
        let mut flags = self.flags();
        flags.set(Flags::VALID, ppn.is_some());
        self.0
            .set_bits(FLAG_RANGE, flags.bits() as usize)
            .set_bits(PAGE_NUMBER_RANGE, ppn.unwrap_or_default().into());
    }

    /// 清除
    pub fn clear(&mut self){
        self.0 = 0;
    }

    /// 获取地址
    pub fn address(&self) -> PhysicalAddress {
        PhysicalAddress::from(self.page_number())
    }

    /// 获取页号
    pub fn page_number(&self) ->PhysicalPageNumber {
        PhysicalPageNumber::from(self.0.get_bits(PAGE_NUMBER_RANGE))
    }

    /// 获取标志位
    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_unchecked(self.0.get_bits(0..8) as u8)}
    }

    /// 是否为空 (可能非空页非 Valid)
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// 是否指向下一级 (RWX 全为0)
    pub fn has_nex_level(&self) -> bool {
        let flags = self.flags();
        !(flags.contains(Flags::READABLE)
            || flags.contains(Flags::WRITABLE)
            || flags.contains(Flags::EXECUTABLE))
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("PageTableEntry")
            .field("value", &self.0)
            .field("page_number", &self.page_number())
            .field("flags", &self.flags())
            .finish()
    }
}

bitflags! {
    /// 页表中的 8 个标志位
    pub struct Flags: u8 {
        /// 有效位
        const VALID =       1<<0;
        /// 可读位
        const READABLE =    1<<1;
        /// 可写位
        const WRITABLE =   1<<2;
        /// 可执行位
        const EXECUTABLE =  1<<3;
        /// 用户位
        const USER =        1<<4;
        /// 全局位, 我们不会使用
        const GLOBAL =      1<<5;
        /// 已使用位, 用于替换算法
        const ACCESSED =    1<<6;
        /// 已修改位, 用于替换算法
        const DIRTY=        1<<7;
    }
}

macro_rules! implement_flags {
    ($field: ident, $name: ident, $quote: literal) => {
        impl Flags {
            #[doc = "返回 `Flags::"]
            #[doc = $quote]
            #[doc = "` 或 `Flags::empty()`"]
            pub fn $name(value: bool) -> Flags {
                if value {
                    Flags::$field
                } else {
                    Flags::empty()
                }
            }
        }
    };
}

implement_flags! {USER, user, "USER"}
implement_flags! {READABLE, readable, "READABLE"}
implement_flags! {WRITABLE, writable, "WRITABLE"}
implement_flags! {EXECUTABLE, executable, "EXECUTABLE"}