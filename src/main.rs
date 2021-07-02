#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;
use std::error::Error;

use crate::models::{Person, Post};
use actix_web::web::Data;
use actix_web::{get, post, web, App, Either, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::fmt::Debug;

mod actions;
mod models;
mod schema;

type DbPool = Pool<ConnectionManager<PgConnection>>;
type WebResult<T> = Result<T, actix_web::Error>;

#[get("/post/{id}")]
async fn get_post(web::Path(path_id): web::Path<usize>) -> impl Responder {
    use self::schema::posts::dsl::*;

    let result = web::block::<_, Vec<Post>, _>(move || {
        let connection = establish_connection();

        posts
            .filter(id.eq(path_id as i32))
            .load::<Post>(&connection)
    })
    .await
    .map_err(|e| {
        eprintln!("{:?}", e);
        HttpResponse::InternalServerError().finish()
    })
    .map(|mut vec| {
        if vec.len() > 0 {
            Either::A(HttpResponse::Ok().json(vec.remove(0)))
        } else {
            Either::B(HttpResponse::NotFound().finish())
        }
    });

    result
}

#[get("/post")]
async fn get_posts() -> impl Responder {
    use crate::schema::posts::dsl::*;

    let results = web::block(|| {
        let connection = establish_connection();
        posts.load::<Post>(&connection)
    })
    .await
    .map_err(|e| {
        eprintln!("{:?}", e);
        HttpResponse::InternalServerError().finish()
    })
    .map(|vec| HttpResponse::Ok().json(vec));

    results
}

#[post("/post")]
async fn post_post(post: web::Json<Post>, data: Data<DbPool>) -> WebResult<HttpResponse> {
    let con = data.get().expect("could not get connection from pool");

    let p = web::block(move || actions::insert_post(&con, &post))
        .await
        .map_err(internal_server_error)?;

    Ok(HttpResponse::Ok().json(p))
}

#[get("/hugo")]
async fn get_hugo(pool: Data<DbPool>) -> WebResult<HttpResponse> {
    let con = pool.get().expect("Could not get db connection from pool");

    let hugo = web::block(move || actions::find_person_by_id(&con, "hugo"))
        .await
        .map_err(internal_server_error)?;

    match hugo {
        Some(p) => Ok(HttpResponse::Ok().json(p)),
        None => Ok(HttpResponse::NotFound()
            .body("Hugo was not found here. Consider creating him with the id 'hugo'.")),
    }
}

#[post("/hugo")]
async fn hugo_post(new_hugo: web::Json<Person>, pool: Data<DbPool>) -> WebResult<impl Responder> {
    let con = pool.get().expect("Could not get db connection from pool");

    let person = web::block(move || actions::update_person(&con, &new_hugo))
        .await
        .map_err(internal_server_error)?;
    Ok(HttpResponse::Ok().json(person))
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("Did not find Database URL");
    let manager = diesel::r2d2::ConnectionManager::<PgConnection>::new(url);

    let pool: DbPool = r2d2::Pool::new(manager).unwrap();

    println!("Started Server...");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(hugo_post)
            .service(get_hugo)
            .service(get_post)
            .service(post_post)
            .service(get_posts)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}

pub fn internal_server_error(err: impl Debug) -> HttpResponse {
    eprintln!("{:?}", err);
    HttpResponse::InternalServerError().finish()
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
