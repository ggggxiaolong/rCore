use crate::memory::address::{PhysicalAddress, PhysicalPageNumber};
use crate::memory::frame::allocator::FRAME_ALLOCATOR;

/// 分配出的物理内存页
///
/// # 'Tracker' 是什么？
/// 太长不看
/// > 可以理解为 ['Box'](alloc::boxed::Box), 而区别在于, 其空间不是分配在堆上，
/// > 而是直接在内存中划一片(一个物理页)
///
/// 在我们实现操作系统过程中，会经常遇到 [指定一块内存区域作为某种用处] 的情况.
/// 此时，我们说这块内存可以用，但是因为它不在堆栈上，Rust 编译器并不知道它是什么， 所以
/// 我们需要 unsafe 地将其转换为 `&`static mut T` 的形式
pub struct FrameTracker(pub(super) PhysicalPageNumber);

impl FrameTracker {
    /// 帧的物理地址
    pub fn address(&self) -> PhysicalAddress {
        self.0.into()
    }

    /// 帧的物理页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().dealloc(self);
    }
}