extern crate actix;
extern crate actix_web;

use actix_web::{server, App, HttpRequest, HttpResponse};

fn get_books(_req: HttpRequest) -> HttpResponse {
    let table_content = "
        <thead>
        </thead>
        <tbody>
            <tr>
                <td>1</td>
                <td>2</td>
            </tr>
        </tbody>";

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
