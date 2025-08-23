use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let user = env::var("MONGO_USER").expect("Mongo user not set");
    let password = env::var("MONGO_PASSWORD").expect("Mongo password not set");
    let uri = format!("mongodb+srv://{user}:{password}@songguessr.quhlriw.mongodb.net/?retryWrites=true&w=majority&appName=SongGuessr");
  let mut client_options =
    ClientOptions::parse(uri)
      $.await?;

  // Set the server_api field of the client_options object to set the version of the Stable API on the client
  let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
  client_options.server_api = Some(server_api);

  // Get a handle to the cluster
  let client = Client::with_options(client_options)?;

  // Ping the server to see if you can connect to the cluster
  client
    .database("admin")
    .run_command(doc! {"ping": 1}, None)
    .await?;

  println!("Pinged your deployment. You successfully connected to MongoDB!");

  Ok(())
}
