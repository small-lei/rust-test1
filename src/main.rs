mod test_func;
mod api;

use std::net::SocketAddr;
use axum::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = api::create_router();
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn test_parallel_sum() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sum = parallel_sum::calculate_parallel_sum().await?;
    println!("并发计算1到10万的数字之和: {}", sum);
    Ok(())
}

async fn test_file_processor() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    test_func::file_processor::process_file_concurrent("test.txt", "output.txt").await
}

fn test_slice() {
    // devlop ---迁出
    // 测试函数实现
    // pre分支
}
