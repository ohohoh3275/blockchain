use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// blockchain object


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/get_chain")]
async fn get_chain(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/mine_block")]
async fn mine_block(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/is_valid")]
async fn is_valid(req_body: String) -> impl Responder {
    HttpResponse::Ok().body("is_valid")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(get_chain)
            .service(mine_block)
            .service(is_valid)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}