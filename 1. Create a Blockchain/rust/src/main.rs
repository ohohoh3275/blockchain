use std::time::SystemTime;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use sha2::{Sha256, Digest};

use std::ptr::hash;

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
    fn proof_of_work(previous_proof: i32) -> i32;
    fn hash(&mut self, block: &Block)-> String;
    fn is_chain_valid(chain: &Vec<Block>) -> bool;
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


    fn proof_of_work(previous_proof: i32) -> i32 {
        let mut new_proof = 1;
        let mut check_proof = false;

        while !check_proof {
            let mut hasher = Sha256::new(); // TODO: common hasher
            let input = format!("{}", new_proof * 2 - previous_proof * 2);
            hasher.update(input.as_bytes());

            let hash_operation = hasher.finalize();
            let hash_hex = format!("{:x}", hash_operation); 

            if hash_hex.starts_with("0000") {
                check_proof = true;
            } else {
                new_proof += 1;
            }
        }

        return new_proof
    }
    
    fn hash(&mut self, block: &Block)-> String {
        todo!()
    }
    
    fn is_chain_valid(chain: &Vec<Block>) -> bool {
        if chain.is_empty() {
            return false;
        }
    
        let mut previous_block = &chain[0];
        let mut block_index = 1;
        
        while block_index < chain.len() {
            let block = &chain[block_index];
            let mut hasher = Sha256::new();
            if block.previous_hash != hasher(&mut previous_block).unwrap_or_default() {
                return false;
            }
    
            let previous_proof = previous_block.proof;
            let previous_proof_num = previous_proof.parse::<i32>().un;
            let proof = block.proof;
            let proof_num = proof.parse::<i32>().unwrap();
            let proof_difference = proof_num.abs().pow(2) as i64 - previous_proof_num.abs().pow(2) as i64;

            let mut hasher = Sha256::new();
            hasher.update(proof_difference.to_string().as_bytes());
            let hash_operation = format!("{:x}", hasher.finalize());
            
            if &hash_operation[..4] != "0000" {
                return false;
            }
    
            previous_block = block;
            block_index += 1;
        }
        true
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
    let response = {
        'chain': 
    }
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