# chapter3练习

## 测试代码

```sh

make run BASE=0 LOG=DEBUG  # 控制框架仅编译实验测例

make run BASE=1 LOG=DEBUG  # 无需修改框架即可正常运行的测例

make run BASE=2 LOG=DEBUG  #控制框架同时编译基础测例和实验测例.
```

给出一个usize类型的指针 如何获得他point的value

rsut中向一个地址_id写入数据data,意思是直接覆盖地址_id原来value吗

如果是,如何操作



```rust
use user_lib::{
    count_syscall, get_time, println, sleep, trace_read, trace_write,
    SYSCALL_EXIT, SYSCALL_GETTIMEOFDAY, SYSCALL_TRACE, SYSCALL_WRITE, SYSCALL_YIELD
};

这里的syscall函数是在那里
let t1 = get_time() as usize;
    get_time();
    sleep(500);
    let t2 = get_time() as usize;
    let t3 = get_time() as usize;
    assert!(3 <= count_syscall(SYSCALL_GETTIMEOFDAY));

Panicked at src/bin/ch3_trace.rs:22, assertion failed: 3 <= count_syscall(SYSCALL_GETTIMEOFDAY)

我给出一些代码的调用,首先你给出我一个这些代码的结构的相互依赖:
pub struct TaskManager {
    /// total number of tasks
    num_app: usize,
    /// use inner value to get mutable access
    inner: UPSafeCell<TaskManagerInner>,
}

/// Inner of Task Manager
pub struct TaskManagerInner {
    /// task list
    tasks: [TaskControlBlock; MAX_APP_NUM],
    /// id of current `Running` task
    current_task: usize,
    
}

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_TRACE => sys_trace(args[0], args[1], args[2]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
pub fn trace(request: TraceRequest, id: usize, data: usize) -> isize {
    sys_trace(request as usize, id, data)
}

pub fn trace_read(addr: *const u8) -> Option<u8> {
    match trace(TraceRequest::Read, addr as usize, 0) {
        -1 => None,
        data => Some(data as u8),
    }
}

pub fn trace_write(addr: *const u8, data: u8) -> isize {
    trace(TraceRequest::Write, addr as usize, data as usize)
}

pub fn count_syscall(id: usize) -> isize {
    trace(TraceRequest::Syscall, id, 0)
}

2.对于这一段建议 请你判断我的对他的理解,然后给出对我的建议

系统调用次数可以考虑在内核态的 syscall 函数中统计.

可以扩展 TaskManagerInner 中的结构来维护新的信息.

我的想法是:我在调用syscall函数的之前引入一个TaskManager然后在syscall的途中
```

```rust

```

这是测试代码

```rust
////这是测试代码

我想要实现trace部分，请你结合我给你的以上材料，做到的提示和下面的的测试代码给出建议
这是作业的提示：1。大胆修改已有框架！除了配置文件，你几乎可以随意修改已有框架的内容。

2.系统调用次数可以考虑在内核态的 syscall 函数中统计。

3.可以扩展 TaskManagerInner 中的结构来维护新的信息。

//这是测试代码
#![no_std]
#![no_main]

extern crate user_lib;

use user_lib::{
    count_syscall, get_time, println, sleep, trace_read, trace_write,
    SYSCALL_EXIT, SYSCALL_GETTIMEOFDAY, SYSCALL_TRACE, SYSCALL_WRITE, SYSCALL_YIELD
};

pub fn write_const(var: &u8, new_val: u8) {
    trace_write(var as *const _, new_val);
}

#[no_mangle]
pub fn main() -> usize {
    let t1 = get_time() as usize;
    get_time();
    sleep(500);
    let t2 = get_time() as usize;
    let t3 = get_time() as usize;
    assert!(3 <= count_syscall(SYSCALL_GETTIMEOFDAY));
    // 注意这次 sys_trace 调用本身也计入
    assert_eq!(2, count_syscall(SYSCALL_TRACE));
    assert_eq!(0, count_syscall(SYSCALL_WRITE));
    assert!(0 < count_syscall(SYSCALL_YIELD));
    assert_eq!(0, count_syscall(SYSCALL_EXIT));

    // 想想为什么 write 调用是两次
    println!("string from task trace test\n");
    let t4 = get_time() as usize;
    let t5 = get_time() as usize;
    assert!(5 <= count_syscall(SYSCALL_GETTIMEOFDAY));
    assert_eq!(7, count_syscall(SYSCALL_TRACE));
    assert_eq!(2, count_syscall(SYSCALL_WRITE));
    assert!(0 < count_syscall(SYSCALL_YIELD));
    assert_eq!(0, count_syscall(SYSCALL_EXIT));

    #[allow(unused_mut)]
    let mut var = 111u8;
    assert_eq!(Some(111), trace_read(&var as *const u8));
    write_const(&var, (t1 ^ t2 ^ t3 ^ t4 ^ t5) as u8);
    assert_eq!((t1 ^ t2 ^ t3 ^ t4 ^ t5) as u8, unsafe { core::ptr::read_volatile(&var) });

    assert!(None != trace_read(main as *const _));
    println!("Test trace OK!");
    0
}

```





```rust

```



### **1. 函数总结**

| 函数名                                    | 功能                                       | 关键点                                                       |
| :---------------------------------------- | :----------------------------------------- | :----------------------------------------------------------- |
| **`TaskManager::run_first_task`**         | 启动第一个任务（通常是初始化后的首个应用） | - 将任务状态设为 `Running` - 调用 `__switch` 切换到任务上下文 |
| **`TaskManager::mark_current_suspended`** | 将当前运行任务标记为 `Ready`（挂起）       | - 修改任务状态，不切换上下文                                 |
| **`TaskManager::mark_current_exited`**    | 将当前运行任务标记为 `Exited`（退出）      | - 终止任务，释放资源                                         |
| **`TaskManager::find_next_task`**         | 寻找下一个可运行的 `Ready` 任务            | - 轮询查找下一个状态为 `Ready` 的任务                        |
| **`TaskManager::run_next_task`**          | 切换到下一个任务                           | - 修改状态为 `Running` - 调用 `__switch` 切换上下文          |
| **`run_first_task`**（模块级）            | 全局启动第一个任务的封装                   | - 直接调用 `TASK_MANAGER.run_first_task()`                   |
| **`suspend_current_and_run_next`**        | 挂起当前任务并切换到下一个                 | - 组合 `mark_current_suspended` + `run_next_task`            |
| **`exit_current_and_run_next`**           | 退出当前任务并切换到下一个                 | - 组合 `mark_current_exited` + `run_next_task`               |





依赖树

```
- syscall/mod.rs（系统调用入口）
  - 依赖 fs（文件系统相关系统调用）
    - sys_write：写操作（实现在 fs 子模块中）
  - 依赖 process（进程管理相关系统调用）
    - sys_exit：任务退出
    - sys_yield：任务让出 CPU
    - sys_get_time：获取时间
    - sys_trace：跟踪（待实现）
  - 功能：根据系统调用号分发请求

- syscall/process.rs（进程管理系统调用）
  - 依赖 task/mod.rs
    - exit_current_and_run_next：退出并切换任务
    - suspend_current_and_run_next：挂起并切换任务
    - TASK_MANAGER：任务管理器实例
  - 依赖 timer
    - get_time_us：获取微秒时间
  - 功能：实现进程退出、调度和时间查询

- task/mod.rs（任务管理核心）
  - 依赖 context（任务上下文）
    - TaskContext：任务上下文结构
  - 依赖 switch（任务切换）
    - __switch：上下文切换的汇编实现
  - 依赖 task（任务定义）
    - TaskControlBlock：任务控制块
    - TaskStatus：任务状态（UnInit、Ready、Running、Exited）
  - 依赖 loader
    - get_num_app：获取应用数量
    - init_app_cx：初始化应用上下文
  - 依赖 sync
    - UPSafeCell：线程安全单元，保护 TaskManagerInner
  - 依赖 config
    - MAX_APP_NUM：最大任务数常量
  - 功能：管理任务状态、调度和切换
```

```rust
error[E0432]: unresolved import `crate::TASK_MANAGER`
 --> src/bin/ch3_trace.rs:5:5
  |
5 | use crate::TASK_MANAGER; // 从 user/src/lib.rs 导入
  |     ^^^^^^^^^^^^^^^^^^^ no `TASK_MANAGER` in the root

这是报错
以下我给出关键部分
// user/src/bin/ch3_trace.rs
extern crate user_lib;
use crate::TASK_MANAGER; // 从 user/src/lib.rs 导入



// os/src/task/mod.rs
mod context;
mod switch;
#[allow(clippy::module_inception)]
pub mod task;

const MAX_SYSCALL_NUM: usize = 512;
use crate::config::MAX_APP_NUM;
use crate::loader::{ get_num_app, init_app_cx };
use crate::sync::UPSafeCell;
use lazy_static::*;
use switch::__switch;
pub use task::{ TaskControlBlock, TaskStatus };

pub use context::TaskContext;

/// The task manager, where all the tasks are managed.
///
/// Functions implemented on `TaskManager` deals with all task state transitions
/// and task context switching. For convenience, you can find wrappers around it
/// in the module level.
///
/// Most of `TaskManager` are hidden behind the field `inner`, to defer
/// borrowing checks to runtime. You can see examples on how to use `inner` in
/// existing functions on `TaskManager`.
pub struct TaskManager {
    /// total number of tasks
    num_app: usize,
    /// use inner value to get mutable access
    inner: UPSafeCell<TaskManagerInner>,
}

/// Inner of Task Manager
pub struct TaskManagerInner {
    /// task list
    tasks: [TaskControlBlock; MAX_APP_NUM],
    /// id of current `Running` task
    current_task: usize,
    syscall_counts: [usize; MAX_SYSCALL_NUM],
}

lazy_static! {
    /// Global variable: TASK_MANAGER
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [
            TaskControlBlock {
                task_cx: TaskContext::zero_init(),
                task_status: TaskStatus::UnInit,
            };
            MAX_APP_NUM
        ];
        for (i, task) in tasks.iter_mut().enumerate() {
            task.task_cx = TaskContext::goto_restore(init_app_cx(i));
            task.task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                    syscall_counts: [0; MAX_SYSCALL_NUM],
                })
            },
        }
    };
}

impl TaskManager {
    /// Run the first task in task list.
    ///
    /// Generally, the first task in task list is an idle task (we call it zero process later).
    /// But in ch3, we load apps statically, so the first task is a real app.
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        // before this, we should drop local variables that must be dropped manually
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }
    /// reset all sys_counts to 0
    pub fn reset_syscall_counts(&self) {
        let mut inner = self.inner.exclusive_access();
        inner.syscall_counts = [0; MAX_SYSCALL_NUM];
    }
    

    //user/src/lib.rs
   #[macro_use]
pub mod console;
mod lang_items;
mod syscall;

extern crate alloc;
extern crate core;
#[macro_use]
extern crate bitflags;

use alloc::vec::Vec;
use buddy_system_allocator::LockedHeap;
pub use console::{flush, STDIN, STDOUT};
pub use syscall::*;
    
   // os/src/syscall/mod.rs 
    /// write syscall
const SYSCALL_WRITE: usize = 64;
/// exit syscall
const SYSCALL_EXIT: usize = 93;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GET_TIME: usize = 169;
/// trace syscall
const SYSCALL_TRACE: usize = 410;

mod fs;
mod process;

use fs::*;
use process::*;

use crate::task::TASK_MANAGER;
/// const for 
pub const SYSCALL_RESET_COUNTS: usize = 999;
/// handle syscall exception with `syscall_id` and other arguments
pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    TASK_MANAGER.add_syscall_count(syscall_id); // 更新计数
    //static mut WRITE_COUNT: usize = 0; // 静态变量记录 SYSCALL_WRITE 调用次数
    /*if syscall_id == SYSCALL_EXIT { // 只检查 SYSCALL_WRITE
        unsafe {
            WRITE_COUNT += 1;
            println!(
                "\x1b[31mSYSCALL_WRITE (id: 64) called #{}, args: {:?}\x1b[0m",
                WRITE_COUNT, args
            );
        }
    }*/
    match syscall_id {
        SYSCALL_RESET_COUNTS => {
            TASK_MANAGER.reset_syscall_counts();
            0
        }
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_TRACE => sys_trace(args[0], args[1], args[2]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
```

```rust
以下是测试
let t1 = get_time() as usize;
    get_time();
    sleep(500);
    let t2 = get_time() as usize;
    let t3 = get_time() as usize;
    assert!(3 <= count_syscall(SYSCALL_GETTIMEOFDAY));
    // 注意这次 sys_trace 调用本身也计入
    assert_eq!(2, count_syscall(SYSCALL_TRACE));
    assert_eq!(0, count_syscall(SYSCALL_WRITE));
    assert!(0 < count_syscall(SYSCALL_YIELD));
    assert_eq!(0, count_syscall(SYSCALL_EXIT));

    // 想想为什么 write 调用是两次
    println!("string from task trace test\n");
    let t4 = get_time() as usize;
    let t5 = get_time() as usize;
    assert!(5 <= count_syscall(SYSCALL_GETTIMEOFDAY));
    assert_eq!(7, count_syscall(SYSCALL_TRACE));
    assert_eq!(2, count_syscall(SYSCALL_WRITE));
    assert!(0 < count_syscall(SYSCALL_YIELD));
    assert_eq!(0, count_syscall(SYSCALL_EXIT));
    

pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    //-1
    if _trace_request == 0 {
        let value = unsafe { *(_id as *const u8) }; // 读取一个字节
        value as isize
    } else if _trace_request == 1 {
        let ptr = _id as *mut usize;
        unsafe {
            *ptr = _data;
        }
        return 0;
    } else if _trace_request == 2 {
        return TASK_MANAGER.get_syscall_id(_id);
    } else {
        return -1;
    }
}

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    TASK_MANAGER.add_syscall_count(syscall_id); // 更新计数
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time(args[0] as *mut TimeVal, args[1]),
        SYSCALL_TRACE => sys_trace(args[0], args[1], args[2]),
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}


const SYSCALL_WRITE: usize = 64;
/// exit syscall
const SYSCALL_EXIT: usize = 93;
/// yield syscall
const SYSCALL_YIELD: usize = 124;
/// gettime syscall
const SYSCALL_GET_TIME: usize = 169;
/// trace syscall
const SYSCALL_TRACE: usize = 410;
```



```
我发现不仅仅是syscall write ,连syscall exit都被提前计算了，
经过我的测试之后发现 加上在当前测试之前的记录次数，测试是对的
有什么办法能够解决
```

```rust
pub fn add_syscall_count(&self, syscall_id: usize) {
        let mut inner = self.inner.exclusive_access();
        if syscall_id < MAX_SYSCALL_NUM {
            inner.syscall_counts[syscall_id] += 1;
        }
    }
    /// os/src/syscall/processer.rs 's sys_trace will call this func
    pub fn get_syscall_id(&self, syscall_id: usize) -> isize {
        let inner = self.inner.exclusive_access();
        inner.syscall_counts[syscall_id] as isize
    }


这里并没有用到pub struct TaskManager {
    /// total number of tasks
    num_app: usize,
    /// use inner value to get mutable access
    inner: UPSafeCell<TaskManagerInner>,
}

/// Inner of Task Manager
pub struct TaskManagerInner {
    /// task list
    tasks: [TaskControlBlock; MAX_APP_NUM],
    /// id of current `Running` task
    current_task: usize,
    syscall_counts: [usize; MAX_SYSCALL_NUM],
}其中的Currenttask 和tasks的数据 请你修改使用这两个来计数`
```

```rust
pub struct TaskContext {
    /// Ret position after task switching
    ra: usize,
    /// Stack pointer
    sp: usize,
    /// s0-11 register, callee saved
    s: [usize; 12],
}


pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
```

