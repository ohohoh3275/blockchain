# module 1 - create a blockchain

from flask import Flask

app=Flask(__name__)

@app.route('/')
def hi():
    return "hello world"