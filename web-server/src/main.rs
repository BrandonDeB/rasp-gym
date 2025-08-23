use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use time::Date;

mod mongo;

#[derive(Deserialize)]
struct WorkoutRequest {
    owner_id: i32,
}

#[derive(Deserialize)]
struct ExerciseRequest {
    owner_id: i32,
    date: Date,
}

#[derive(Serialize)]
struct Workout {
    id: i32,
    template_id: i32,
    owner_id: i32,
    date: Option<Date>,
}

#[derive(Serialize)]
struct Exercise {
    id: i32,
    name: String,
    date: Option<Date>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let client = mongo::create_client()
        .await
        .expect("Create client failed");
    
    HttpServer::new(move || {
        App::new()
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
