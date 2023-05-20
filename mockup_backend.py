from flask import Flask, jsonify, request
import random
from flask_cors import CORS
import time

app = Flask(__name__)
CORS(app)

@app.route('/analyze', methods=['POST'])
def analyze():
    time.sleep(200)
    data = {}
    for file in request.files.getlist('f[]'):
        if file:
            data[file.filename] = {'overall_class': {'шипун':random.random(), 'кликун': random.random(), 'малый': random.random()}}
    return jsonify(data)

@app.route('/')
def index():
    return '''
    <form method=POST action="/analyze" enctype="multipart/form-data">
    <input type="file" name="f[]">
    <input type="file" name="f[]">
    <input type="file" name="f[]">
    <input type="file" name="f[]">
    <input type=submit>
    </form>
    '''

if __name__ == '__main__':
    app.run('0.0.0.0', 5000)
