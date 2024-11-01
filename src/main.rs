use mongodb::{Client, bson::doc};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let uri = if args.len() > 1 { &args[1] } else { "mongodb://127.0.0.1:27017" };
    let client = Client::with_uri_str(uri).await.unwrap();
    let database = client.database("admin");
    println!("{}", database.run_command(doc! {"logRotate":"server"}).await.unwrap());
}
