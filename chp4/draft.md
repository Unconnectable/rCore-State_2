





```rust
我会给出代码和问题
//os/src/syscall/processer.rs
// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
     const PAGE_SIZE: usize = 4096;
    if _start % PAGE_SIZE != 0 {d  
        return -1;
    }
    if (_port & !0x7) != 0 || (_port & 0x7) == 0 {
        return -1;
    }

    let aligned_len = ((_len + PAGE_SIZE - 1) / PAGE_SIZE) * PAGE_SIZE;
    let token = current_user_token();
    let mut _page_table = PageTable::from_token(token);

    let _start_vpn = VirtAddr::from(_start).floor();

    let _pages_count = aligned_len / PAGE_SIZE;

    for i in 0.._pages_count {
        let vpn = VirtPageNum(_start_vpn.0 + i);
        if let Some(pte) = _page_table.translate(vpn) {
            if pte.is_valid(){
                return -1;
            }
            // 检测到已映射
            //return 3是为了定位在那里的return错误
        }
    }
    //分配新的物理页面并映射
    for i in 0.._pages_count {
        let vpn = VirtPageNum(_start_vpn.0 + i);

        //使用frame_alloc分配物理页面
        let ppn = match frame_alloc() {
            Some(frame) => frame.ppn,
            None => {
                return -1;
            } // 物理内存不足
        };
        let flags = match _port {
            1 => PTEFlags::R | PTEFlags::U ,
            2 => PTEFlags::W | PTEFlags::U,
            3 => PTEFlags::R | PTEFlags::W | PTEFlags::U,
            _ => unreachable!("_port should be 1, 2, or 3 due to prior check"),
        };
        _page_table.map(vpn, ppn, flags as PTEFlags);
    }
    0
}
// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let start_va = VirtAddr::from(_start);
    if !start_va.aligned() {
        return -1;
    }
    const PAGE_SIZE: usize = 4096;
    //检查参数
    if _start % PAGE_SIZE != 0 {
        println!("\x1b[31m argu Wrong\x1b[0m");
        return -1;
    }
    let aligned_len = ((_len + PAGE_SIZE - 1) / PAGE_SIZE) * PAGE_SIZE;
    let token = current_user_token();
    let mut _page_table = PageTable::from_token(token);

    let _start_vpn = VirtAddr::from(_start).floor();
    let _pages_count = aligned_len / PAGE_SIZE;

    for i in 0.._pages_count {
        let vpn = VirtPageNum(_start_vpn.0 + i);
        match _page_table.translate(vpn) {
            Some(pte) => {
                if !pte.is_valid() {
                    //println!("\x1b[31m vpn {:?} is invalid 页面存在但无效 \x1b[0m", vpn);
                    return -1; // 页面存在但无效
                }
            }
            None => {
                //println!("\x1b[31m vpn {:?} not mapped \x1b[0m", vpn);
                return -1;
            }
        }
    }
    for i in 0.._pages_count {
        let vpn = VirtPageNum(_start_vpn.0 + i);
        _page_table.unmap(vpn);
    }

    0
}

这是报错的测试
Panicked at src/bin/ch4_unmap.rs:20, assertion `left == right` failed
  left: -1
 right: 0

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

assert_eq!(munmap(start, len), 0);在这一行报错了
这是另一个测试
fn main() -> i32 {
    let start: usize = 0x10000000;
    let len: usize = 4096;
    let prot: usize = 3;
    assert_eq!(0, mmap(start, len, prot));
    assert_eq!(munmap(start, len + 1), -1);
    assert_eq!(munmap(start + 1, len - 1), -1);
    println!("Test 04_6 ummap2 OK!");
    0
}全部通过 请你解释我的错误
```

```rust
is_overleap:
https://github.com/LearningOS/2025s-rcore-NoahNieh/blob/ch4/os/src/task/mod.rs

map and unmap in memory set:

https://github.com/LearningOS/2025s-rcore-NoahNieh/blob/ch4/os/src/mm/memory_set.rs
// os/src/task.mod.rs

//上面是 impl TASK
pub fn mmap(start: usize, len: usize, port: usize) -> isize {
    debug!("kernel: mmap: start = {:#x}, len = {:#x}, port = {:#x}", start, len, port);
    let start_va:VirtAddr = start.into();
    if !start_va.aligned() {
        return -1;
    }
    if (port & (!0x7)) != 0 || (port & 0x7) == 0 {
        return -1;
    }
    let end_va: VirtAddr = (start + len).into();
    let start_vpn = start_va.floor();
    let end_vpn = end_va.ceil();
    let mut inner = TASK_MANAGER.inner.exclusive_access();
    let cur = inner.current_task;
    if inner.tasks[cur].memory_set.is_overlap(start_vpn, end_vpn) {
        debug!("kernel: mmap: overlap");
        return -1;
    }
    debug!("kernel: mmap: start_vpn = {:?}, end_vpn = {:?}", start_vpn, end_vpn);
    inner.tasks[cur].memory_set.mmap(start_vpn, end_vpn, MapPermission::from_bits_truncate((port as u8) << 1) | MapPermission::U);
    return 0
}

pub fn munmap(start: usize, len: usize) -> isize {
    debug!("kernel: munmap: start = {:#x}, len = {:#x}", start, len);
    let start_va:VirtAddr= start.into();
    if !start_va.aligned() {
        return -1;
    }
    let start_vpn = start_va.floor();
    let end_va: VirtAddr = (start + len).into();
    let end_vpn = end_va.ceil();
    let mut inner = TASK_MANAGER.inner.exclusive_access();
    let cur = inner.current_task;
    debug!("kernel: munmap: start_vpn = {:?}, end_vpn = {:?}", start_vpn, end_vpn);
    return match inner.tasks[cur].memory_set.munmap(start_vpn, end_vpn)
    {
        Ok(_) => 0,
        Err(_) => -1,
    }
}
```



$$
a_{17}a_{16}..a_0\\
校验公式,\sum_{}^{} a_i \times 2_i \mod 11,a_i =每一位的数字
$$


```cpp
你有
n
个砝码,第
i
个砝码重量为
a
i
.你有一架秤,你只能在同一边放上砝码,现在你想这架秤最小的称不出的重量是多少？

比如,有
3
个砝码,重量分别为
1
,
2
,
4
,那么第一个你没法称出的重量是
8
.
  
 题解
  题意
n
个砝码,只能放天平的一边,问最小的无法称重的重量.
思路
如果你现在能称
[l,r]
,新加入的砝码重量为
a
,显然,必须使用这颗砝码能称的重量区间为
[l+a,r+a]
.
这时,如果
r≥l+a−1
,那么称重连续区间就可以扩展成
[l,r+a]
.显然,初始区间为
[0,0]
;然后按砝码升序(题目已经给了,不需要排序)计算即可.
时间复杂度为
O(n)
.
  
  检查我的代码的错误
  #include <algorithm>
#include <cstdio>
#include <cstring>
#include <functional>
#include <iostream>
#include <ostream>
#include <sstream>
#include <string>

#define lld long long // long long 的printf 占位符是lld
#define ENDL '\n'     // 将 endl 替换为 \n 取消缓冲区

const long long MAX_ = 1e9;

using std::cin;
using std::cout;
using std::string;
int a[10909];
void solve() {
  memset(a, 0, sizeof(a));
  int n;
  cin >> n;

  int x;
  int L = 0, R = 0;
  for (int i = 0; i < n; ++i)
    cin >> a[i];
  bool tag = 0;
  for (int i = 0; i < n; ++i) {
    if (L + a[i] - 1 <= R) {
      L = L;
      R = R + a[i];
    } else {
      tag = 1;
      cout << L + 1 << ENDL;
      return;
    }
  }
  cout << R + 1 << ENDL;
}
int main() {
  int T = 1;
  cin >> T;
  while (T--) {
    solve();
  }
}
```

