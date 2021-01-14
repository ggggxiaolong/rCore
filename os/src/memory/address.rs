//! 定义地址类型和地址常量
//!
//! 我们为虚拟地址和物理地址设立两种类型， 利用编译器检查来防止混淆

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PhysicalAddress(pub usize);

impl PhysicalAddress {
    // pub fn page_offset(&self) -> usize {
    //     self.0
    // }
}