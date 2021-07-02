#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;

use std::error::Error;
use std::sync::Mutex;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::{Person, Post};
use actix_web::{get, post, web, App, Either, HttpResponse, HttpServer, Responder};

struct AppState {
    posts: Mutex<Vec<Post>>,
    hugo: Mutex<Person>,
}

#[get("post")]
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

#[get("/post/{id}")]
async fn get_post(
    web::Path(id): web::Path<usize>,
    data: web::Data<AppState>,
) -> Either<impl Responder, impl Responder> {
    let posts = data.posts.lock().unwrap();

    match posts.get(id) {
        None => Either::A(HttpResponse::NotFound()),
        Some(post) => Either::B(post.clone()),
    }
}

#[post("/post")]
async fn post_post(mut post: web::Json<Post>, data: web::Data<AppState>) -> impl Responder {
    let mut posts = data.posts.lock().unwrap();
    post.id = posts.len() as i32;
    posts.push(post.clone());
    post
}

#[get("/hugo")]
async fn hugo(data: web::Data<AppState>) -> impl Responder {
    let hugo_person = data.hugo.lock().unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&*hugo_person).unwrap())
}

#[post("/hugo")]
async fn hugo_post(new_hugo: web::Json<Person>, data: web::Data<AppState>) -> impl Responder {
    let mut hugo_person = data.hugo.lock().unwrap();
    *hugo_person = new_hugo.clone();
    new_hugo
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /*let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:karl@localhost/karlheinz")
    .await?;*/

    let posts = vec![Post {
        id: 0,
        author: "Hugo Boss".to_string(),
        title: "I like winning".to_string(),
        body: "I really like winning. That's why I always win at everything".to_string(),
    }];

    let data = web::Data::new(AppState {
        posts: Mutex::new(posts),
        hugo: Mutex::new(Person {
            name: "Hugo Boss".to_string(),
            age: 40,
        }),
    });

    println!("Started Server...");
    HttpServer::new(move || {
        App::new()
            //   .data(pool.clone())
            .app_data(data.clone())
            .service(hugo_post)
            .service(hugo)
            .service(get_post)
            .service(post_post)
            .service(get_posts)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;
    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
