use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use time::PrimitiveDateTime;

#[derive(Deserialize)]
struct WorkoutRequest {
    owner_id: i32,
}

#[derive(Serialize)]
struct Workout {
    id: i32,
    template_id: i32,
    owner_id: i32,
    created_at: Option<PrimitiveDateTime>,
}

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/workouts")]
pub async fn get_workouts(
    pool: web::Data<PgPool>,
    query: web::Query<WorkoutRequest>,
) -> impl Responder {
    let owner_id = query.owner_id;

    let result = sqlx::query_as!(
        Workout,
        r#"
        SELECT id, template_id, created_at, owner_id
        FROM workouts
        WHERE owner_id = $1
        ORDER BY created_at DESC
        LIMIT 5
        "#,
        owner_id
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(workouts) => HttpResponse::Ok().json(workouts),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch workouts")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to connect to the database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(greet)
            .service(get_workouts)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
