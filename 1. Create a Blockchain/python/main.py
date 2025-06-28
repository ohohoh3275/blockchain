from flask import Flask, jsonify, render_template, request, redirect, url_for
import datetime
import hashlib
import json
from blockchain import Blockchain

app = Flask(__name__)

# 블록체인 인스턴스 생성
blockchain = Blockchain()

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/mine_block', methods=['GET'])
def mine_block():
    previous_block = blockchain.get_previous_block()
    previous_proof = previous_block['proof']
    proof = blockchain.proof_of_work(previous_proof)
    previous_hash = blockchain.hash(previous_block)
    block = blockchain.create_block(proof, previous_hash)
    response = {
        'message': '축하합니다! 블록 채굴 성공!',
        'index': block['index'],
        'timestamp': block['timestamp'],
        'proof': block['proof'],
        'previous_hash': block['previous_hash']
    }
    return jsonify(response), 200

@app.route('/get_chain', methods=['GET'])
def get_chain():
    response = {
        'chain': blockchain.chain,
        'length': len(blockchain.chain)
    }
    return jsonify(response), 200

@app.route('/is_valid', methods=['GET'])
def is_valid():
    response = {
        'is_valid': blockchain.is_chain_valid(blockchain.chain)
    }
    return jsonify(response), 200

def main():
    print("🚀 블록체인 시연 애플리케이션을 시작합니다...")
    print("📱 웹 브라우저에서 http://localhost:5000 으로 접속하세요")
    print("⏹️  종료하려면 Ctrl+C를 누르세요")
    print("-" * 50)
    app.run(debug=True, host='0.0.0.0', port=5000)

if __name__ == '__main__':
    main() 