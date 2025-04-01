mod test_func;

use test_func::test1::{self, *};
use test_func::sephone::{self, *};
use test_func::parallel_sum;

/// This function is a placeholder for a test function.//+
/////+
/// # Purpose//+
/////+
/// The purpose of this function is to demonstrate how to add documentation comments to a Rust function.//+
/////+
/// # Parameters//+
/////+
/// This function does not take any parameters.//+
/////+
/// # Return Value//+
/////+
/// This function does not return any value. It simply prints a message to the console.//+
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    test_file_processor().await?;
    test_slice();
    test_parallel_sum().await?;
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
