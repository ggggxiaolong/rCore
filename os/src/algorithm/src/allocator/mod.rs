//! 负责分配 / 回收数据结构

mod stacked_allocator;
mod bitmap_vector_allocator;

/// 分配器：固定容量，每次分配 / 回收一个元素
pub trait Allocator {
    /// 给定容量, 创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配一个元素, 无法分配则返回 ‘None’
    fn alloc(&mut self) -> Option<usize>;
    /// 回收一个元素
    fn dealloc(&mut self, index: usize);
}

pub trait VectorAllocator {
    /// 给定容量，创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配指定长度的空间，无法分配则返回 `None`
    fn alloc(&mut self, size: usize, align: usize) -> Option<usize>;
    /// 回收指定空间（一定是之前分配的）
    fn dealloc(&mut self, start: usize, size: usize, align: usize);
}

pub use stacked_allocator::StackedAllocator;
pub use bitmap_vector_allocator::BitmapVectorAllocator;

pub type AllocatorImpl = StackedAllocator;
pub type VectorAllocatorImpl = BitmapVectorAllocator;