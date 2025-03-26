# chp1

## 1.1

`SBI` (Supervisor Binary Interface) 是 RISC-V 架构中定义的一种 supervisor 模式下的二进制接口规范,用于操作系统与底层固件(如 OpenSBI)之间的通信.

`riscv64gc-unknown-none-elf` 的 CPU 架构是 riscv64gc,厂商是 unknown,操作系统是 none, elf 表示没有标准的运行时库.没有任何系统调用的封装支持,但可以生成 ELF 格式的执行程序.

## 1.3

`#[no_mangle]` 是 Rust 中的一个 属性宏(attribute),用于告诉编译器不要对函数或静态变量的名称进行名称修饰(name mangling),禁止 Rust 编译器重命名函数,使其在编译后的二进制文件中保持原始名称,以便其他语言(如 C/C++)或外部工具可以通过符号名直接调用.

[文件格式]

```sh
`$ file target/riscv64gc-unknown-none-elf/debug/os`
```

[文件头信息]

```sh
$ rust-readobj -h target/riscv64gc-unknown-none-elf/debug/os
```

[反汇编导出汇编程序]

```sh
$ rust-objdump -S target/riscv64gc-unknown-none-elf/debug/os
```

`qemu-riscv64 target/riscv64gc-unknown-none-elf/debug/os`命令可以执行这个程序

```rust

```

## ELF文件

### **1. ELF 文件**

ELF 文件主要分为三类:

1. **可执行文件(Executable)**
   - 可直接运行的程序(如 `/bin/ls`).
   - 示例:你的 `os` 二进制文件.
2. **共享库(Shared Object)**
   - 动态链接库(如 `.so`、`.dll`).
3. **目标文件(Relocatable)**
   - 编译生成的中间文件(`.o`),需进一步链接.

---

### **2. ELF 文件结构**

ELF 文件由 **头部 + 节(Sections)+ 段(Segments)** 组成:

| 组成部分                 | 作用                                                         |
| :----------------------- | :----------------------------------------------------------- |
| **ELF Header**           | 描述文件类型(如可执行文件/RISC-V 架构)、入口地址(`_start`)、节/段信息位置. |
| **Program Headers(段)** | 告诉操作系统如何加载程序(如代码段、数据段).                |
| **Section Headers(节)** | 包含代码、数据、符号表等(供链接器使用).                    |

## 1.5
用 QEMU 软件 qemu-system-riscv64 来模拟 RISC-V 64 计算机.加载内核程序的命令如下:

```sh
qemu-system-riscv64 \
            -machine virt \
            -nographic \
            -bios $(BOOTLOADER) \
            -device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)
```