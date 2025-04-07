





```rust
根据下面的systrace调用的参数和语法实现sys_get_time
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let token = current_user_token();
    let _page_table = PageTable::from_token(token);
    

    //检查指针是否有效
    if __(){
        return -1;
    }
    
    if {
       //
        0
    }
    else {
        -1
    }
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// /// 提示：你可能需要通过虚拟内存管理重新实现它
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    // 获取当前任务的用户态token
    let token = current_user_token(); //satp 寄存器的值，包含页表物理地址和模式信息
    let _page_table = PageTable::from_token(token); //临时PageTable

    match _trace_request {
        // 读取操作 - 读取1字节内存
        0 => {
            let is_valid_id = |id: usize, len: usize| -> bool {
                const ADD_MAX: usize = 0x8000_0000;
                const PAGE_SIZE: usize = 4096; //4KB
                //end = id + len ,end是在增加之后的实际页码,但是是左闭右开区间[start,end)
                //所以下面需要 -1
                let end = match id.checked_add(len) {
                    Some(val) => val,
                    None => {
                        return false;
                    }
                };
                if end > ADD_MAX {
                    return false;
                }
                if len > PAGE_SIZE {
                    let strat_page = id / PAGE_SIZE;
                    let end_page = (end - 1) / PAGE_SIZE;
                    if strat_page != end_page {
                        return false;
                    }
                }
                true
            };
            //以下是判断代码
            let _vpn = VirtAddr::from(_id).floor();
            if !is_valid_id(_id, 1) {
                return -1;
            }
            let vpn = VirtAddr::from(_id).floor();

            // 检查页表项是否存在且可读 Read仅仅需要readable
            if let Some(pte) = _page_table.translate(vpn) {
                if !pte.is_valid() || !pte.readable() {
                    return -1;
                }
                        
                // 安全读取字节
                let ppn = pte.ppn();
                let offset = VirtAddr::from(_id).page_offset();
                let byte = ppn.get_bytes_array()[offset];
                byte as isize
            } else {
                -1
            }
        }

        // 写入操作 - 写入usize数据
        1 => {
            let vpn = VirtAddr::from(_id).floor();

            // 检查页表项是否存在且可写
            if let Some(pte) = _page_table.translate(vpn) {
                if !pte.is_valid() || !pte.writable() || !pte.readable() {
                    return -1;
                }

                // 检查是否跨页
                let start = _id;
                let end = _id + core::mem::size_of::<usize>();
                if VirtAddr::from(start).floor() != VirtAddr::from(end - 1).floor() {
                    return -1; // 不支持跨页写入
                }

                // 安全写入
                let ppn = pte.ppn();
                let offset = VirtAddr::from(_id).page_offset();
                let bytes = _data.to_ne_bytes();
                ppn.get_bytes_array()[
                    offset..offset + core::mem::size_of::<usize>()
                ].copy_from_slice(&bytes);
                0
            } else {
                -1
            }
        }

        // 无效请求
        _ => -1,
    }
}
```





```rust
impl From<usize> for VirtAddr {
    fn from(v: usize) -> Self {
        Self(v & ((1 << VA_WIDTH_SV39) - 1))
    }
}

pub fn page_offset(&self) -> usize {
        self.0 & (PAGE_SIZE - 1)
    }
    
    
pub fn get_bytes_array(&self) -> &'static mut [u8] {
        //物理页号转换为 物理地址 PhysAddr  返回字节数组
        let pa: PhysAddr = (*self).into();
        unsafe { core::slice::from_raw_parts_mut(pa.0 as *mut u8, 4096) }
    }
    
以上是调用的代码，根据以上讲解
let offset = VirtAddr::from(start).page_offset();
        
        let time_bytes = unsafe{
            core::slice::from_raw_parts(
                &curr_time as * const TimeVal as *const u8,
                core::mem::size_of::<TimeVal>(),
            )
        };
        
        我实现的代码
```



```rust
这是unmap2.rs
#[no_mangle]
fn main() -> i32 {
    let start: usize = 0x10000000;
    let len: usize = 4096;
    let prot: usize = 3;
    assert_eq!(0, mmap(start, len, prot));
    assert_eq!(munmap(start, len + 1), -1);
    assert_eq!(munmap(start + 1, len - 1), -1);
    println!("Test 04_6 ummap2 OK!");
    0
}

这是unmap1.rs的测试
#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{mmap, munmap};

/*
理想结果：输出 Test 04_5 ummap OK!
*/

#[no_mangle]
fn main() -> i32 {
    let start: usize = 0x10000000;
    let len: usize = 4096;
    let prot: usize = 3;
    assert_eq!(0, mmap(start, len, prot));
    assert_eq!(mmap(start + len, len * 2, prot), 0);
    assert_eq!(munmap(start, len), 0);
    assert_eq!(mmap(start - len, len + 1, prot), 0);
    for i in (start - len)..(start + len * 3) {
        let addr: *mut u8 = i as *mut u8;
        unsafe {
            *addr = i as u8;
        }
    }
    for i in (start - len)..(start + len * 3) {
        let addr: *mut u8 = i as *mut u8;
        unsafe {
            assert_eq!(*addr, i as u8);
        }
    }
    println!("Test 04_5 ummap OK!");
    0
}

```

