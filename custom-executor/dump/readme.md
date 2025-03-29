# Tokio Task Dump 示例项目

## 项目简介

这是 Tokio 官方仓库 中的一个[示例项目](https://github.com/tokio-rs/tokio/blob/master/examples/dump.rs)，展示了如何使用 Tokio 的实验性任务转储（task dump）功能来调试异步任务中的死锁问题。项目故意设计了一个死锁场景，通过捕获 `CTRL+C` 信号打印任务调用栈，帮助开发者分析异步任务的执行状态。本人完整地实现了这个项目，并在此记录运行方式。

### 功能

- **死锁演示**：创建两个异步任务，等待一个需要三个任务才能解除的 `Barrier`，导致死锁。
- **任务转储**：按下 `CTRL+C` 时，打印当前任务的调用栈。
- **退出机制**：在一秒内连续按两次 `CTRL+C` 退出程序。

### 技术细节

- **依赖**：Tokio（实验性特性 `tokio_unstable` 和 `tokio_taskdump`）。
- **条件编译**：仅在 Linux 系统（支持 `x86_64`、`x86`、`aarch64` 架构）上启用完整功能。

## 运行环境要求

- **操作系统**：Linux（`target_os = "linux"`）。
- **架构**：`x86_64`、`x86` 或 `aarch64`。
- **Rust 版本**：建议使用最新稳定版（通过 `rustup update` 更新）。
- **Tokio 版本**：需要支持 `tokio_taskdump` 的版本（例如 1.44.1）。

## 运行

### 1. 运行项目

在 Linux 环境下运行：

```bash
cargo run
```

如果条件编译未完全满足，可能需要强制启用 `cfg`：

```bash
RUSTFLAGS="--cfg tokio_unstable --cfg tokio_taskdump" cargo run
```

### 2.项目输出

1. 启动时：

   ```
   这个程序存在死锁
   输入CTRL+C以打印任务转储
   在一秒内输入两次CTRL+C以退出
   ```

2. 程序卡住（死锁）。
3. 按 `CTRL+C`，打印任务转储，例如：

   ```
   TASK 14:
       a -> b -> c -> barrier.wait
   TASK 15:
       a -> b -> c -> barrier.wait
   ```
实际上   
```
TASK 14:
╼ dump::main::{{closure}}::a::{{closure}} at /mnt/data/rust/tokio-example/custom-executor/dump/src/main.rs:15:20
  └╼ dump::main::{{closure}}::b::{{closure}} at /mnt/data/rust/tokio-example/custom-executor/dump/src/main.rs:20:20
     └╼ dump::main::{{closure}}::c::{{closure}} at /mnt/data/rust/tokio-example/custom-executor/dump/src/main.rs:25:24
        └╼ tokio::sync::barrier::Barrier::wait::{{closure}} at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/sync/barrier.rs:134:10
           └╼ <tokio::util::trace::InstrumentedAsyncOp<F> as core::future::future::Future>::poll at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/util/trace.rs:159:46
              └╼ tokio::sync::barrier::Barrier::wait_internal::{{closure}} at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/sync/barrier.rs:188:36
                 └╼ tokio::sync::watch::Receiver<T>::changed::{{closure}} at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/sync/watch.rs:747:68
                    └╼ <tokio::task::coop::Coop<F> as core::future::future::Future>::poll at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/task/coop/mod.rs:403:39
                       └╼ tokio::sync::watch::changed_impl::{{closure}} at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/sync/watch.rs:927:18
                          └╼ <tokio::sync::notify::Notified as core::future::future::Future>::poll at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/sync/notify.rs:1152:9
                             └╼ tokio::sync::notify::Notified::poll_notified at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/sync/notify.rs:1045:43

TASK 15:
╼ dump::main::{{closure}}::a::{{closure}} at /mnt/data/rust/tokio-example/custom-executor/dump/src/main.rs:15:20
  └╼ dump::main::{{closure}}::b::{{closure}} at /mnt/data/rust/tokio-example/custom-executor/dump/src/main.rs:20:20
     └╼ dump::main::{{closure}}::c::{{closure}} at /mnt/data/rust/tokio-example/custom-executor/dump/src/main.rs:25:24
        └╼ tokio::sync::barrier::Barrier::wait::{{closure}} at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/sync/barrier.rs:134:10
           └╼ <tokio::util::trace::InstrumentedAsyncOp<F> as core::future::future::Future>::poll at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/util/trace.rs:159:46
              └╼ tokio::sync::barrier::Barrier::wait_internal::{{closure}} at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/sync/barrier.rs:188:36
                 └╼ tokio::sync::watch::Receiver<T>::changed::{{closure}} at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/sync/watch.rs:747:68
                    └╼ <tokio::task::coop::Coop<F> as core::future::future::Future>::poll at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/task/coop/mod.rs:403:39
                       └╼ tokio::sync::watch::changed_impl::{{closure}} at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/sync/watch.rs:927:18
                          └╼ <tokio::sync::notify::Notified as core::future::future::Future>::poll at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/sync/notify.rs:1152:9
                             └╼ tokio::sync::notify::Notified::poll_notified at /home/dell/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/tokio-1.44.1/src/sync/notify.rs:1045:43
```

4. 一秒内再按 `CTRL+C`，程序退出。

## 注意事项

- **非 Linux 系统**：如果运行在 Windows 或 macOS 上，默认输出“任务转储不可用”，需切换到 Linux。
- **Tokio 版本**：`tokio_taskdump` 是实验性特性，可能在未来版本中变更，建议固定版本（如 1.44.1）。
- **调试**：任务转储显示了调用栈，可用于学习异步任务的执行流程。
