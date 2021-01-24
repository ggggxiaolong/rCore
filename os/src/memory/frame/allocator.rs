use lazy_static::*;
use crate::memory::address::{PhysicalPageNumber, PhysicalAddress};
use crate::memory::config:: {MEMORY_END_ADDRESS, MEMORY_START_ADDRESS, KERNEL_END_ADDRESS};
use algorithm::{Allocator, AllocatorImpl};
use crate::memory::frame::frame_tracker::FrameTracker;
use spin::Mutex;
use crate::memory::range::Range;
use crate::memory::MemoryResult;

lazy_static! {
    /// 帧分配
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>> = Mutex::new(FrameAllocator::new(Range::from(
        PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(MEMORY_END_ADDRESS),
    )));
}

/// 基于线段树的帧分配 / 回收
pub struct FrameAllocator<T: Allocator> {
    /// 可用区间的起始
    start_ppn: PhysicalPageNumber,
    /// 分配器
    allocator: T,
}

impl<T: Allocator> FrameAllocator<T> {
    pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        FrameAllocator {
            start_ppn: range.into().start,
            allocator: T::new(range.into().len()),
        }
    }

    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        self.allocator
            .alloc()
            .ok_or("no available frame to allocate")
            .map(|offset| FrameTracker(self.start_ppn + offset))
    }

    pub(super) fn dealloc(&mut self, frame: &FrameTracker) {
        self.allocator.dealloc(frame.page_number() - self.start_ppn);
    }
}