//! Memory management implementation
//!
//! SV39 page-based virtual-memory architecture for RV64 systems, and
//! everything about memory management, like frame allocator, page table,
//! map area and memory set, is implemented here.
//!
//! Every task or process has a memory_set to control its virtual memory.

mod address;
mod frame_allocator;
mod heap_allocator;
mod memory_set;
mod page_table;

pub use address::{PhysAddr, PhysPageNum, VirtAddr, VirtPageNum};
use address::{StepByOne, VPNRange};
use alloc::format;
use alloc::string::String;
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use memory_set::remap_test;
pub use memory_set::{kernel_stack_position, MapPermission, MemorySet, KERNEL_SPACE};
pub use page_table::{translated_byte_buffer, PageTableEntry};
use page_table::{PTEFlags, PageTable};

use crate::config::{PAGE_SIZE, PAGE_SIZE_BITS};
use crate::task::{mem_conflict, mem_map, mem_unmap};

/// initiate heap allocator, frame allocator and kernel space
pub fn init() {
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}

/// 将一个虚拟内存地址转换换位物理地址指针
pub fn to_physics_ptr<T>(token: usize, ptr: *mut T) -> *mut T {
    let va = VirtAddr::from(ptr as usize);
    let off = va.page_offset();
    let table = PageTable::from_token(token);
    let vpn = va.floor();
    let ppn = table.translate(vpn).unwrap().ppn();
    (ppn.0 << PAGE_SIZE_BITS | off) as *mut T
}

/// 申请内存
/// 申请长度为 len 字节的物理内存，将其映射到 start 开始的虚存，内存页属性为 port
pub fn mmap(start: usize, len: usize, port: usize) -> Result<(), String> {
    let sa = VirtAddr::from(start);
    let ea = VirtAddr::from(start + len);
    let p = MapPermission::from_bits((port as u8) << 1).unwrap() | MapPermission::U;
    // 错误检查
    if start % PAGE_SIZE != 0 {
        Err(format!("E: start % PAGE_SIZE | PAGE_SIZE={}, start=0x{:o}", PAGE_SIZE, start))
    } else if port & !0x7 != 0 {
        Err(format!("E: port & !0x7 | port=0b{:b}", port))
    } else if port & 0x7 == 0 {
        Err(format!("E: port & 0x7 | port=0b{:b}", port))
    } else if mem_conflict(&sa, &ea) {
        Err(format!("E: memory conflict | {} - {}", sa.floor().0, ea.ceil().0))
    } else {
        mem_map(sa, ea, p);
        Ok(())
    }
}

/// 释放内存
pub fn munmap(start: usize, len: usize) -> isize {
    let sa = VirtAddr::from(start);
    let ea = VirtAddr::from(start + len);
    if mem_unmap(sa, ea) {
        0
    } else {
        -1
    }
}
