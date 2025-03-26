# chp3

## 3.1
>**栈通常是从高地址向低地址增长的,所以栈顶指针指向栈空间的最高地址之后的位置**

**栈的生长方向：**

栈是一种后进先出(LIFO)的数据结构,通常从高地址向低地址方向增长(这是大多数系统的实现方式,比如x86架构).

例如,如果栈的内存范围是 0x1000 到 0x2000(高地址是 0x2000),那么栈会从 0x2000 开始,向 0x1000 方向生长.

**栈顶指针的初始位置：**

栈顶指针(如x86的 ESP 或 RSP)通常指向当前栈顶元素.

在栈初始化时(空栈),栈顶指针会指向栈空间的最高地址的下一个位置(即“栈空间的最高地址 + 1”).这是因为：

当第一个元素入栈时,栈顶指针会先递减(因为栈向低地址生长),然后再存储数据.

例如,如果栈空间是 0x1000 到 0x2000,初始时栈顶指针可能是 0x2004(假设指针本身是32位,+4字节),但更常见的初始值是 0x2000(栈的最高可用地址),然后第一次 push 时会先减 4(如 sub esp, 4),再写入数据.

**关键点：**

向低地址生长：每次 push 操作会先减少栈顶指针,再存储数据;pop 操作会先读取数据,再增加栈顶指针.

初始栈顶指针的位置：指向栈空间最高地址的“下一个位置”是为了方便第一个 push 操作可以直接通过递减指针来定位到栈的起始可用地址.


讲解 loader.rs

```rust
/// Load nth user app at
/// [APP_BASE_ADDRESS + n * APP_SIZE_LIMIT, APP_BASE_ADDRESS + (n+1) * APP_SIZE_LIMIT).
pub fn load_apps() {
    extern "C" {
        fn _num_app();
    }
    let num_app_ptr = _num_app as usize as *const usize;
    let num_app = get_num_app();
    let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };
    // load apps
    for i in 0..num_app {
        let base_i = get_base_i(i);
        // clear region
        (base_i..base_i + APP_SIZE_LIMIT)
            .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });
        // load app from data section to memory
        let src = unsafe {
            core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i])
        };
        let dst = unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };
        dst.copy_from_slice(src);
    }
    // Memory fence about fetching the instruction memory
    // It is guaranteed that a subsequent instruction fetch must
    // observes all previous writes to the instruction memory.
    // Therefore, fence.i must be executed after we have loaded
    // the code of the next app into the instruction memory.
    // See also: riscv non-priv spec chapter 3, 'Zifencei' extension.
    unsafe {
        asm!("fence.i");
    }
}
```
```
load_apps 函数的作用：将第 n 个用户应用程序加载到指定的内存范围.
pub fn load_apps() { ... }: 定义了一个名为 load_apps 的公共函数,它没有参数也没有返回值.
extern "C" { fn _num_app(); }: 再次声明外部函数 _num_app.
let num_app_ptr = _num_app as usize as *const usize;: 获取 _num_app 的地址,并将其转换为一个指向 usize 的常量指针.
let num_app = get_num_app();: 调用 get_num_app 函数获取应用程序的总数量.
let app_start = unsafe { core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1) };: 这部分用于获取每个应用程序在内核二进制文件中的起始地址和结束地址.
num_app_ptr.add(1): 在 num_app_ptr 指向的地址之后的一个位置,通常会存储第一个应用程序的起始地址.假设在 _num_app 之后依次存储了每个应用程序的起始地址,最后一个是所有应用程序数据的结束地址.
core::slice::from_raw_parts(..., num_app + 1): 创建一个裸指针切片 app_start.这个切片包含了 num_app + 1 个 usize 类型的值,其中 app_start[i] 存储了第 i 个应用程序的起始地址,而 app_start[num_app] 存储了最后一个应用程序的结束地址(或者下一个应用程序的起始地址,如果存在的话).
for i in 0..num_app { ... }: 循环遍历所有的应用程序.
let base_i = get_base_i(i);: 计算当前应用程序 i 在内存中的基地址.
(base_i..base_i + APP_SIZE_LIMIT).for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });: 这部分代码用于清除当前应用程序将要加载到的内存区域,将其所有字节都设置为 0.
(base_i..base_i + APP_SIZE_LIMIT): 生成一个从 base_i 到 base_i + APP_SIZE_LIMIT 的地址范围.
.for_each(|addr| ...): 对这个范围内的每个地址执行闭包中的代码.
unsafe { (addr as *mut u8).write_volatile(0) }: 将当前地址 addr 转换为一个指向 u8 的可变指针,并使用 write_volatile 将该地址的内存设置为 0.write_volatile 确保写入操作不会被优化掉.
let src = unsafe { core::slice::from_raw_parts(app_start[i] as *const u8, app_start[i + 1] - app_start[i]) };: 这部分代码创建了一个裸指针切片 src,它指向当前应用程序在内核二进制文件中的数据.
app_start[i] as *const u8: 获取当前应用程序的起始地址,并将其转换为一个指向 u8 的常量指针.
app_start[i + 1] - app_start[i]: 计算当前应用程序的大小(结束地址减去起始地址).
core::slice::from_raw_parts(...): 使用起始地址和大小创建一个裸指针切片 src.
let dst = unsafe { core::slice::from_raw_parts_mut(base_i as *mut u8, src.len()) };: 这部分代码创建了一个裸指针切片 dst,它指向当前应用程序在内存中将要存储的位置.
base_i as *mut u8: 获取当前应用程序在内存中的基地址,并将其转换为一个指向 u8 的可变指针.
src.len(): 获取源数据的大小.
core::slice::from_raw_parts_mut(...): 使用基地址和大小创建一个可变裸指针切片 dst.
dst.copy_from_slice(src);: 将源数据 src 复制到目标内存区域 dst.
unsafe { asm!("fence.i"); }: 这部分代码嵌入了一个 RISC-V 汇编指令 fence.i.这是一个指令缓存刷新指令,用于确保在加载完应用程序代码后,后续的指令获取操作能够看到最新的代码.这在修改了指令内存后是必要的.注释中也解释了其原因,并提到了 RISC-V 非特权规范中的 'Zifencei' 扩展.
```


## 3.2
### 任务切换的设计与实现
>任务切换与上一章提及的 Trap 控制流切换相比,有如下异同：与 Trap 切换不同,它不涉及特权级切换,部分由编译器完成;与 Trap 切换相同,它对应用是透明的.

>事实上,任务切换是来自两个不同应用在内核中的 Trap 控制流之间的切换. 当一个应用 Trap 到 S 态 OS 内核中进行进一步处理时, 其 Trap 控制流可以调用一个特殊的 `__switch` 函数. 在 `__switch` 返回之后,Trap 控制流将继续从调用该函数的位置继续向下执行. 而在调用 `__switch` 之后到返回前的这段时间里, 原 Trap 控制流 A 会先被暂停并被切换出去, CPU 转而运行另一个应用的 Trap 控制流 B . `__switch` 返回之后,原 Trap 控制流 A 才会从某一条 Trap 控制流 C 切换回来继续执行.

**`__switch` 函数的作用：**

特殊的 `__switch` 函数： 这是一个关键的函数,负责实际的任务切换操作.当内核决定切换到另一个应用程序时,当前应用程序 A 的 Trap 控制流会调用 `__switch` 函数.
**``__switch`` 函数的执行过程：**

调用 ``__switch`` 之后到返回前： 在调用 `__switch` 到它返回的这段时间内,会发生以下事情：

原 Trap 控制流 A 会先被暂停并被切换出去： `__switch` 函数会保存当前应用程序 A 的执行状态(即任务上下文).
CPU 转而运行另一个应用的 Trap 控制流 B： `__switch` 函数会加载另一个应用程序 B 之前保存的执行状态,使得 CPU 开始执行应用程序 B 的代码.
`__switch` 返回之后： 当 CPU 再次需要执行应用程序 A 时,会发生以下事情：

原 Trap 控制流 A 才会从某一条 Trap 控制流 C 切换回来继续执行： 这里提到的 "某一条 Trap 控制流 C" 可能指代的是触发切换回应用程序 A 的那个应用程序的 Trap 控制流.更直观地理解,当应用程序 B 的执行需要暂停,并切换回应用程序 A 时,应用程序 B 的 Trap 控制流也会调用 `__switch`,将 CPU 的控制权交还给之前保存的应用程序 A 的上下文.
继续从调用该函数的位置继续向下执行： 当 `__switch` 函数返回时,应用程序 A 的 Trap 控制流会从它之前调用 `__switch` 函数的位置继续往下执行,就好像中间的切换过程没有发生过一样.

**任务上下文 (Task Context)：**

我们需要在 `__switch` 中保存 CPU 的某些寄存器,它们就是任务上下文 (Task Context). 为了能够正确地暂停一个应用程序的执行并在之后恢复,操作系统需要保存该应用程序在被暂停时的 CPU 状态.这个状态主要包括 CPU 的一些关键寄存器的值,例如：

通用寄存器： 存储程序运行时的各种数据.\
程序计数器 (PC)： 指示下一条要执行的指令的地址.\
栈指针 (SP)： 指向当前栈顶的位置.\
帧指针 (FP)： 用于管理函数调用的栈帧(可能需要保存).\
状态寄存器： 存储 CPU 的各种状态标志.\
``__switch`` 函数的核心任务就是将当前应用程序的这些关键寄存器的值保存到一个特定的数据结构中(通常是任务控制块 TCB),然后从另一个应用程序的任务控制块中加载之前保存的寄存器值,从而完成任务的切换.

**表格对比**
| **对比维度**     | **任务切换 (Task Switch)**               | **Trap 切换 (Trap Switch)**                    |
| ---------------- | ---------------------------------------- | ---------------------------------------------- |
| **触发原因**     | 主动调用 ``__switch`` 或调度器触发         | 中断、异常、系统调用等硬件/软件事件触发        |
| **特权级变化**   | 无(均在内核态 S 下切换)                | 必须切换(如 U→S 或 S→M)                      |
| **硬件参与**     | 仅依赖软件保存上下文                     | 硬件自动保存部分上下文(如 `sepc`, `scause`)  |
| **编译器参与**   | 需生成寄存器保存/恢复代码(如 `s0-s11`) | 无直接参与,由硬件和内核协作完成               |
| **上下文内容**   | 任务上下文(`sp`, `ra`, `s0-s11` 等)    | Trap 帧(`pc`, `x1-x31`, `sstatus`, `scause`) |
| **对应用透明性** | 透明(应用无感知)                       | 透明(应用无感知)                             |
| **典型场景**     | 多任务调度(时间片轮转、阻塞唤醒)       | 系统调用、页错误、时钟中断处理                 |
| **切换代价**     | 较低(无特权级切换,仅寄存器操作)       | 较高(需硬件保存状态,可能刷新 TLB/缓存)      |
| **返回行为**     | 可能从任意其他任务切换回来(如 A→B→C→A) | 必须返回到原触发点(如用户态系统调用后返回)   |


## 3.4

### 程序主动暂停 sys_yield 和主动退出 sys_exit

