# 问答作业

## 1.实现的功能

在引入`PageTable`之后实现了`sys_trace`对`sys_call`的记录,重新获取`syts_gettime`,同样在`memory_set`文件中引入了`mmap munmap`函数,在`/task/mod.rs`中实现了对物理页的分配和取消映射

## 2.问答题

### 1.请列举 SV39 页表页表项的组成,描述其中的标志位有何作用？

组成:虚拟地址39位 低12位是偏移 高27位是虚拟页号VPN,物理地址56位 低12位是偏移 高44位是物理页号PPN

标志位:第八位[7:0]都是标志位,分别7到0是 D A G U RWX V

- 仅当 V(Valid) 位为 1 时,页表项才是合法的;
- R/W/X 分别控制索引到这个页表项的对应虚拟页面是否允许读/写/取指;
- U 控制索引到这个页表项的对应虚拟页面是否在 CPU 处于 U 特权级的情况下是否被允许访问;
- G 我们不理会;
- A(Accessed) 记录自从页表项上的这一位被清零之后,页表项的对应虚拟页面是否被访问过;
- D(Dirty) 则记录自从页表项上的这一位被清零之后,页表项的对应虚拟页表是否被修改过.



### 2.缺页

缺页指的是进程访问页面时页面不在页表中或在页表中无效的现象,此时 MMU 将会返回一个中断, 告知 os 进程内存访问出了问题.os 选择填补页表并重新执行异常指令或者杀死进程.

- 请问哪些异常可能是缺页导致的？
- 发生缺页时,描述相关重要寄存器的值,上次实验描述过的可以简略.

缺页有两个常见的原因,其一是 Lazy 策略,也就是直到内存页面被访问才实际进行页表操作. 比如,一个程序被执行时,进程的代码段理论上需要从磁盘加载到内存.但是 os 并不会马上这样做, 而是会保存 .text 段在磁盘的位置信息,在这些代码第一次被执行时才完成从磁盘的加载操作.

- 这样做有哪些好处？

其实,我们的 mmap 也可以采取 Lazy 策略,比如:一个用户进程先后申请了 10G 的内存空间, 然后用了其中 1M 就直接退出了.按照现在的做法,我们显然亏大了,进行了很多没有意义的页表操作.

- 处理 10G 连续的内存页面,对应的 SV39 页表大致占用多少内存 (估算数量级即- 可)？

- 请简单思考如何才能实现 Lazy 策略,缺页时又如何处理？描述合理即可,不需要考虑实现.缺页的另一个常见原因是 swap 策略,也就是内存页面可能被换到磁盘上了,导致对应页面失效.

- 此时页面失效如何表现在页表项(PTE)上?



---

```rust
    /// Get the syscall times of the current 'Running' task 🔴🔴🔴🔴🔴🔴🔴🔴🔴🔴
    pub fn get_syscall_times(&self, syscall_id: usize) -> isize {
        let inner = TASK_MANAGER.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].syscall_times[syscall_id] as isize
    }
```

`git rebase`的使用

```sh
git log -oneline #把 log 输出为一行

#查看log历史后使用rebase变基
filament@7945hx:~/Courses/2025s-rcore-Unconnectable$ git log --oneline
440d30a (HEAD -> ch4, origin/ch4) 完成ch4
7ce602c 修改reports
aef5b0d 测试ch4正确程度
f0df504 还差unmap1的测试
3bc5156 未完成
0b7c8e5 继续ch4
12c1e3d 还差map测试
1672914 测试ch4
ab507ee finish sys_trace
073b2ef 添加了reports
eed20af 注释代码
dbd3002 添加注释
4d98714 增加了很多注释
4014758 [chore] remove continue-on-error to confirm correctness
18b6e57 Initialize ch4

#比如需要保留 Initialize ch4 和[chore] remove continue-on-error to confirm correctness 
#把剩下的合并为一个commit

git rebase -i 4014758^
#进入todo页面
```

修改为以下代码

`pick 4014758`:保留` [chore] `commit 不变.

`pick 4d98714`:作为合并的基础 commit.

`squash dbd3002` 到 `squash 440d30a`:将后续 12 个 commit 合并到 `4d98714`,形成一个新 commit.

```math
pick 4014758 [chore] remove continue-on-error to confirm correctness
pick 4d98714 增加了很多注释
squash dbd3002 添加注释
squash eed20af 注释代码
squash 073b2ef 添加了reports
squash ab507ee finish sys_trace
squash 1672914 测试ch4
squash 12c1e3d 还差map测试
squash 0b7c8e5 继续ch4
squash 3bc5156 未完成
squash f0df504 还差unmap1的测试
squash aef5b0d 测试ch4正确程度
squash 7ce602c 修改reports
squash 440d30a 完成ch4
```

接下来需要编辑新的commit消息

```sh
完成 ch4 开发、测试及文档,包括 sys_trace 和实验报告
```

然后验证`git log --oneline`

输出为

```sh
9fba672 (HEAD -> ch4) 完成 ch4的作业 rebase之前的commit 还差实验报告
4014758 [chore] remove continue-on-error to confirm correctness
18b6e57 Initialize ch4
```

完成`rebase`后需要强制推送

```sh
git push --force
```

