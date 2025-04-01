use tokio::sync::mpsc;
use std::sync::Arc;

const CHUNK_SIZE: usize = 10000;
const MAX_CONCURRENT: usize = 10;

pub async fn calculate_parallel_sum() -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    let (tx, mut rx) = mpsc::channel(MAX_CONCURRENT);
    let mut handles = vec![];
    let total_numbers = 100000;

    for chunk_index in 0..(total_numbers / CHUNK_SIZE + 1) {
        let tx = tx.clone();
        let start = chunk_index * CHUNK_SIZE + 1;
        let end = std::cmp::min(start + CHUNK_SIZE - 1, total_numbers);

        let handle = tokio::spawn(async move {
            let sum: u64 = (start..=end).map(|x| x as u64).sum();
            tx.send(sum).await?;
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
        });

        handles.push(handle);
    }

    drop(tx);

    // 初始化总和为0
    let mut total_sum: u64 = 0;
    // 通过channel接收每个工作线程计算的部分和
    // 当channel关闭且所有消息都被接收后，循环会自动退出
    while let Some(partial_sum) = rx.recv().await {
        // 将接收到的部分和累加到总和中
        total_sum += partial_sum;
    }

    // 等待所有任务完成
    // 使用双问号运算符(??)处理两层Result:
    // 1. handle.await 返回 Result<Result<_, Box<dyn Error>>, JoinError>
    // 2. 第一个?处理JoinError，第二个?处理内部Result的错误
    for handle in handles {
        handle.await??;
    }

    Ok(total_sum)
}