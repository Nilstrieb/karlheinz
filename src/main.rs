use std::error::Error;
use std::sync::Mutex;

use actix_web::{App, Either, get, HttpResponse, HttpServer, post, Responder, web};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

macro_rules! impl_responder {
    (for $name: ident) => {
        impl actix_web::Responder for $name {
            type Error = actix_web::Error;
            type Future = std::future::Ready<Result<actix_web::HttpResponse, actix_web::Error>>;

            fn respond_to(self, _req: &actix_web::HttpRequest) -> Self::Future {
                let body = serde_json::to_string(&self).unwrap();

                std::future::ready(Ok(actix_web::HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body)))
            }
        }
    };
}
impl_responder!(for Person);
impl_responder!(for Post);

#[derive(Serialize, Deserialize, Clone)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Post {
    #[serde(default)]
    id: usize,
    author: String,
    title: String,
    content: String,
}

struct AppState {
    posts: Mutex<Vec<Post>>,
    hugo: Mutex<Person>,
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
    post.id = posts.len();
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
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:karl@localhost/karlheinz")
        .await?;

    let posts = vec![Post {
        id: 0,
        author: "Hugo Boss".to_string(),
        title: "I like winning".to_string(),
        content: "I really like winning. That's why I always win at everything".to_string(),
    }];

    let data = web::Data::new(AppState {
        posts: Mutex::new(posts),
        hugo: Mutex::new(Person {
            name: "Hugo Boss".to_string(),
            age: 40,
        }),
    });

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .app_data(data.clone())
            .service(hugo_post)
            .service(hugo)
            .service(get_post)
            .service(post_post)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
    Ok(())
}
