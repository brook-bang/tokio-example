#[cfg(all(
    tokio_unstable,
    tokio_taskdump,
    target_os = "linux",
    any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
))]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //复制粘贴导入声明
    use std::sync::Arc;
    use tokio::sync::Barrier;

    #[inline(never)]
    async fn a(barrier: Arc<Barrier>) {
        b(barrier).await
    }

    #[inline(never)]
    async fn b(barrier: Arc<Barrier>) {
        c(barrier).await
    }

    #[inline(never)]
    async fn c(barrier: Arc<Barrier>) {
        barrier.wait().await;
    }

    async fn dump_or_quit() {
        //复制粘贴导入声明
        use tokio::time::{Duration, Instant, timeout};
        let handle = tokio::runtime::Handle::current();
        let mut last_signal: Option<Instant> = None;
        while let Ok(_) = tokio::signal::ctrl_c().await {
            if let Some(time_since_last_signal) = last_signal.map(|i| i.elapsed()) {
                if time_since_last_signal < Duration::from_secs(1) {
                    return;
                }
            }
            last_signal = Some(Instant::now());

            println!("{:-<80}", "");
            if let Ok(dump) = timeout(Duration::from_secs(2), handle.dump()).await {
                for task in dump.tasks().iter() {
                    let id = task.id();
                    let trace = task.trace();
                    println!("TASK {id}:");
                    println!("{trace}\n");
                }
            } else {
                println!("任务转储超时。请使用本地调试器(如gdb)调试死锁");
            }
            println!("{:-<80}","");
            println!("在一秒内输入两次CTRL+C以退出");
        }
    }

    println!("这个程序存在死锁");
    println!("输入CTRL+C以打印任务转储");
    println!("在一秒内输入两次CTRL+C以退出。");

    let barrier = Arc::new(Barrier::new(3));
    let task_1 = tokio::spawn(a(barrier.clone()));
    let task_2 = tokio::spawn(a(barrier));

    tokio::select! {
        _ = dump_or_quit() => {},
        _ = task_1 => {},
        _ = task_2 => {},
    };

    Ok(())
}

#[cfg(not(all(
    tokio_unstable,
    tokio_taskdump,
    target_os = "linux",
    any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64")
)))]
fn main() {
    println!("任务转储不可用")
}
