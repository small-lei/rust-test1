use std::path::Path;
use std::sync::Arc;
use tokio::{fs::File, io::{AsyncReadExt, AsyncWriteExt, AsyncSeekExt}, sync::{Semaphore, mpsc}};

const CHUNK_SIZE: usize = 1024 * 1024; // 1MB per chunk
const MAX_CONCURRENT: usize = 4;

pub async fn process_file_concurrent(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT));
    let (tx, mut rx) = mpsc::channel(MAX_CONCURRENT);

    let file_size = File::open(input_path).await?.metadata().await?.len();
    let mut handles = vec![];

    for chunk_index in 0..(file_size as usize / CHUNK_SIZE + 1) {
        let permit = semaphore.clone().acquire_owned().await?;
        let tx = tx.clone();
        let input = input_path.to_string();

        let handle = tokio::task::spawn(async move {
            let mut file = File::open(&input).await?;
            let start = chunk_index * CHUNK_SIZE;
            let end = std::cmp::min(start + CHUNK_SIZE, file_size as usize);
            let mut buffer = vec![0u8; end - start];

            file.seek(std::io::SeekFrom::Start(start as u64)).await?;
            file.read_exact(&mut buffer).await?;

            tx.send((chunk_index, buffer)).await?;
            drop(permit);
            Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
        });

        handles.push(handle);
    }

    drop(tx);

    let mut chunks = std::collections::BTreeMap::new();
    let mut output_file = File::create(output_path).await?;

    let mut next_chunk_to_write = 0;
    while let Some((index, data)) = rx.recv().await {
        chunks.insert(index, data);

        while chunks.contains_key(&next_chunk_to_write) {
            let data = chunks.remove(&next_chunk_to_write).unwrap();
            output_file.write_all(&data).await?;
            next_chunk_to_write += 1;
        }
    }

    for handle in handles {
        handle.await??;
    }

    Ok(())
}