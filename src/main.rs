mod servers;
mod server;

#[tokio::main]
async fn main() {
    println!("Nordiumm Monitor");

    let status = servers::agoniasmp::check().await;
    println!("{:?}", status);
}