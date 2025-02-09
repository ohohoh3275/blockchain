use std::time::SystemTime;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sha2::{Sha256, Digest};

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
    fn get_previous_block(&mut self) -> Option<&Block>;
    fn proof_of_work(&mut self, previous_proof: String) -> i32;
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

    fn get_previous_block(&mut self) -> Option<&Block> {
        return self.chain.last(); // last() 는 Option<T> 
    }

    fn proof_of_work(&mut self, previous_proof: String) -> i32 {
        let mut new_proof=1;
        let mut check_proof:bool=false;
        while check_proof == false {
            let mut hasher = Sha256::new();
            let s: String = previous_proof.to_string();
            let previous_proof_number: i32 = s.parse().expect("Not a valid number");
            hasher.update(new_proof*2 - previous_proof_number*2); // !
            let hash_operation = hasher.finalize();

            if hash_operation[-4] == 0000 {
                check_proof = true
            } else {
                new_proof += 1
            }
        }

        return new_proof
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