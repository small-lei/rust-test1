mod test_func;
mod api;
mod database;
mod middleware;

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()
            .add_directive("middleware=info".parse()?))
        .init();
    let public_router = api::create_public_router();
    let private_router = api::create_private_router()
        .layer(tower::ServiceBuilder::new()
            .layer(axum::middleware::from_fn(middleware::auth::auth_middleware)));
    let app = public_router.merge(private_router);
    
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn test_parallel_sum() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let sum = test_func::parallel_sum::calculate_parallel_sum().await?;
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
