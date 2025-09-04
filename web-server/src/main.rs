use actix_web::{post, web, App, HttpServer, Result};
use mongodb::{bson::{self, doc}, options::UpdateOptions, Database};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;

mod mongo;

#[derive(Deserialize, Serialize)]
struct TodayFocus {
    upper: bool,
    lower: bool,
    core: bool,
}

#[derive(Deserialize, Serialize)]
struct Workout {
    date: String,
    day: String,
    week: u32,
    cardioTable: Vec<Vec<String>>,
    weightTable: Vec<Vec<String>>,
    todayFocus: TodayFocus
}

#[post("/log")]
async fn log_workout(info: web::Json<Workout>, db: web::Data<Database>) -> Result<String> {
    let collection = db.collection::<Workout>("Workouts");

    let doc = info.into_inner();
    let date = doc.date.clone();

    let filter = doc! { "date": &date };
    let options = UpdateOptions::builder().upsert(true).build();
    let workout_doc = bson::to_document(&doc)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Serialization failed"))?;

    collection
        .update_one(filter, doc! { "$set": workout_doc })
        .with_options(options)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(format!("Workout logged/updated for {}!", date))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let client = mongo::create_client()
        .await
        .expect("Create client failed");
    
    let db = client.database("GymTracker");

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .app_data(web::Data::new(db.clone()))
            .service(log_workout)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
