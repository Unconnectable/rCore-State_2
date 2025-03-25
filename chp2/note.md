# chp2

把ch1修改的文件`commit`后切换到ch2

```sh
git checkout ch2

防止拉不下来，用镜像
#原始仓库
#git clone https://github.com/LearningOS/rCore-Tutorial-Test-2025S.git user

git clone https://kkgithub.com/LearningOS/rCore-Tutorial-Test-2025S.git user

cd os
make run LOG=INFO
```

我的输出

```sh
[rustsbi] RustSBI version 0.3.0-alpha.4, adapting to RISC-V SBI v1.0.0
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
[rustsbi] Platform Name      : riscv-virtio,qemu
[rustsbi] Platform SMP       : 1
[rustsbi] Platform Memory    : 0x80000000..0x88000000
[rustsbi] Boot HART          : 0
[rustsbi] Device Tree Region : 0x87000000..0x87000ef2
[rustsbi] Firmware Address   : 0x80000000
[rustsbi] Supervisor Address : 0x80200000
[rustsbi] pmp01: 0x00000000..0x80000000 (-wr)
[rustsbi] pmp02: 0x80000000..0x80200000 (---)
[rustsbi] pmp03: 0x80200000..0x88000000 (xwr)
[rustsbi] pmp04: 0x88000000..0x00000000 (-wr)
[kernel] Hello, world!
[ INFO] [kernel] .data [0x8020a000, 0x80227000)
[ WARN] [kernel] boot_stack top=bottom=0x80237000, lower_bound=0x80227000
[ERROR] [kernel] .bss [0x80237000, 0x80238000)
[kernel] num_app = 7
[kernel] app_0 [0x8020a048, 0x8020e0f0)
[kernel] app_1 [0x8020e0f0, 0x80212198)
[kernel] app_2 [0x80212198, 0x80216240)
[kernel] app_3 [0x80216240, 0x8021a2e8)
[kernel] app_4 [0x8021a2e8, 0x8021e390)
[kernel] app_5 [0x8021e390, 0x80222438)
[kernel] app_6 [0x80222438, 0x802264e0)
[kernel] Loading app_0
[kernel] PageFault in application, kernel killed it.
[kernel] Loading app_1
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] Loading app_2
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] Loading app_3
Hello, world from user mode program!
[kernel] Loading app_4
power_3 [10000/200000]
power_3 [20000/200000]
power_3 [30000/200000]
power_3 [40000/200000]
power_3 [50000/200000]
power_3 [60000/200000]
power_3 [70000/200000]
power_3 [80000/200000]
power_3 [90000/200000]
power_3 [100000/200000]
power_3 [110000/200000]
power_3 [120000/200000]
power_3 [130000/200000]
power_3 [140000/200000]
power_3 [150000/200000]
power_3 [160000/200000]
power_3 [170000/200000]
power_3 [180000/200000]
power_3 [190000/200000]
power_3 [200000/200000]
3^200000 = 871008973(MOD 998244353)
Test power_3 OK!
[kernel] Loading app_5
power_5 [10000/140000]
power_5 [20000/140000]
power_5 [30000/140000]
power_5 [40000/140000]
power_5 [50000/140000]
power_5 [60000/140000]
power_5 [70000/140000]
power_5 [80000/140000]
power_5 [90000/140000]
power_5 [100000/140000]
power_5 [110000/140000]
power_5 [120000/140000]
power_5 [130000/140000]
power_5 [140000/140000]
5^140000 = 386471875(MOD 998244353)
Test power_5 OK!
[kernel] Loading app_6
power_7 [10000/160000]
power_7 [20000/160000]
power_7 [30000/160000]
power_7 [40000/160000]
power_7 [50000/160000]
power_7 [60000/160000]
power_7 [70000/160000]
power_7 [80000/160000]
power_7 [90000/160000]
power_7 [100000/160000]
power_7 [110000/160000]
power_7 [120000/160000]
power_7 [130000/160000]
power_7 [140000/160000]
power_7 [150000/160000]
power_7 [160000/160000]
7^160000 = 667897727(MOD 998244353)
Test power_7 OK!
All applications completed!
```

