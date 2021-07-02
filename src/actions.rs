use crate::models::{NewPost, Person, Post};

use crate::schema::*;
use diesel::prelude::*;

pub fn insert_post(con: &PgConnection, new_post: &Post) -> QueryResult<Post> {
    let new_post = NewPost {
        author: &new_post.author,
        title: &new_post.title,
        body: &new_post.body,
    };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result::<Post>(con)
}

pub fn update_person(con: &PgConnection, update_person: &Person) -> QueryResult<Person> {
    use crate::schema::person::dsl::*;
    diesel::update(person.find("hugo".to_string()))
        .set((name.eq(&update_person.name), age.eq(update_person.age)))
        .get_result(con)
}

pub fn find_person_by_id(con: &PgConnection, person_id: &str) -> QueryResult<Option<Person>> {
    use crate::schema::person::dsl::*;
    let user: Option<Person> = person
        .filter(id.eq(person_id))
        .first::<Person>(con)
        .optional()?;
    Ok(user)
}
