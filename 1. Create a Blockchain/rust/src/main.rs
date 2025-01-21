use std::time::SystemTime;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Clone)]
struct Block {
    index: usize,
    timestamp: SystemTime,
    proof: String,
    previous_hash: String
}

struct Blockchain {
    chain: Vec<Block>
}

trait BlockchainTrait {
    fn create_block(&mut self, proof: String, previous_hash: String) -> Option<&Block>;
}

impl BlockchainTrait for Blockchain {
    fn create_block(&mut self, proof: String, previous_hash: String) -> Option<&Block> {
        let new_index: usize = self.chain.len()+1;
        let block= Block {
            index: new_index,
            timestamp: SystemTime::now(),
            proof: String::from(proof),
            previous_hash: String::from(previous_hash)
        };

        self.chain.push(block);
        let last_chain= Some(self.chain.last());
        return last_chain.unwrap_or_default();
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