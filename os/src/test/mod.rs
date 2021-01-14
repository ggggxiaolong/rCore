pub fn break_test() {
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
}

pub fn memory_test() {
    // 动态内存分配测试
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..10_000 {
        vec.push(i)
    }
    assert_eq!(vec.len(), 10_000);
    for (i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }
    println!("Heap test passes")
}

pub fn kernel_address_test() {
    println!("kernel_address: 0x{:x}", (*crate::memory::config::KERNEL_END_ADDRESS).0);
}

pub fn physical_memory_test() {
    use crate::memory;
    for _ in 0..2 {
        let frame_0 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
           Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };

        let frame_1 = match memory::frame::FRAME_ALLOCATOR.lock().alloc() {
            Result::Ok(frame_tracker) => frame_tracker,
            Result::Err(err) => panic!("{}", err)
        };
        println!("{} and {}", frame_0.address(), frame_1.address())
    }
}