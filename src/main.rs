use std::sync::Mutex;
use actix_cors::Cors;

use actix_web::{web::Data, get, HttpResponse, HttpServer, Responder, web, App, http};

struct Counter{
    i: Mutex<isize>
}

#[get("/")]
async fn index() -> impl Responder{
    "LOL"
}
#[get("/add_count/{i}")]
async fn set_count(data: Data<Counter>, p: web::Path<isize>) -> impl Responder{
    let mut d = data.i.lock().unwrap();
    let i = *p;
    print!("{d} -> ");
    *d += i;
    print!("{d}\n");
    format!("true")
}
#[get("/get_count")]
async fn get_count(data: Data<Counter>) -> impl Responder{
    println!("/get_count");
    let d = data.i.lock().unwrap();
    HttpResponse::Ok()
        // .content_type("text/html")
        .body(format!("{}\"count\": {}{}", "{", d, "}"))
        // .body(d.to_string())
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder{
    HttpResponse::Ok()
        .content_type("text/html;carset=UTF-8")
        .body(format!("hello <b>{name}</b>!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        let cors = Cors::default()
              .allowed_origin("http://127.0.0.1:5500");
            //   .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(Data::new(Counter{i: Mutex::new(0)}))
            .service(index)
            .service(set_count)
            .service(get_count)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?.run().await
}