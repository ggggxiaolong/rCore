use lazy_static::*;
use crate::memory::address::{PhysicalAddress, VirtualAddress};

/// 操作系统动态分配内存所作用的堆大小(8M)
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;

/// 页 / 帧大小，必须是 2^n (4K)
pub const PAGE_SIZE: usize = 0x1_000;

/// 可以访问的内存区域起始地址
pub const MEMORY_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x8000_0000);

/// 可以访问的内存区域结束地址
pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);

lazy_static! {
 pub static ref KERNEL_END_ADDRESS: VirtualAddress = VirtualAddress(kernel_end as usize);
}

/// 内核使用线性映射的偏移量
pub const KERNEL_MAP_OFFSET: usize = 0xffff_ffff_0000_0000;

extern "C" {
    /// 由 ‘Linker.ld’ 指定的内核代码结束位置
    ///
    /// 作为变量存在 ['KERNEL_END_ADDRESS']
    fn kernel_end();
}