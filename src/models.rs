use super::schema::posts;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, Clone)]
pub struct Post {
    #[serde(default)]
    pub id: i32,
    pub author: String,
    pub title: String,
    pub body: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub author: &'a str,
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Serialize, Deserialize, Clone, Queryable)]
pub struct Person {
    #[serde(default)]
    pub id: String,
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

                std::future::ready(Ok(actix_web::HttpResponse::Ok().json(body)))
            }
        }
    };
}

impl_responder!(for Person);
impl_responder!(for Post);
