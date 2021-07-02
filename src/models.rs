use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct Post {
    #[serde(default)]
    pub id: i32,
    pub author: String,
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Person {
    pub name: String,
    pub age: i32,
}

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
