use lazy_static::*;
use crate::memory::address::PhysicalAddress;

/// 操作系统动态分配内存所作用的堆大小(8M)
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;

lazy_static! {
 pub static ref KERNEL_END_ADDRESS: PhysicalAddress = PhysicalAddress(kernel_end as usize);
}

extern "C" {
    /// 由 ‘Linker.ld’ 指定的内核代码结束位置
    ///
    /// 作为变量存在 ['KERNEL_END_ADDRESS']
    fn kernel_end();
}