运行测试

```shell
make run BASE=2

#进入qemu后
ch5_usertest
```

## 1 实现`spawn`

    新建(new)一个子进程,注意和fork的实现不同,参考new的写法
    然后把exec的实现加入进来就可以了
    注意转换usize和isize

---

## `frok` 和`exec`的伪代码

### `sys_fork`

```python
def sys_fork():
    # 获取当前进程(父进程)的 PCB (进程控制块)
    parent_pcb = get_current_process()

    # 创建子进程的 PCB (复制父进程的状态)
    child_pcb = copy_pcb(parent_pcb)

    # 分配新的PID给子进程
    child_pcb.pid = allocate_new_pid()

    # 设置子进程的返回值为0(区分父子进程)
    child_pcb.return_value = 0

    # 父进程返回子进程的PID
    parent_pcb.return_value = child_pcb.pid

    # 写时复制(COW)优化:标记内存页为只读,触发缺页时再实际复制
    mark_memory_pages_cow(parent_pcb, child_pcb)

    # 将子进程加入调度队列
    schedule_process(child_pcb)

    return parent_pcb.return_value  # 父进程返回子进程PID

# 关键行为:
# - 子进程是父进程的完整复制(内存、寄存器、文件描述符等).
# - 父子进程通过返回值区分(子进程返回0,父进程返回子进程PID).
```

## ` sys_exec`

```python
def sys_exec(program_path, argv, envp):
    # 获取当前进程的PCB
    pcb = get_current_process()

    # 1. 加载新程序到内存
    program_code, entry_point = load_program_from_disk(program_path)

    # 2. 释放旧程序的资源(内存、文件描述符等)
    free_process_resources(pcb)

    # 3. 设置新的地址空间
    setup_new_address_space(pcb, program_code)

    # 4. 更新进程的元数据
    pcb.program_path = program_path
    pcb.entry_point = entry_point

    # 5. 设置命令行参数和环境变量
    pcb.argv = argv
    pcb.envp = envp

    # 6. 跳转到新程序的入口点(永不返回)
    jump_to_entry_point(entry_point)

    # 若失败则返回错误(实际不会执行到这里)
    return -1

# 关键行为:
# - 完全替换当前进程的代码和数据.
# - 不创建新进程,仅替换当前进程的上下文.
# - 若成功,永不返回;若失败(如程序不存在),返回错误.
```

### `spawn`伪代码

```py
def sys_spawn(program_path, argv, envp, flags):
    # 1. 创建新进程(类似fork,但可能优化)
    if flags & SPAWN_NO_FORK:
        # 直接创建空进程(不复制父进程,如Windows的CreateProcess)
        child_pcb = create_empty_process()
    else:
        # 类Unix风格:基于fork优化(如vfork或COW)
        child_pcb = sys_fork_lightweight()  # 轻量级复制

    # 2. 在子进程上下文中加载程序
    if child_pcb.pid == 0:  # 子进程
        sys_exec(program_path, argv, envp)  # 加载新程序
        exit(1)  # 若exec失败则退出

    # 3. 返回子进程PID(父进程)
    return child_pcb.pid

# 辅助函数:轻量级fork(示例)
def sys_fork_lightweight():
    # 可能实现为vfork(共享地址空间直到exec)
    # 或直接复制必要资源(如文件描述符)
    pcb = get_current_process()
    child_pcb = minimal_copy_pcb(pcb)  # 仅复制必要部分
    child_pcb.pid = allocate_new_pid()
    return child_pcb

# 关键行为:
# - 结合了进程创建和程序加载的一步操作.
# - 可能避免完整的fork复制(根据flags优化).

```

## 具体实现

### `sys_spawn`

```rust
// os/src/syscall/process.rs
pub fn sys_spawn(path: *const u8) -> isize {
    trace!("kernel:pid[{}] sys_spawn NOT IMPLEMENTED", current_task().unwrap().pid.0);
    let curr_task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    if path.is_empty() {
        //检查无效的文件名
        return -1;
    }
    if let Some(elf_data) = get_app_data_by_name(path.as_str()) {
        let new_task = curr_task.spawn(elf_data);
        let new_pid = new_task.pid.0;
        add_task(new_task);
        new_pid as isize
    } else {
        -1
    }
}
```

### `TaskControlBlock`里面的`spawn`

```rust
// os/src/task/task.rs
impl TaskControlBlock {
    // ...
    pub fn spawn(self: &Arc<Self>, elf_data: &[u8]) -> Arc<Self> {
        let mut parent_inner = self.inner_exclusive_access();
        //let memory_set = MemorySet::from_existed_user(&parent_inner.memory_set);
        let (memory_set, user_sp, entry_point) = MemorySet::from_elf(elf_data);

        let trap_cx_ppn = memory_set
            .translate(VirtAddr::from(TRAP_CONTEXT_BASE).into())
            .unwrap()
            .ppn();

        let pid_handle = pid_alloc();
        let kernel_stack = kstack_alloc();
        let kernel_stack_top = kernel_stack.get_top();
        let task_control_block = Arc::new(Self {
            pid: pid_handle,
            kernel_stack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    trap_cx_ppn,
                    base_size: user_sp,
                    task_cx: TaskContext::goto_trap_return(kernel_stack_top),
                    task_status: TaskStatus::Ready,
                    memory_set,
                    parent: Some(Arc::downgrade(self)),
                    children: Vec::new(),
                    exit_code: 0,
                    heap_bottom: user_sp,
                    program_brk: user_sp,
                    priority: 16,
                    stride: 0,
                })
            },
        });
        parent_inner.children.push(task_control_block.clone());

        let trap_cx = task_control_block.inner_exclusive_access().get_trap_cx();
        *trap_cx = TrapContext::app_init_context(
            entry_point,
            user_sp,
            KERNEL_SPACE.exclusive_access().token(),
            kernel_stack_top,
            trap_handler as usize
        );
        task_control_block
    }
}

```
