use tokio::sync::Semaphore;
use std::sync::Arc;
use futures::stream::StreamExt;
use tokio::time::{timeout, Duration};


async fn limited_task(id: usize, semaphore: &Semaphore) {
        // 获取许可
        let permit = semaphore.acquire().await.unwrap();
        
        println!("任务 {} 获得许可，开始执行", id);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("任务 {} 完成", id);
        
        // 释放许可（通过 drop）
        drop(permit);
}

pub async fn test_sephone() {
     const MAX_CONCURRENT_TASKS: usize = 3;
    const TOTAL_TASKS: usize = 10;
    
    // 创建信号量，限制并发任务数量
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_TASKS));
    
    println!("启动 {} 个任务，最大并发数为 {}", TOTAL_TASKS, MAX_CONCURRENT_TASKS);
    
    // 创建多个任务
    let mut handles = vec![];
    for i in 0..TOTAL_TASKS {
        let sem_clone = semaphore.clone();
        handles.push(tokio::spawn(async move {
            limited_task(i, &sem_clone).await;
        }));
    }
    
    // 等待所有任务完成
    for handle in handles {
        handle.await.unwrap();
    }
    
    println!("所有任务已完成");
}

async fn async_range(from:i32, to:i32) -> impl tokio_stream::Stream<Item = i32>{
    println!("从{} 到 {}", from, to);
    tokio_stream::iter((from..to).map(|i| i))
}

pub async fn test_streamExt() {
    let mut stream = async_range(1,5).await;
    while let Some(i) = stream.next().await {
        println!("获取到值：{}", i);
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
    println!("流已耗尽力");
}

async fn potentially_slow_operation() ->Result<i32, &'static str> {
    tokio::time::sleep(Duration::from_secs(2)).await;
    Ok(42)
}

async fn with_timeout_and_retry() -> Result<i32, &'static str> {
    const RETRY_COUNT: usize = 3;
    const TIMEOUT_DURATION: Duration = Duration::from_secs(1);

    for attempt in 1..= RETRY_COUNT {
        match timeout(TIMEOUT_DURATION, potentially_slow_operation()).await {
            Ok(result) => {
                return result;
            }
            Err(_) => {
                println!("操作超时，准备重试...");
            }
        }
    }

    Err("尝试多次失败")
}


pub async fn retry_timeout() {
    match with_timeout_and_retry().await {
        Ok(result) => {
            println!("操作成功，结果为：{}", result);
        }
        Err(error) => {
            println!("操作失败：{}", error);
        }
    }
}

fn process_data(data: &mut Vec<i32>) {
    // 处理数据
    data.iter().for_each(|&value| {
        println!("Processing value: {}", value);
    })
}

pub fn test_vec() {
    let mut numbers = vec![1, 2, 3];
    // let first = &numbers[0];
    process_data(&mut numbers); // 编译错误：cannot borrow `numbers` as mutable
    println!("First element: {}", numbers[0]);
}