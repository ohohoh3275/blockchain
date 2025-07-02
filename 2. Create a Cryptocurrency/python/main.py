from flask import Flask, render_template, request, jsonify
from flask_cors import CORS
import requests
import json
import threading
import time

app = Flask(__name__)

CORS(app)

# 기본 노드 설정
DEFAULT_NODE = "http://127.0.0.1:5000"
NODES = ["http://127.0.0.1:5001", "http://127.0.0.1:5002", "http://127.0.0.1:5003"]

@app.route('/')
def index():
    return render_template('index.html')

@app.route('/api/mine_block', methods=['POST'])
def mine_block():
    try:
        data = request.get_json()
        node_url = data.get('node_url', DEFAULT_NODE)
        
        response = requests.get(f"{node_url}/mine_block")
        if response.status_code == 200:
            return jsonify(response.json()), 200
        else:
            return jsonify({"error": "블록 마이닝 실패"}), 400
    except Exception as e:
        return jsonify({"error": f"서버 오류: {str(e)}"}), 500

@app.route('/api/get_chain', methods=['POST'])
def get_chain():
    try:
        data = request.get_json()
        node_url = data.get('node_url', DEFAULT_NODE)
        
        response = requests.get(f"{node_url}/get_chain")
        if response.status_code == 200:
            return jsonify(response.json()), 200
        else:
            return jsonify({"error": "체인 조회 실패"}), 400
    except Exception as e:
        return jsonify({"error": f"서버 오류: {str(e)}"}), 500

@app.route('/api/is_valid', methods=['POST'])
def is_valid():
    try:
        data = request.get_json()
        node_url = data.get('node_url', DEFAULT_NODE)
        
        response = requests.get(f"{node_url}/is_valid")
        if response.status_code == 200:
            return jsonify(response.json()), 200
        else:
            return jsonify({"error": "유효성 검사 실패"}), 400
    except Exception as e:
        return jsonify({"error": f"서버 오류: {str(e)}"}), 500

@app.route('/api/add_transaction', methods=['POST'])
def add_transaction():
    try:
        data = request.get_json()
        node_url = data.get('node_url', DEFAULT_NODE)
        transaction_data = {
            'sender': data.get('sender', ''),
            'receiver': data.get('receiver', ''),
            'amount': float(data.get('amount', 0))
        }
        
        response = requests.post(f"{node_url}/add_transaction", json=transaction_data)
        if response.status_code == 201:
            return jsonify(response.json()), 201
        else:
            return jsonify({"error": "트랜잭션 추가 실패"}), 400
    except Exception as e:
        return jsonify({"error": f"서버 오류: {str(e)}"}), 500

@app.route('/api/connect_node', methods=['POST'])
def connect_node():
    try:
        data = request.get_json()
        node_url = data.get('node_url', DEFAULT_NODE)
        nodes_to_connect = data.get('nodes', NODES)
        
        response = requests.post(f"{node_url}/connect_node", json={'nodes': nodes_to_connect})
        if response.status_code == 201:
            return jsonify(response.json()), 201
        else:
            return jsonify({"error": "노드 연결 실패"}), 400
    except Exception as e:
        return jsonify({"error": f"서버 오류: {str(e)}"}), 500

@app.route('/api/replace_chain', methods=['POST'])
def replace_chain():
    try:
        data = request.get_json()
        node_url = data.get('node_url', DEFAULT_NODE)
        
        response = requests.get(f"{node_url}/replace_chain")
        if response.status_code == 200:
            return jsonify(response.json()), 200
        else:
            return jsonify({"error": "체인 교체 실패"}), 400
    except Exception as e:
        return jsonify({"error": f"서버 오류: {str(e)}"}), 500

import concurrent.futures
import subprocess
import os

@app.route('/api/start_nodes', methods=['POST'])
def start_nodes():
    try:
        def run_node_file(port):
            env = os.environ.copy()
            env.pop('WERKZEUG_SERVER_FD', None)
            env.pop('WERKZEUG_RUN_MAIN', None)
            
            process = subprocess.Popen(['python3', f'node_{port}.py'], 
                                     cwd='./',
                                     env=env)
            return {'port': port, 'pid': process.pid}
        
        ports = ['5001', '5002', '5003']
        results = []
        
        # pickle issue fixed
        with concurrent.futures.ThreadPoolExecutor(max_workers=3) as executor:
            futures = [executor.submit(run_node_file, port) for port in ports]
            for future in concurrent.futures.as_completed(futures):
                result = future.result()
                results.append(result)
        
        return jsonify({"message": "노드들이 시작되었습니다", "results": results}), 200
    except Exception as e:
        return jsonify({"error": str(e)}), 500


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=8080, debug=True) 