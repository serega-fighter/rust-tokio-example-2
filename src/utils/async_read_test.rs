use tokio::io::{self, AsyncReadExt};
use tokio::fs::File;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut f = File::open("test.txt").await?;
    let mut buffer = Vec::new();

    // read the whole file
    f.read_to_end(&mut buffer).await?;

    println!("Read {:?} bytes", buffer.len());
    Ok(())
}