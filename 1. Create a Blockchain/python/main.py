# module 1 - create a blockchain

import datetime
import hashlib
import json
from flask import Flask, jsonify

# building a blockchain
class Blockchain:
    def __init__(self):
        self.chain = []
        # genesis block create
        self.create_block(proof=1, previous_hash='0')

    def create_block(self, proof, previous_hash):
        block = {
            'index': len(self.chain) + 1,
            'timestamp': str(datetime.datetime.now()),
            'proof': proof,
            'previous_hash': previous_hash,
        }

        self.chain.append(block)
        return block
    
    def get_previous_block(self):
        return self.chain[-1]
    
    def proof_of_work(self, privious_proof):
        new_proof=1
        check_proof=False
        while check_proof is False:
            hash_operation=hashlib.sha256(str(int(new_proof)**2 - int(privious_proof)**2).encode()).hexdigest()
            if hash_operation[:4] == '0000': # first check
                check_proof=True
            else:
                new_proof+=1
            return new_proof
    
    def hash(self, block):
        encoded_block=json.dumps(block, sort_keys=True).encode()
        return hashlib.sha256(encoded_block).hexdigest()

    def is_chain_valid(self, chain):
        previous_block=chain[0]
        block_index=1
        while block_index < len(chain):
            block=chain[block_index]
            if block['previous_hash'] != self.hash(previous_block):
                return False
            privious_proof=previous_block['proof']
            proof=block['proof']
            hash_operation=hashlib.sha256(str(int(proof)**2 - int(privious_proof)**2).encode()).hexdigest()
            if hash_operation[:4] != '0000':
                return False
            previous_block=block
            block_index+=1
        return True


# mining our blockchain

app = Flask(__name__)

# create class Blockchain
blockchain=Blockchain()

@app.route('/mine_block', methods=['GET'])
def mine_block():
    previous_block=blockchain.get_previous_block()
    previous_proof=previous_block['proof']
    proof=blockchain.proof_of_work(previous_proof)
    previous_hash=blockchain.hash(previous_block)
    block=blockchain.create_block(proof, previous_hash)
    response={
        'message': 'Congrats! mine success',
        'index': block['index'],
        'timestamp': block['timestamp'],
        'proof': block['proof'],
        'previous_hash': block['previous_hash']
    }
    return jsonify(response), 200

# get the full blockchain
@app.route('/get_chain', methods=['GET'])
def get_chain():
    response={
        'chain': blockchain.chain,
        'length': len(blockchain.chain)
    }

    return jsonify(response), 200


@app.route('/is_valid', methods=['GET'])
def is_valid():
    response={
        'is_valid': blockchain.is_chain_valid(blockchain.chain)
    }
    return jsonify(response), 200