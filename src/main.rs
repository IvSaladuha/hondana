#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate actix;
extern crate actix_web;

use actix_web::{server, App, HttpRequest, HttpResponse};
use diesel::prelude::*;

use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

mod models;
use models::Book;
mod schema;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn get_books(_req: HttpRequest) -> HttpResponse {
    use self::schema::books::dsl::*;

    let connection = establish_connection();

    let results = books.limit(5).load::<Book>(&connection)
        .expect("Error loading books");
    let rows: Vec<String> = results.iter().map(|b| format!("<tr><td>{}</td></tr>", b.title)).collect();

    let table_content = format!("
        <thead>
        </thead>
        <tbody>
            <tr>
                {}
            </tr>
        </tbody>", rows.join(" "));

    let body = format!("
    <!DOCTYPE html>
    <html>
        <head>
            <meta http-equiv=\"Content-Type\" content=\"text/html\" charset=\"utf-8\"/>
        </head>
        <body>
            <table>{}</table>
        </body>
    </html>", table_content);
    HttpResponse::Ok().content_type("text/html").body(body)
}

fn main() {
    let sys = actix::System::new("guide");

    server::HttpServer::new(
        || App::new()
            .resource("/", |r| r.f(get_books)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .start();

    println!("started http server: 127.0.0.1:8088");
    let _ = sys.run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        assert!(true);
    }
}
