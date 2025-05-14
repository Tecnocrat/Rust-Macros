from flask import Flask, jsonify
import json

app = Flask(__name__)

@app.route("/index")
def get_index():
    with open("workspace_index.json", "r", encoding="utf-8") as f:
        return jsonify(json.load(f))

@app.route("/manifest")
def get_manifest():
    with open("codebot.json", "r", encoding="utf-8") as f:
        return jsonify(json.load(f))

if __name__ == "__main__":
    app.run(port=5000)