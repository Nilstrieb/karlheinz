use std::sync::Mutex;

use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
}

struct AppState {
    hugos_name: Mutex<String>,
    request_amount: Mutex<i32>,
}

#[get("/hugo")]
async fn hugo(data: web::Data<AppState>) -> impl Responder {
    let hugo_name = &data.hugos_name.lock().unwrap();
    let mut counter = data.request_amount.lock().unwrap();
    *counter += 1;

    HttpResponse::Ok().body(format!("{} as been requested {} times", hugo_name, counter))
}

#[post("/hugo")]
async fn hugo_post(req_body: String, data: web::Data<AppState>) -> impl Responder {
    let mut hugo_name = data.hugos_name.lock().unwrap();
    *hugo_name = req_body;

    HttpResponse::Ok().body(&*hugo_name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        request_amount: Mutex::new(0),
        hugos_name: Mutex::new("Hugo Boss".to_string()),
    });

    HttpServer::new(move ||
        App::new()
            .app_data(data.clone())
            .service(hugo_post)
            .service(hugo)
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
