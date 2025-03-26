# chp2

把ch1修改的文件`commit`后切换到ch2

```sh
git checkout ch2

防止拉不下来,用镜像
#原始仓库
#git clone https://github.com/LearningOS/rCore-Tutorial-Test-2025S.git user

git clone https://kkgithub.com/LearningOS/rCore-Tutorial-Test-2025S.git user

cd os
make run LOG=INFO
```

当前文件夹查找名字为`filename`的文件,`*`是通配符

```
find . -type f -name "*filename*"
find . -type f -name "*hello_world*"
```

## 2.3

### **`print_app_info`:打印应用程序信息**

```rust
pub fn print_app_info(&self) {
    println!("[kernel] num_app = {}", self.num_app);
    for i in 0..self.num_app {
        println!(
            "[kernel] app_{} [{:#x}, {:#x})",
            i,
            self.app_start[i],
            self.app_start[i + 1]
        );
    }
}
```

一个示例输出

```text
[kernel] num_app = 3
[kernel] app_0 [0x80200000, 0x80203000)
[kernel] app_1 [0x80203000, 0x80206000)
[kernel] app_2 [0x80206000, 0x80209000)
```

### **`load_app`**:**加载应用程序到内存**

```rust
unsafe fn load_app(&self, app_id: usize) {
    if app_id >= self.num_app {
        println!("All applications completed!");
        use crate::board::QEMUExit;
        crate::board::QEMU_EXIT_HANDLE.exit_success();
    }
    println!("[kernel] Loading app_{}", app_id);
    // 清空应用程序内存区域
    core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);
    // 获取应用程序二进制数据
    let app_src = core::slice::from_raw_parts(
        self.app_start[app_id] as *const u8,
        self.app_start[app_id + 1] - self.app_start[app_id],
    );
    // 复制到目标地址
    let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());
    app_dst.copy_from_slice(app_src);
    // 执行 `fence.i` 确保指令缓存同步
    asm!("fence.i");
}
```

1. **检查 `app_id` 是否合法**:
   - 如果 `app_id >= num_app`,说明所有程序已运行完毕,直接 **退出 QEMU**(用于测试环境).
2. **清空目标内存区域**:
   - `APP_BASE_ADDRESS` 是应用程序加载的目标地址(如 `0x80400000`).
   - `APP_SIZE_LIMIT` 是应用程序的最大允许大小.
   - 使用 `.fill(0)` 清零,避免残留数据影响新程序.
3. **从存储位置复制应用程序到目标内存**:
   - `app_src`:从 `app_start[app_id]` 读取应用程序的二进制数据.
   - `app_dst`:目标地址 `APP_BASE_ADDRESS`.
   - `copy_from_slice`:执行复制.
4. **执行 `fence.i` 指令**:
   - 确保 CPU 的 **指令缓存** 和 **内存** 同步(避免执行旧指令).

### **关键点**

- **`unsafe`**:因为涉及直接操作内存(`from_raw_parts`).
- **`fence.i`**:RISC-V 指令,用于 **指令缓存同步**(确保新加载的代码能被正确执行).
- **`APP_BASE_ADDRESS`**:通常是固定的地址(如 `0x80400000`),由链接脚本决定.



## **2.4**

解释以下

> 本章中我们仅考虑当 CPU 在 U 特权级运行用户程序的时候触发 Trap, 并切换到 S 特权级的批处理操作系统进行处理

### **1. 特权级(Privilege Levels)**

RISC-V 定义了多个特权级,常见的有:

- **U 模式(User Mode)**:运行用户程序,权限最低.
- **S 模式(Supervisor Mode)**:运行操作系统内核,可管理硬件资源.
- **M 模式(Machine Mode)**:最高权限,通常用于固件(如 Bootloader).

这里讨论的是 **U → S 的切换**,即用户程序触发 Trap 后,由操作系统的 S 模式处理.

### **2. Trap 的定义**

Trap 是 **从低特权级进入高特权级的异常控制流**,触发原因包括:

- **异常(Exception)**:如非法指令、除零、页错误等.
- **中断(Interrupt)**:如时钟中断、外设中断.
- **系统调用(ECALL)**:用户程序主动请求操作系统服务.



### **3. 本场景的具体含义**

> “仅考虑当 CPU 在 U 特权级运行用户程序的时候触发 Trap,并切换到 S 特权级的批处理操作系统进行处理.”

- **U → S 的 Trap**:
  - CPU 正在 **用户态(U 模式)** 执行应用程序.
  - 发生 Trap(如 `ecall` 或异常),硬件自动:
    1. 保存当前上下文(PC、寄存器等)到 **控制状态寄存器(如 `sepc`、`scause`)**.
    2. 切换到 **内核态(S 模式)**.
    3. 跳转到 **Trap 处理入口(`stvec` 寄存器指定的地址)**.
  - 操作系统(S 模式)开始处理 Trap:
    - 读取 `scause` 判断 Trap 原因.
    - 执行相应处理(如终止程序、返回错误、加载下一个程序).
- **批处理系统的特殊性**:
  - 每次 Trap 处理后,操作系统可能直接加载下一个程序(而不是返回原程序).
  - 不需要复杂的进程调度或上下文保存(因为程序是顺序执行的).

### **特权级切换相关的控制状态寄存器**

**CSR**(**Control and Status Register**,**控制与状态寄存器**)是 RISC-V 架构中用于管理 CPU 运行状态、配置硬件行为以及处理异常/中断的一类特殊寄存器.

**sstatus(Supervisor Status Register)**

 **sepc(Supervisor Exception Program Counter)**

 **scause(Supervisor Cause Register)**

 **stval(Supervisor Trap Value Register)**

**stvec(Supervisor Trap Vector Base Address Register)**

| CSR 名    | 该 CSR 与 Trap 相关的功能                                    |
| --------- | ------------------------------------------------------------ |
| `sstatus` | `SPP` 等字段给出 Trap 发生之前 CPU 处在哪个特权级(S/U)等信息 |
| `sepc`    | 当 Trap 是一个异常的时候,记录 Trap 发生之前执行的最后一条指令的地址 |
| `scause`  | 描述 Trap 的原因                                             |
| `stval`   | 给出 Trap 附加信息                                           |
| `stvec`   | 控制 Trap 处理代码的入口地址                                 |

## Trap 管理

### Trap 上下文的保存与恢复

**Trap(异常/中断)上下文保存与恢复** 的核心汇编逻辑,主要分为 `__alltraps`(保存上下文并跳转处理)和 `__restore`(恢复上下文并返回用户态)两部分.



### Trap 分发与处理 (`trap_handler` 函数)

`trap_handler` 是操作系统的 Trap 处理核心,负责根据 Trap 原因(如系统调用、内存错误、非法指令)进行分发和处理,并维护用户程序的执行上下文.

```rust
#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    // 读取 Trap 原因和附加信息
    let scause = scause::read();
    let stval = stval::read();
    
    match scause.cause() {
        // 处理系统调用 (Environment Call from U-mode)
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4; // ecall 指令占 4 字节,跳过以执行下一条指令
            // 提取系统调用参数并处理
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        // 处理存储错误(如页面错误)
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) => {
            println!("PageFault in application");
            run_next_app(); // 终止当前应用,加载下一个
        }
        // 处理非法指令
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("IllegalInstruction");
            run_next_app();
        }
        // 其他未处理的 Trap 类型触发 panic
        _ => panic!("Unsupported trap {:?}", scause.cause()),
    }
    cx // 返回修改后的上下文
}
```



### **系统调用实现**

### **`sys.wrtie`(**写标准输出**)**

```rust
//! File and filesystem-related syscalls

const FD_STDOUT: usize = 1;  // 标准输出的文件描述符(POSIX 标准)

/// write buf of length `len` to a file with `fd`
pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    trace!("kernel: sys_write");  // 调试日志
    match fd {
        FD_STDOUT => {  // 处理标准输出
            let slice = unsafe { core::slice::from_raw_parts(buf, len) };
            let str = core::str::from_utf8(slice).unwrap();
            print!("{}", str);
            len as isize  // 返回成功写入的字节数
        }
        _ => {  // 其他文件描述符不支持
            panic!("Unsupported fd in sys_write!");
        }
    }
}
```

#### **`sys_exit`(退出应用)**

```rust
pub fn sys_exit(xstate: i32) -> ! {
    println!("Application exited with code {}", xstate);
    run_next_app() // 切换到下一个应用,永不返回
}
```

- **设计意图**:
  - 用户程序通过此调用主动结束,批处理系统立即加载下一应用.
  - **`-> !`**:函数不会返回,直接跳转.

### **应用程序启动 (`run_next_app` 函数)**

#### **功能**

加载下一个应用程序的二进制代码,初始化用户态上下文,并通过 `__restore` 切换到用户态执行.

```rust
pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    let current_app = app_manager.get_current_app();
    unsafe { app_manager.load_app(current_app); } // 加载应用二进制
    app_manager.move_to_next_app(); // 更新索引
    drop(app_manager); // 显式释放锁

    extern "C" { fn __restore(cx_addr: usize); }
    unsafe {
        // 构造初始 Trap 上下文并压入内核栈
        let cx = KERNEL_STACK.push_context(
            TrapContext::app_init_context(APP_BASE_ADDRESS, USER_STACK.get_sp())
        );
        __restore(cx as *const _ as usize); // 恢复上下文并切换到用户态
    }
    panic!("Unreachable!"); // 理论上不会执行
}
```

#### **关键点**

- **`load_app`**:将应用程序二进制复制到 `APP_BASE_ADDRESS`(如 `0x80400000`),清空旧数据.
- **Trap 上下文初始化**:
  - **`app_init_context`**:
    - **`sepc`**:设置为应用入口地址(`APP_BASE_ADDRESS`).
    - **`sp`**:指向用户栈顶(`USER_STACK.get_sp()`).
    - **`sstatus`**:设置 `SPP` 为 `User`,使得 `sret` 后进入 U 模式.
- **`__restore` 调用**:
  - 将构造的上下文压入内核栈,调用 `__restore` 从该上下文恢复寄存器并执行 `sret`.

###  **Trap 上下文初始化 (`TrapContext::app_init_context`)**

```rust
impl TrapContext {
    pub fn app_init_context(entry: usize, sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User); // 设置返回后为 U 模式
        Self {
            x: [0; 32], // 通用寄存器初始化为 0
            sstatus,
            sepc: entry, // 入口地址
        }.set_sp(sp) // 设置用户栈指针
    }
    pub fn set_sp(&mut self, sp: usize) { self.x[2] = sp; }
}
```

#### **关键点**

- **`sstatus.SPP`**:设置为 `User`,确保 `sret` 后进入用户态.
- **`sepc`**:应用程序入口地址,通常是二进制文件的起始位置.
- **用户栈指针**:通过 `set_sp` 设置到 `x[2]`(即 `sp` 寄存器).



### **总结与示意图**

#### **Trap 处理流程**

1. **用户程序触发 Trap**(如 `ecall`):
   - 硬件保存上下文到内核栈,跳转到 `stvec` 指向的 `__alltraps`.
2. **保存寄存器**:`__alltraps` 将通用寄存器和 CSR 保存到 `TrapContext`.
3. **分发处理**:`trap_handler` 根据 `scause` 调用相应处理逻辑.
4. **恢复执行**:修改后的 `TrapContext` 通过 `__restore` 恢复,执行 `sret` 返回用户态.

#### **应用启动流程**

1. **加载二进制**:复制到固定地址 `APP_BASE_ADDRESS`.
2. **构造上下文**:初始化 `sepc`、`sp`、`sstatus`.
3. **切换特权级**:通过 `__restore` 和 `sret` 进入用户态.

#### **示意图**

```sh
用户程序
│
├─ ecall → Trap (U→S)
│  │
│  └─ 保存上下文 → trap_handler → syscall → sys_write/sys_exit
│     │
│     ├─ 写操作:输出到标准输出
│     └─ 退出:run_next_app → 加载新应用
│
└─ sret (S→U) → 执行下一条指令或新应用入口
```





## 补充

### **通用寄存器**



- - 程友好名称):`zero`、`ra`、`sp`、`a0`-`a7`、`t0`-`t6`、`s0`-`s11` 等(见下表).
- **位宽**:
  - 32 位架构(RV32):32 位
  - 64 位架构(RV64):64 位(寄存器仍叫 `x0`-`x31`,但可存储 64 位数据).

------

### **2. 寄存器功能详解**

| 寄存器  | ABI 别名   | 用途                                           | 是否调用保存(Call-Saved) | 备注                                                   |
| :------ | :--------- | :--------------------------------------------- | :------------------------- | :----------------------------------------------------- |
| **x0**  | `zero`     | **硬编码零**                                   | -                          | 读取始终返回 `0`,写入无效(用于丢弃结果或生成常量). |
| **x1**  | `ra`       | **返回地址** (Return Address)                  | ❌                          | 存储函数调用后的返回地址(由 `jal` 指令自动写入).    |
| **x2**  | `sp`       | **栈指针** (Stack Pointer)                     | ✅                          | 指向当前栈顶(必须按约定维护).                       |
| **x3**  | `gp`       | **全局指针** (Global Pointer)                  | -                          | 用于优化全局变量访问(可选使用).                     |
| **x4**  | `tp`       | **线程指针** (Thread Pointer)                  | -                          | 存储线程局部存储(TLS)的基地址.                      |
| **x5**  | `t0`       | **临时寄存器 0**                               | ❌                          | 临时存储,跨函数调用不保留.                           |
| **x6**  | `t1`       | **临时寄存器 1**                               | ❌                          | 同上.                                                 |
| **x7**  | `t2`       | **临时寄存器 2**                               | ❌                          | 同上.                                                 |
| **x8**  | `s0`/`fp`  | **保存寄存器 0** 或 **帧指针** (Frame Pointer) | ✅                          | 可保存长期变量,或用作栈帧基址(可选).               |
| **x9**  | `s1`       | **保存寄存器 1**                               | ✅                          | 跨函数调用需由被调用者保存.                           |
| **x10** | `a0`       | **函数参数 0** / **返回值 0**                  | ❌                          | 传递第一个参数或返回主值.                             |
| **x11** | `a1`       | **函数参数 1** / **返回值 1**                  | ❌                          | 传递第二个参数或返回副值.                             |
| **x12** | `a2`       | **函数参数 2**                                 | ❌                          | 传递第三个参数.                                       |
| **x13** | `a3`       | **函数参数 3**                                 | ❌                          | 传递第四个参数.                                       |
| **x14** | `a4`       | **函数参数 4**                                 | ❌                          | 传递第五个参数.                                       |
| **x15** | `a5`       | **函数参数 5**                                 | ❌                          | 传递第六个参数.                                       |
| **x16** | `a6`       | **函数参数 6**                                 | ❌                          | 传递第七个参数.                                       |
| **x17** | `a7`       | **函数参数 7** / **系统调用号**                | ❌                          | 传递第八个参数,或存储系统调用编号.                   |
| **x18** | `s2`       | **保存寄存器 2**                               | ✅                          | 跨函数调用需保存.                                     |
| **x19** | `s3`       | **保存寄存器 3**                               | ✅                          | 同上.                                                 |
| ...     | `s4`-`s11` | **保存寄存器 4-11**                            | ✅                          | 同上(x20-x31).                                      |
| **x28** | `t3`       | **临时寄存器 3**                               | ❌                          | 临时使用.                                             |
| **x29** | `t4`       | **临时寄存器 4**                               | ❌                          | 同上.                                                 |
| **x30** | `t5`       | **临时寄存器 5**                               | ❌                          | 同上.                                                 |
| **x31** | `t6`       | **临时寄存器 6**                               | ❌                          | 同上.                                                 |



### **3. 关键寄存器用途**

#### **(1) 特殊功能寄存器**

- **`x0` (zero)**:

  - 只读,恒为 `0`,常用于:
    - 清零操作:`addi x1, x0, 42`(将 42 存入 `x1`).
    - 丢弃结果:`add x0, x1, x2`(计算结果被忽略).

- **`x1` (ra)**:

  - 存储函数返回地址(由 `jal` 或 `jalr` 指令自动写入).

  - 示例:

    ```asm
    jal ra, foo  # 调用函数 foo,返回地址存入 ra
    ```

- **`x2` (sp)**:

  - 栈指针,必须由程序显式维护(如分配/释放栈空间):

    ```assembly
    addi sp, sp, -16  # 分配 16 字节栈空间
    ```

- **`x10-x11` (a0-a1)**:

  - 传递函数参数和返回值:

    ```assembly
    addi a0, x0, 42  # 参数 a0 = 42
    jal foo          # 调用函数 foo
    mv x1, a0        # 获取返回值
    ```

#### **(2) 调用约定(Calling Convention)**

- **临时寄存器 (`t0`-`t6`)**:
  - 调用者需保存(若需跨调用保留值).
- **保存寄存器 (`s0`-`s11`)**:
  - 被调用者需保存(若使用,需在返回前恢复原值).

------

### **4. 示例代码**

```asm
# 函数调用示例
func:
    addi sp, sp, -16   # 分配栈空间
    sw   ra, 12(sp)    # 保存返回地址
    sw   s0, 8(sp)     # 保存 s0
    add  s0, a0, a1    # 使用保存寄存器 s0
    lw   s0, 8(sp)     # 恢复 s0
    lw   ra, 12(sp)    # 恢复返回地址
    addi sp, sp, 16    # 释放栈空间
    ret                # 返回 (等价于 jalr x0, ra, 0)
```
