use std::time::SystemTime;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};


struct Chain {
    index: i32,
    timestamp: SystemTime,
    proof: String,
    previous_hash: String
}
trait BlockchainTrait {
    fn create_block(&mut self, proof: String, previous_hash: String) -> Chain;
}

impl BlockchainTrait for Chain{
    fn create_block(&mut self, proof: String, previous_hash: String) -> Chain {
        let  new_index = self.index+1;
        let chain= Chain {
            index: new_index,
            timestamp: SystemTime::now(),
            proof: String::from(proof),
            previous_hash: String::from(previous_hash)
        };

        // ...
        chain
    }
}

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