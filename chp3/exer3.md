# chapter3练习

## 测试代码

```sh

make run BASE=0 LOG=DEBUG  # 控制框架仅编译实验测例

make run BASE=1 LOG=DEBUG  # 无需修改框架即可正常运行的测例

make run BASE=2 LOG=DEBUG  #控制框架同时编译基础测例和实验测例.
```

### **1. 函数总结**

| 函数名                                    | 功能                                       | 关键点                                                       |
| :---------------------------------------- | :----------------------------------------- | :----------------------------------------------------------- |
| **`TaskManager::run_first_task`**         | 启动第一个任务(通常是初始化后的首个应用) | - 将任务状态设为 `Running` - 调用 `__switch` 切换到任务上下文 |
| **`TaskManager::mark_current_suspended`** | 将当前运行任务标记为 `Ready`(挂起)       | - 修改任务状态,不切换上下文                                 |
| **`TaskManager::mark_current_exited`**    | 将当前运行任务标记为 `Exited`(退出)      | - 终止任务,释放资源                                         |
| **`TaskManager::find_next_task`**         | 寻找下一个可运行的 `Ready` 任务            | - 轮询查找下一个状态为 `Ready` 的任务                        |
| **`TaskManager::run_next_task`**          | 切换到下一个任务                           | - 修改状态为 `Running` - 调用 `__switch` 切换上下文          |
| **`run_first_task`**(模块级)            | 全局启动第一个任务的封装                   | - 直接调用 `TASK_MANAGER.run_first_task()`                   |
| **`suspend_current_and_run_next`**        | 挂起当前任务并切换到下一个                 | - 组合 `mark_current_suspended` + `run_next_task`            |
| **`exit_current_and_run_next`**           | 退出当前任务并切换到下一个                 | - 组合 `mark_current_exited` + `run_next_task`               |





依赖树

```
- syscall/mod.rs(系统调用入口)
  - 依赖 fs(文件系统相关系统调用)
    - sys_write:写操作(实现在 fs 子模块中)
  - 依赖 process(进程管理相关系统调用)
    - sys_exit:任务退出
    - sys_yield:任务让出 CPU
    - sys_get_time:获取时间
    - sys_trace:跟踪(待实现)
  - 功能:根据系统调用号分发请求

- syscall/process.rs(进程管理系统调用)
  - 依赖 task/mod.rs
    - exit_current_and_run_next:退出并切换任务
    - suspend_current_and_run_next:挂起并切换任务
    - TASK_MANAGER:任务管理器实例
  - 依赖 timer
    - get_time_us:获取微秒时间
  - 功能:实现进程退出、调度和时间查询

- task/mod.rs(任务管理核心)
  - 依赖 context(任务上下文)
    - TaskContext:任务上下文结构
  - 依赖 switch(任务切换)
    - __switch:上下文切换的汇编实现
  - 依赖 task(任务定义)
    - TaskControlBlock:任务控制块
    - TaskStatus:任务状态(UnInit、Ready、Running、Exited)
  - 依赖 loader
    - get_num_app:获取应用数量
    - init_app_cx:初始化应用上下文
  - 依赖 sync
    - UPSafeCell:线程安全单元,保护 TaskManagerInner
  - 依赖 config
    - MAX_APP_NUM:最大任务数常量
  - 功能:管理任务状态、调度和切换
```

```sh
这是 on qemu下测试我的2025s-rcore-Unconnectable/src/bin/ch2b_bad_address.rs文件
2025s-rcore-Unconnectable/bootloader/rustsbi-qemu.bin 这是rustsbi文件的路径

我直接运行我的测试文件以下是输出
cargo run --package user_lib --bin ch2b_bad_address 

warning: `/home/filament/.cargo/config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
warning: `/home/filament/.cargo/config` is deprecated in favor of `config.toml`
note: if you need to support cargo 1.38 or earlier, you can symlink `config` to `config.toml`
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/riscv64gc-unknown-none-elf/debug/ch2b_bad_address`
target/riscv64gc-unknown-none-elf/debug/ch2b_bad_address: 1: ELF�@��@8@: not found
target/riscv64gc-unknown-none-elf/debug/ch2b_bad_address: 4: Syntax error: Unterminated quoted string

 *  终端进程“cargo 'run', '--package', 'user_lib', '--bin', 'ch2b_bad_address'”启动失败(退出代码: 2). 
 
 
 qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios $(pwd)/bootloader/rustsbi-qemu.bin \
    -kernel $(pwd)/user/target/riscv64gc-unknown-none-elf/debug/ch2b_bad_address
    
    
    
以下是输出
qemu-system-riscv64: Some ROM regions are overlapping
These ROM regions might have been loaded by direct user request or by default.
They could be BIOS/firmware images, a guest kernel, initrd or some other file loaded into guest memory.
Check whether you intended to load all this guest code, and whether it has been built to load to the correct addresses.

The following two regions overlap (in the memory address space):
  /home/filament/Courses/2025s-rcore-Unconnectable/user/target/riscv64gc-unknown-none-elf/debug/ch2b_bad_address ELF program header segment 0 (addresses 0x0000000000000000 - 0x00000000000082bc)
  mrom.reset (addresses 0x0000000000001000 - 0x0000000000001028)
  
  
我有很多个像ch2b_bad_address.rs这样的文件 我应该如何运行和测试他们,按照上面的说明
```

