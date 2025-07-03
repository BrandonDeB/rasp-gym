use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use time::Date;

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

#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/exercises")]
pub async fn get_exercises(
    pool: web::Data<PgPool>,
    query: web::Query<ExerciseRequest>,
) -> impl Responder {
    let owner_id = query.owner_id;
    let date = query.date;
    let result = sqlx::query_as!(
        Exercise,
        r#"
        SELECT 
            e.id, 
            e.name, 
            DATE(w.created_at)
        FROM template_exercises te
        JOIN workouts w ON w.template_id = te.template_id
        JOIN exercises e ON e.id = te.exercise_id
        WHERE w.owner_id = $1 
          AND DATE(w.created_at) = $2
        "#,
        owner_id,
        date
    )
    .fetch_all(pool.get_ref())
    .await;

    match result {
        Ok(exercises) => HttpResponse::Ok().json(exercises),
        Err(e) => {
            eprintln!("Database error: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch workouts")
        }
    }
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
        SELECT id, template_id, DATE(created_at), owner_id
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
            .service(get_exercises)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
