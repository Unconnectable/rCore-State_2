```sh
以下是我的ci日志
Run qemu-system-riscv64 --version
QEMU emulator version 7.0.0
info: syncing channel updates for 'nightly-2024-05-02-x86_64-unknown-linux-gnu'
Copyright (c) 2003-2022 Fabrice Bellard and the QEMU Project developers
info: latest update on 2024-05-02, rust version 1.80.0-nightly (c987ad527 2024-05-01)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'llvm-tools'
info: downloading component 'rust-src'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'llvm-tools'
info: installing component 'rust-src'
info: installing component 'rust-std'
info: installing component 'rustc'
info: installing component 'rustfmt'
info: downloading component 'rust-std' for 'riscv64gc-unknown-none-elf'
info: installing component 'rust-std' for 'riscv64gc-unknown-none-elf'
Cloning into 'ci-user'...
Cloning into 'ci-user/user'...
     Removed 0 files
     Removed 0 files
    Updating `rsproxy` index
From https://rsproxy.cn/crates.io-index
 * [new ref]                    -> origin/HEAD
 Downloading crates ...
  Downloaded cargo-binutils v0.3.6 (registry `rsproxy`)
  Installing cargo-binutils v0.3.6
     Locking 58 packages to latest compatible versions
      Adding bitflags v1.3.2 (latest: v2.9.0)
      Adding cargo-platform v0.1.9 (latest: v0.2.0)
      Adding cargo_metadata v0.14.2 (latest: v0.19.2)
      Adding clap v2.34.0 (latest: v4.5.34)
      Adding hermit-abi v0.1.19 (latest: v0.5.0)
      Adding rustc-cfg v0.4.0 (latest: v0.5.0)
      Adding strsim v0.8.0 (latest: v0.11.1)
      Adding syn v1.0.109 (latest: v2.0.100)
      Adding synstructure v0.12.6 (latest: v0.13.1)
      Adding textwrap v0.11.0 (latest: v0.16.2)
      Adding toml v0.5.11 (latest: v0.8.20)
      Adding unicode-width v0.1.14 (latest: v0.2.0)
      Adding windows-targets v0.52.6 (latest: v0.53.0)
      Adding windows_aarch64_gnullvm v0.52.6 (latest: v0.53.0)
      Adding windows_aarch64_msvc v0.52.6 (latest: v0.53.0)
      Adding windows_i686_gnu v0.52.6 (latest: v0.53.0)
      Adding windows_i686_gnullvm v0.52.6 (latest: v0.53.0)
      Adding windows_i686_msvc v0.52.6 (latest: v0.53.0)
      Adding windows_x86_64_gnu v0.52.6 (latest: v0.53.0)
      Adding windows_x86_64_gnullvm v0.52.6 (latest: v0.53.0)
      Adding windows_x86_64_msvc v0.52.6 (latest: v0.53.0)
 Downloading crates ...
  Downloaded adler2 v2.0.0 (registry `rsproxy`)
  Downloaded backtrace v0.3.74 (registry `rsproxy`)
  Downloaded cfg-if v1.0.0 (registry `rsproxy`)
  Downloaded addr2line v0.24.2 (registry `rsproxy`)
  Downloaded rustc-demangle v0.1.24 (registry `rsproxy`)
  Downloaded anyhow v1.0.97 (registry `rsproxy`)
  Downloaded semver v1.0.26 (registry `rsproxy`)
  Downloaded syn v1.0.109 (registry `rsproxy`)
  Downloaded textwrap v0.11.0 (registry `rsproxy`)
  Downloaded regex v1.11.1 (registry `rsproxy`)
  Downloaded atty v0.2.14 (registry `rsproxy`)
  Downloaded quote v1.0.40 (registry `rsproxy`)
  Downloaded miniz_oxide v0.8.5 (registry `rsproxy`)
  Downloaded serde v1.0.219 (registry `rsproxy`)
  Downloaded cargo_metadata v0.14.2 (registry `rsproxy`)
  Downloaded unicode-xid v0.2.6 (registry `rsproxy`)
  Downloaded itoa v1.0.15 (registry `rsproxy`)
  Downloaded ryu v1.0.20 (registry `rsproxy`)
  Downloaded regex-automata v0.4.9 (registry `rsproxy`)
  Downloaded serde_derive v1.0.219 (registry `rsproxy`)
  Downloaded clap v2.34.0 (registry `rsproxy`)
  Downloaded gimli v0.31.1 (registry `rsproxy`)
  Downloaded rustc_version v0.4.1 (registry `rsproxy`)
  Downloaded vec_map v0.8.2 (registry `rsproxy`)
  Downloaded libc v0.2.171 (registry `rsproxy`)
  Downloaded memchr v2.7.4 (registry `rsproxy`)
  Downloaded unicode-ident v1.0.18 (registry `rsproxy`)
  Downloaded cargo-platform v0.1.9 (registry `rsproxy`)
  Downloaded synstructure v0.12.6 (registry `rsproxy`)
  Downloaded rustc-cfg v0.4.0 (registry `rsproxy`)
  Downloaded ansi_term v0.12.1 (registry `rsproxy`)
  Downloaded aho-corasick v1.1.3 (registry `rsproxy`)
  Downloaded serde_json v1.0.140 (registry `rsproxy`)
  Downloaded toml v0.5.11 (registry `rsproxy`)
  Downloaded proc-macro2 v1.0.94 (registry `rsproxy`)
  Downloaded syn v2.0.100 (registry `rsproxy`)
  Downloaded failure_derive v0.1.8 (registry `rsproxy`)
  Downloaded object v0.36.7 (registry `rsproxy`)
  Downloaded strsim v0.8.0 (registry `rsproxy`)
  Downloaded camino v1.1.9 (registry `rsproxy`)
  Downloaded regex-syntax v0.8.5 (registry `rsproxy`)
  Downloaded failure v0.1.8 (registry `rsproxy`)
  Downloaded unicode-width v0.1.14 (registry `rsproxy`)
  Downloaded bitflags v1.3.2 (registry `rsproxy`)
   Compiling proc-macro2 v1.0.94
   Compiling unicode-ident v1.0.18
   Compiling memchr v2.7.4
   Compiling serde v1.0.219
   Compiling libc v0.2.171
   Compiling syn v1.0.109
   Compiling object v0.36.7
   Compiling quote v1.0.40
   Compiling syn v2.0.100
   Compiling gimli v0.31.1
   Compiling unicode-xid v0.2.6
   Compiling adler2 v2.0.0
   Compiling failure_derive v0.1.8
   Compiling semver v1.0.26
   Compiling miniz_oxide v0.8.5
   Compiling serde_derive v1.0.219
   Compiling addr2line v0.24.2
   Compiling synstructure v0.12.6
   Compiling serde_json v1.0.140
   Compiling camino v1.1.9
   Compiling rustc-demangle v0.1.24
   Compiling cfg-if v1.0.0
   Compiling aho-corasick v1.1.3
   Compiling ryu v1.0.20
   Compiling backtrace v0.3.74
   Compiling itoa v1.0.15
   Compiling anyhow v1.0.97
   Compiling unicode-width v0.1.14
   Compiling regex-syntax v0.8.5
   Compiling cargo-platform v0.1.9
   Compiling regex-automata v0.4.9
   Compiling textwrap v0.11.0
   Compiling failure v0.1.8
   Compiling atty v0.2.14
   Compiling ansi_term v0.12.1
   Compiling vec_map v0.8.2
   Compiling bitflags v1.3.2
   Compiling strsim v0.8.0
   Compiling regex v1.11.1
   Compiling clap v2.34.0
   Compiling rustc-cfg v0.4.0
   Compiling cargo_metadata v0.14.2
   Compiling rustc_version v0.4.1
   Compiling toml v0.5.11
   Compiling cargo-binutils v0.3.6
    Finished `release` profile [optimized] target(s) in 1m 12s
  Installing /usr/local/cargo/bin/cargo-cov
  Installing /usr/local/cargo/bin/rust-cov
   Replacing /usr/local/cargo/bin/cargo-nm
   Replacing /usr/local/cargo/bin/cargo-objcopy
   Replacing /usr/local/cargo/bin/cargo-objdump
   Replacing /usr/local/cargo/bin/cargo-profdata
   Replacing /usr/local/cargo/bin/cargo-readobj
   Replacing /usr/local/cargo/bin/cargo-size
   Replacing /usr/local/cargo/bin/cargo-strip
   Replacing /usr/local/cargo/bin/rust-ar
   Replacing /usr/local/cargo/bin/rust-ld
   Replacing /usr/local/cargo/bin/rust-lld
   Replacing /usr/local/cargo/bin/rust-nm
   Replacing /usr/local/cargo/bin/rust-objcopy
   Replacing /usr/local/cargo/bin/rust-objdump
   Replacing /usr/local/cargo/bin/rust-profdata
   Replacing /usr/local/cargo/bin/rust-readobj
   Replacing /usr/local/cargo/bin/rust-size
   Replacing /usr/local/cargo/bin/rust-strip
   Installed package `cargo-binutils v0.3.6` (executables `cargo-cov`, `rust-cov`)
    Replaced package `cargo-binutils v0.2.0` with `cargo-binutils v0.3.6` (executables `cargo-nm`, `cargo-objcopy`, `cargo-objdump`, `cargo-profdata`, `cargo-readobj`, `cargo-size`, `cargo-strip`, `rust-ar`, `rust-ld`, `rust-lld`, `rust-nm`, `rust-objcopy`, `rust-objdump`, `rust-profdata`, `rust-readobj`, `rust-size`, `rust-strip`)
info: component 'rust-src' is up to date
info: component 'llvm-tools' for target 'x86_64-unknown-linux-gnu' is up to date
     Removed 0 files
    Updating `rsproxy` index
     Locking 8 packages to latest compatible versions
      Adding bitflags v1.3.2 (latest: v2.9.0)
      Adding buddy_system_allocator v0.6.0 (latest: v0.11.0)
      Adding lock_api v0.4.6 (latest: v0.4.12)
      Adding spin v0.7.1 (latest: v0.10.0)
      Adding spin v0.9.8 (latest: v0.10.0)
 Downloading crates ...
  Downloaded lazy_static v1.5.0 (registry `rsproxy`)
  Downloaded spin v0.7.1 (registry `rsproxy`)
  Downloaded buddy_system_allocator v0.6.0 (registry `rsproxy`)
  Downloaded lock_api v0.4.6 (registry `rsproxy`)
  Downloaded spin v0.9.8 (registry `rsproxy`)
  Downloaded scopeguard v1.2.0 (registry `rsproxy`)
   Compiling scopeguard v1.2.0
   Compiling spin v0.7.1
   Compiling bitflags v1.3.2
   Compiling lock_api v0.4.6
   Compiling buddy_system_allocator v0.6.0
   Compiling spin v0.9.8
   Compiling lazy_static v1.5.0
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 4.22s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.21s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.21s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.21s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.22s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.22s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.22s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.21s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.21s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.23s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.22s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.22s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.22s
    Updating `rsproxy` index
From https://rsproxy.cn/crates.io-index
 + c773f3dc68...f53470a37e            -> origin/HEAD  (forced update)
    Updating git repository `https://github.com/rcore-os/riscv`
From https://github.com/rcore-os/riscv
 * [new ref]                    -> origin/HEAD
     Locking 19 packages to latest compatible versions
      Adding bare-metal v0.2.5 (latest: v1.0.0)
      Adding bitflags v1.3.2 (latest: v2.9.0)
      Adding buddy_system_allocator v0.6.0 (latest: v0.11.0)
      Adding rustc_version v0.2.3 (latest: v0.4.1)
      Adding semver v0.9.0 (latest: v1.0.26)
      Adding semver-parser v0.7.0 (latest: v0.10.3)
      Adding spin v0.7.1 (latest: v0.10.0)
      Adding spin v0.9.8 (latest: v0.10.0)
 Downloading crates ...
  Downloaded bare-metal v0.2.5 (registry `rsproxy`)
  Downloaded bit_field v0.10.2 (registry `rsproxy`)
  Downloaded rustc_version v0.2.3 (registry `rsproxy`)
  Downloaded semver-parser v0.7.0 (registry `rsproxy`)
  Downloaded riscv-target v0.1.2 (registry `rsproxy`)
  Downloaded semver v0.9.0 (registry `rsproxy`)
  Downloaded log v0.4.27 (registry `rsproxy`)
   Compiling memchr v2.7.4
   Compiling regex-syntax v0.8.5
   Compiling semver-parser v0.7.0
   Compiling lazy_static v1.5.0
   Compiling spin v0.9.8
   Compiling spin v0.7.1
   Compiling semver v0.9.0
   Compiling os v0.1.0 (/__w/2025s-rcore-Unconnectable/2025s-rcore-Unconnectable/os)
   Compiling aho-corasick v1.1.3
   Compiling rustc_version v0.2.3
   Compiling log v0.4.27
   Compiling bare-metal v0.2.5
   Compiling bit_field v0.10.2
   Compiling bitflags v1.3.2
   Compiling buddy_system_allocator v0.6.0
   Compiling regex-automata v0.4.9
   Compiling regex v1.11.1
   Compiling riscv-target v0.1.2
   Compiling riscv v0.6.0 (https://github.com/rcore-os/riscv#11d43cf7)
    Finished `release` profile [optimized] target(s) in 7.94s
make: *** [Makefile:120: test] Error 1
Error: Process completed with exit code 2.

以下是正确运行的ci日志
Run qemu-system-riscv64 --version
QEMU emulator version 7.0.0
Copyright (c) 2003-2022 Fabrice Bellard and the QEMU Project developers
info: syncing channel updates for 'nightly-2024-05-02-x86_64-unknown-linux-gnu'
info: latest update on 2024-05-02, rust version 1.80.0-nightly (c987ad527 2024-05-01)
info: downloading component 'cargo'
info: retrying download for 'https://rsproxy.cn/dist/2024-05-02/cargo-nightly-x86_64-unknown-linux-gnu.tar.xz'
info: downloading component 'clippy'
info: downloading component 'llvm-tools'
info: downloading component 'rust-src'
info: downloading component 'rust-std' for 'riscv64gc-unknown-none-elf'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: retrying download for 'https://rsproxy.cn/dist/2024-05-02/rustc-nightly-x86_64-unknown-linux-gnu.tar.xz'
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'llvm-tools'
info: installing component 'rust-src'
info: installing component 'rust-std' for 'riscv64gc-unknown-none-elf'
info: installing component 'rust-std'
info: installing component 'rustc'
info: installing component 'rustfmt'
info: component 'rust-std' for target 'riscv64gc-unknown-none-elf' is up to date
Cloning into 'ci-user'...
Cloning into 'ci-user/user'...
     Removed 0 files
     Removed 0 files
    Updating `rsproxy` index
From https://rsproxy.cn/crates.io-index
 * [new ref]                    -> origin/HEAD
 Downloading crates ...
  Downloaded cargo-binutils v0.3.6 (registry `rsproxy`)
  Installing cargo-binutils v0.3.6
     Locking 58 packages to latest compatible versions
      Adding bitflags v1.3.2 (latest: v2.9.0)
      Adding cargo-platform v0.1.9 (latest: v0.2.0)
      Adding cargo_metadata v0.14.2 (latest: v0.19.2)
      Adding clap v2.34.0 (latest: v4.5.32)
      Adding hermit-abi v0.1.19 (latest: v0.5.0)
      Adding rustc-cfg v0.4.0 (latest: v0.5.0)
      Adding strsim v0.8.0 (latest: v0.11.1)
      Adding syn v1.0.109 (latest: v2.0.100)
      Adding synstructure v0.12.6 (latest: v0.13.1)
      Adding textwrap v0.11.0 (latest: v0.16.2)
      Adding toml v0.5.11 (latest: v0.8.20)
      Adding unicode-width v0.1.14 (latest: v0.2.0)
      Adding windows-targets v0.52.6 (latest: v0.53.0)
      Adding windows_aarch64_gnullvm v0.52.6 (latest: v0.53.0)
      Adding windows_aarch64_msvc v0.52.6 (latest: v0.53.0)
      Adding windows_i686_gnu v0.52.6 (latest: v0.53.0)
      Adding windows_i686_gnullvm v0.52.6 (latest: v0.53.0)
      Adding windows_i686_msvc v0.52.6 (latest: v0.53.0)
      Adding windows_x86_64_gnu v0.52.6 (latest: v0.53.0)
      Adding windows_x86_64_gnullvm v0.52.6 (latest: v0.53.0)
      Adding windows_x86_64_msvc v0.52.6 (latest: v0.53.0)
 Downloading crates ...
  Downloaded adler2 v2.0.0 (registry `rsproxy`)
  Downloaded backtrace v0.3.74 (registry `rsproxy`)
  Downloaded addr2line v0.24.2 (registry `rsproxy`)
  Downloaded anyhow v1.0.97 (registry `rsproxy`)
  Downloaded cfg-if v1.0.0 (registry `rsproxy`)
  Downloaded object v0.36.7 (registry `rsproxy`)
  Downloaded serde_json v1.0.140 (registry `rsproxy`)
  Downloaded camino v1.1.9 (registry `rsproxy`)
  Downloaded failure_derive v0.1.8 (registry `rsproxy`)
  Downloaded gimli v0.31.1 (registry `rsproxy`)
  Downloaded strsim v0.8.0 (registry `rsproxy`)
  Downloaded bitflags v1.3.2 (registry `rsproxy`)
  Downloaded atty v0.2.14 (registry `rsproxy`)
  Downloaded serde_derive v1.0.219 (registry `rsproxy`)
  Downloaded miniz_oxide v0.8.5 (registry `rsproxy`)
  Downloaded ryu v1.0.20 (registry `rsproxy`)
  Downloaded unicode-width v0.1.14 (registry `rsproxy`)
  Downloaded quote v1.0.40 (registry `rsproxy`)
  Downloaded libc v0.2.171 (registry `rsproxy`)
  Downloaded semver v1.0.26 (registry `rsproxy`)
  Downloaded aho-corasick v1.1.3 (registry `rsproxy`)
  Downloaded vec_map v0.8.2 (registry `rsproxy`)
  Downloaded clap v2.34.0 (registry `rsproxy`)
  Downloaded cargo_metadata v0.14.2 (registry `rsproxy`)
  Downloaded syn v2.0.100 (registry `rsproxy`)
  Downloaded proc-macro2 v1.0.94 (registry `rsproxy`)
  Downloaded regex-syntax v0.8.5 (registry `rsproxy`)
  Downloaded itoa v1.0.15 (registry `rsproxy`)
  Downloaded unicode-ident v1.0.18 (registry `rsproxy`)
  Downloaded synstructure v0.12.6 (registry `rsproxy`)
  Downloaded unicode-xid v0.2.6 (registry `rsproxy`)
  Downloaded rustc_version v0.4.1 (registry `rsproxy`)
  Downloaded rustc-demangle v0.1.24 (registry `rsproxy`)
  Downloaded textwrap v0.11.0 (registry `rsproxy`)
  Downloaded failure v0.1.8 (registry `rsproxy`)
  Downloaded toml v0.5.11 (registry `rsproxy`)
  Downloaded regex v1.11.1 (registry `rsproxy`)
  Downloaded ansi_term v0.12.1 (registry `rsproxy`)
  Downloaded syn v1.0.109 (registry `rsproxy`)
  Downloaded regex-automata v0.4.9 (registry `rsproxy`)
  Downloaded serde v1.0.219 (registry `rsproxy`)
  Downloaded rustc-cfg v0.4.0 (registry `rsproxy`)
  Downloaded cargo-platform v0.1.9 (registry `rsproxy`)
  Downloaded memchr v2.7.4 (registry `rsproxy`)
   Compiling proc-macro2 v1.0.94
   Compiling unicode-ident v1.0.18
   Compiling memchr v2.7.4
   Compiling serde v1.0.219
   Compiling libc v0.2.171
   Compiling syn v1.0.109
   Compiling object v0.36.7
   Compiling quote v1.0.40
   Compiling syn v2.0.100
   Compiling failure_derive v0.1.8
   Compiling adler2 v2.0.0
   Compiling unicode-xid v0.2.6
   Compiling semver v1.0.26
   Compiling gimli v0.31.1
   Compiling miniz_oxide v0.8.5
   Compiling serde_derive v1.0.219
   Compiling addr2line v0.24.2
   Compiling synstructure v0.12.6
   Compiling camino v1.1.9
   Compiling cfg-if v1.0.0
   Compiling rustc-demangle v0.1.24
   Compiling serde_json v1.0.140
   Compiling aho-corasick v1.1.3
   Compiling backtrace v0.3.74
   Compiling ryu v1.0.20
   Compiling anyhow v1.0.97
   Compiling itoa v1.0.15
   Compiling unicode-width v0.1.14
   Compiling regex-syntax v0.8.5
   Compiling cargo-platform v0.1.9
   Compiling regex-automata v0.4.9
   Compiling textwrap v0.11.0
   Compiling failure v0.1.8
   Compiling atty v0.2.14
   Compiling bitflags v1.3.2
   Compiling vec_map v0.8.2
   Compiling strsim v0.8.0
   Compiling ansi_term v0.12.1
   Compiling clap v2.34.0
   Compiling regex v1.11.1
   Compiling rustc-cfg v0.4.0
   Compiling cargo_metadata v0.14.2
   Compiling rustc_version v0.4.1
   Compiling toml v0.5.11
   Compiling cargo-binutils v0.3.6
    Finished `release` profile [optimized] target(s) in 1m 17s
  Installing /usr/local/cargo/bin/cargo-cov
  Installing /usr/local/cargo/bin/rust-cov
   Replacing /usr/local/cargo/bin/cargo-nm
   Replacing /usr/local/cargo/bin/cargo-objcopy
   Replacing /usr/local/cargo/bin/cargo-objdump
   Replacing /usr/local/cargo/bin/cargo-profdata
   Replacing /usr/local/cargo/bin/cargo-readobj
   Replacing /usr/local/cargo/bin/cargo-size
   Replacing /usr/local/cargo/bin/cargo-strip
   Replacing /usr/local/cargo/bin/rust-ar
   Replacing /usr/local/cargo/bin/rust-ld
   Replacing /usr/local/cargo/bin/rust-lld
   Replacing /usr/local/cargo/bin/rust-nm
   Replacing /usr/local/cargo/bin/rust-objcopy
   Replacing /usr/local/cargo/bin/rust-objdump
   Replacing /usr/local/cargo/bin/rust-profdata
   Replacing /usr/local/cargo/bin/rust-readobj
   Replacing /usr/local/cargo/bin/rust-size
   Replacing /usr/local/cargo/bin/rust-strip
   Installed package `cargo-binutils v0.3.6` (executables `cargo-cov`, `rust-cov`)
    Replaced package `cargo-binutils v0.2.0` with `cargo-binutils v0.3.6` (executables `cargo-nm`, `cargo-objcopy`, `cargo-objdump`, `cargo-profdata`, `cargo-readobj`, `cargo-size`, `cargo-strip`, `rust-ar`, `rust-ld`, `rust-lld`, `rust-nm`, `rust-objcopy`, `rust-objdump`, `rust-profdata`, `rust-readobj`, `rust-size`, `rust-strip`)
info: component 'rust-src' is up to date
info: component 'llvm-tools' for target 'x86_64-unknown-linux-gnu' is up to date
     Removed 0 files
    Updating `rsproxy` index
     Locking 8 packages to latest compatible versions
      Adding bitflags v1.3.2 (latest: v2.9.0)
      Adding buddy_system_allocator v0.6.0 (latest: v0.11.0)
      Adding lock_api v0.4.6 (latest: v0.4.12)
      Adding spin v0.7.1 (latest: v0.9.8)
 Downloading crates ...
  Downloaded spin v0.7.1 (registry `rsproxy`)
  Downloaded lock_api v0.4.6 (registry `rsproxy`)
  Downloaded lazy_static v1.5.0 (registry `rsproxy`)
  Downloaded scopeguard v1.2.0 (registry `rsproxy`)
  Downloaded spin v0.9.8 (registry `rsproxy`)
  Downloaded buddy_system_allocator v0.6.0 (registry `rsproxy`)
   Compiling scopeguard v1.2.0
   Compiling spin v0.7.1
   Compiling bitflags v1.3.2
   Compiling lock_api v0.4.6
   Compiling buddy_system_allocator v0.6.0
   Compiling spin v0.9.8
   Compiling lazy_static v1.5.0
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 5.78s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.22s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.22s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.22s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.23s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.23s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.23s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.22s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.22s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.23s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.23s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.22s
   Compiling user_lib v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user)
    Finished `release` profile [optimized] target(s) in 0.23s
    Updating `rsproxy` index
    Updating git repository `https://github.com/rcore-os/riscv`
From https://github.com/rcore-os/riscv
 * [new ref]                    -> origin/HEAD
     Locking 29 packages to latest compatible versions
      Adding bare-metal v0.2.5 (latest: v1.0.0)
      Adding bitflags v1.3.2 (latest: v2.9.0)
      Adding rustc_version v0.2.3 (latest: v0.4.1)
      Adding semver v0.9.0 (latest: v1.0.26)
      Adding semver-parser v0.7.0 (latest: v0.10.3)
 Downloading crates ...
  Downloaded autocfg v1.4.0 (registry `rsproxy`)
  Downloaded rustversion v1.0.20 (registry `rsproxy`)
  Downloaded lock_api v0.4.12 (registry `rsproxy`)
  Downloaded semver v0.9.0 (registry `rsproxy`)
  Downloaded strum v0.27.1 (registry `rsproxy`)
  Downloaded semver-parser v0.7.0 (registry `rsproxy`)
  Downloaded buddy_system_allocator v0.11.0 (registry `rsproxy`)
  Downloaded rustc_version v0.2.3 (registry `rsproxy`)
  Downloaded strum_macros v0.27.1 (registry `rsproxy`)
warning: spurious network error (3 tries remaining): [7] Couldn't connect to server (Failed to connect to lf6-static.rsproxy.cn port 443 after 3739 ms: Couldn't connect to server)
warning: spurious network error (3 tries remaining): [7] Couldn't connect to server (Failed to connect to lf6-static.rsproxy.cn port 443 after 4002 ms: Couldn't connect to server)
warning: spurious network error (3 tries remaining): [7] Couldn't connect to server (Failed to connect to lf6-static.rsproxy.cn port 443 after 0 ms: Couldn't connect to server)
warning: spurious network error (3 tries remaining): [7] Couldn't connect to server (Failed to connect to lf6-static.rsproxy.cn port 443 after 0 ms: Couldn't connect to server)
warning: spurious network error (3 tries remaining): [7] Couldn't connect to server (Failed to connect to lf6-static.rsproxy.cn port 443 after 0 ms: Couldn't connect to server)
warning: spurious network error (2 tries remaining): [7] Couldn't connect to server (Failed to connect to lf6-static.rsproxy.cn port 443 after 528 ms: Couldn't connect to server)
warning: spurious network error (2 tries remaining): [7] Couldn't connect to server (Failed to connect to lf6-static.rsproxy.cn port 443 after 268 ms: Couldn't connect to server)
warning: spurious network error (2 tries remaining): [7] Couldn't connect to server (Failed to connect to lf6-static.rsproxy.cn port 443 after 0 ms: Couldn't connect to server)
warning: spurious network error (2 tries remaining): [7] Couldn't connect to server (Failed to connect to lf6-static.rsproxy.cn port 443 after 0 ms: Couldn't connect to server)
  Downloaded bare-metal v0.2.5 (registry `rsproxy`)
  Downloaded riscv-target v0.1.2 (registry `rsproxy`)
warning: spurious network error (1 tries remaining): [7] Couldn't connect to server (Failed to connect to lf6-static.rsproxy.cn port 443 after 0 ms: Couldn't connect to server)
warning: spurious network error (1 tries remaining): [7] Couldn't connect to server (Failed to connect to lf6-static.rsproxy.cn port 443 after 0 ms: Couldn't connect to server)
  Downloaded heck v0.5.0 (registry `rsproxy`)
  Downloaded bit_field v0.10.2 (registry `rsproxy`)
  Downloaded log v0.4.27 (registry `rsproxy`)
   Compiling memchr v2.7.4
   Compiling regex-syntax v0.8.5
   Compiling proc-macro2 v1.0.94
   Compiling semver-parser v0.7.0
   Compiling autocfg v1.4.0
   Compiling semver v0.9.0
   Compiling aho-corasick v1.1.3
   Compiling lock_api v0.4.12
   Compiling unicode-ident v1.0.18
   Compiling rustc_version v0.2.3
   Compiling scopeguard v1.2.0
   Compiling rustversion v1.0.20
   Compiling lazy_static v1.5.0
   Compiling quote v1.0.40
   Compiling bare-metal v0.2.5
   Compiling spin v0.9.8
   Compiling regex-automata v0.4.9
   Compiling syn v2.0.100
   Compiling heck v0.5.0
   Compiling bitflags v1.3.2
   Compiling os v0.1.0 (/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/os)
   Compiling bit_field v0.10.2
   Compiling log v0.4.27
   Compiling buddy_system_allocator v0.11.0
   Compiling regex v1.11.1
   Compiling strum_macros v0.27.1
   Compiling riscv-target v0.1.2
   Compiling riscv v0.6.0 (https://github.com/rcore-os/riscv#11d43cf7)
   Compiling strum v0.27.1
    Finished `release` profile [optimized] target(s) in 25.00s
Backing up original files to temp-* dirs
make[1]: Entering directory '/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/os'
cargo clean
make[1]: Leaving directory '/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/os'
make[1]: Entering directory '/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user'
make[1]: Leaving directory '/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user'
(rustup target list | grep "riscv64gc-unknown-none-elf (installed)") || rustup target add riscv64gc-unknown-none-elf
riscv64gc-unknown-none-elf (installed)
cargo install cargo-binutils
rustup component add rust-src
rustup component add llvm-tools-preview
find user/src/bin -name "*.rs" | xargs -I {} sh -c 'sed -i.bak 's/OK/OK43625/g' {} && rm -rf {}.bak'
find user/src/bin -name "*.rs" | xargs -I {} sh -c 'sed -i.bak 's/passed/passed43625/g' {} && rm -rf {}.bak'
find check -name "*.py" | xargs -I {} sh -c 'sed -i.bak 's/OK/OK43625/g' {} && rm -rf {}.bak'
find check -name "*.py" | xargs -I {} sh -c 'sed -i.bak 's/passed/passed43625/g' {} && rm -rf {}.bak'
python3 overwrite.py 3
make -C user build BASE=2 TEST=3 CHAPTER=3
make[1]: Entering directory '/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user'
target/riscv64gc-unknown-none-elf/release/ch2b_power_7 target/riscv64gc-unknown-none-elf/release/ch2b_power_3 target/riscv64gc-unknown-none-elf/release/ch2b_bad_instructions target/riscv64gc-unknown-none-elf/release/ch2b_power_5 target/riscv64gc-unknown-none-elf/release/ch2b_bad_register target/riscv64gc-unknown-none-elf/release/ch2b_hello_world target/riscv64gc-unknown-none-elf/release/ch2b_bad_address target/riscv64gc-unknown-none-elf/release/ch3b_yield0 target/riscv64gc-unknown-none-elf/release/ch3_sleep1 target/riscv64gc-unknown-none-elf/release/ch3b_yield2 target/riscv64gc-unknown-none-elf/release/ch3_trace target/riscv64gc-unknown-none-elf/release/ch3_sleep target/riscv64gc-unknown-none-elf/release/ch3b_yield1
[build.py] application ch2b_bad_address start with address 0x80400000
[build.py] application ch2b_bad_instructions start with address 0x80420000
[build.py] application ch2b_bad_register start with address 0x80440000
[build.py] application ch2b_hello_world start with address 0x80460000
[build.py] application ch2b_power_3 start with address 0x80480000
[build.py] application ch2b_power_5 start with address 0x804a0000
[build.py] application ch2b_power_7 start with address 0x804c0000
[build.py] application ch3_sleep start with address 0x804e0000
[build.py] application ch3_sleep1 start with address 0x80500000
[build.py] application ch3_trace start with address 0x80520000
[build.py] application ch3b_yield0 start with address 0x80540000
[build.py] application ch3b_yield1 start with address 0x80560000
[build.py] application ch3b_yield2 start with address 0x80580000
make[1]: Leaving directory '/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/ci-user/user'
make -C ../os run OFFLINE= | tee stdout-ch3
make[1]: Entering directory '/__w/2025s-rcore-Kh05ifr4nD/2025s-rcore-Kh05ifr4nD/os'
cargo build --release
timeout --foreground 30s qemu-system-riscv64 \
	-machine virt \
	-nographic \
	-bios ../bootloader/rustsbi-qemu.bin \
	-kernel target/riscv64gc-unknown-none-elf/release/os
[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0
```





```
以下是我的cargo.toml文件
[package]
name = "os"
version = "0.1.0"
authors = ["Yifan Wu <shinbokuow@163.com>"]
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
buddy_system_allocator = "0.6"
lazy_static = { version = "1.4.0", features = ["spin_no_std"] }
log = "0.4"
riscv = { git = "https://github.com/rcore-os/riscv", features = ["inline-asm"] }

这是success的toml
[package]
name = "os"
version = "0.1.0"
authors = ["Yifan Wu <shinbokuow@163.com>"]
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
riscv = { git = "https://github.com/rcore-os/riscv", features = ["inline-asm"] }
buddy_system_allocator = { default-features = false, features = [
  "use_spin",
], version = "0.11" }
lazy_static = { default-features = false, features = [
  "spin_no_std",
], version = "1.5.0" }
log = "0.4"
strum = { default-features = false, features = ["derive"], version = "0.27" }

当我把buddy_system改为0.11时,也就是以下这样
[dependencies]
buddy_system_allocator = "0.11"
lazy_static = { version = "1.4.0", features = ["spin_no_std"] } 其他没有修改
出现了以下的错误
   --> src/heap_alloc.rs:8:24
    |
8   | static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
    |                        ^^^^^^^^^^ expected 1 generic argument
    |
note: struct defined here, with 1 generic parameter: `ORDER`
   --> /home/phrink/.cargo/registry/src/index.crates.io-6f17d22bba15001f/buddy_system_allocator-0.11.0/src/lib.rs:244:12
    |
244 | pub struct LockedHeap<const ORDER: usize>(Mutex<Heap<ORDER>>);
    |            ^^^^^^^^^^ ------------------
help: add missing generic argument
    |
8   | static HEAP_ALLOCATOR: LockedHeap<ORDER> = LockedHeap::empty();
    |                                  +++++++

但是我按照编译器的提示区修改也是不对的
```

