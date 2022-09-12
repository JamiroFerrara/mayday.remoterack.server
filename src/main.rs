use server::Listener::*;
use server::Updater::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    update()?;
    tokio_main()?;
    Ok(())
}

#[tokio::main]
async fn tokio_main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting server...");
    // let ip = "localhost".to_string();
    let ip = "172.105.66.226".to_string();
    let port = 8080;
    let listener = Listener::new(2, ip , port).await;
    listener.start().await; //This is blocking

    Ok(())
}
