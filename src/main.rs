mod user;

use actix_web::{get, post, HttpResponse, HttpServer, Responder, web, App, cookie::Cookie};
use std::{fs, path::Path};
use actix_cors::Cors;

use user::user::User;
use serde::Deserialize;



#[get("/")]
async fn index() -> impl Responder{
    "hello"
}

#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder{
    HttpResponse::Ok()
        .content_type("text/html;carset=UTF-8")
        .body(format!("hello <b>{name}</b>!"))
}


#[derive(Deserialize, Debug)]
struct UserInfo{
    id: String,
    pw: String
}

#[post("/signup")]
async fn make_user(form: web::Form<UserInfo>) -> impl Responder{
    let id = form.id.clone();
    let pw = form.pw.clone();
    
    if id == ""{ return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(format!("<title>가입</title> <script>alert(`id가 비워저있어요!`);location.href = `signup??{}`</script>", pw.clone())) }
    if pw == ""{ return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(format!("<title>가입</title> <script>alert(`name이 비워저있어요!`);location.href = `signup?{}?`</script>", id.clone())) }

    let mut users = user::db::Db::new();
    let user = User{id, pw};
    if !users.new_user(user.clone()){
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(format!("<title>가입</title><script>alert(`이미 존재하는 id에요!`);location.href = `signup?{}?{}`</script>", user.id, user.pw))
    }else{
        println!("{:?}", user);
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(format!("<title>가입</title><script>alert(`가입성공!`);location.href = `login`</script>"))
    }
}



#[get("/signup")]
async fn signup() -> impl Responder{
    let mut a = fs::read_to_string(Path::new("front-end/signup.html")).unwrap();
    a.push_str(&format!("<style>{}</style>", fs::read_to_string(Path::new("front-end/styles/signup.css")).unwrap()));
    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
        .body(a)
}

#[get("/login")]
async fn login() -> impl Responder{
    let mut a = fs::read_to_string(Path::new("front-end/login.html")).unwrap();
    a.push_str(&format!("<style>{}</style>", fs::read_to_string(Path::new("front-end/styles/login.css")).unwrap()));
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(a)
}

#[post("/login")]
async fn login_checker(form: web::Form<UserInfo>) -> impl Responder{
    println!("request id: {:?}", form.id);
    println!("request pw: {:?}", form.pw);

    let mut res = String::new();

    Cookie::new("asdf", "asdddf");
    Cookie::build("asdf", "asdddf")
        .domain("127.0.0.1:8080")
        .secure(true)
        .http_only(true)
        .finish()
        ;
    res.push_str("<title>로그인</title>");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        let cors = Cors::default()
              .allowed_origin("http://127.0.0.1:8080");
            //   .max_age(3600);
        App::new()
            .wrap(cors)
            .service(index)
            .service(make_user)
            .service(hello)
            .service(signup)
            .service(login)
            .service(login_checker)
    })
    .bind(("127.0.0.1", 8080))?.run().await
}