import json, uuid
from flask import Flask
from models.user import User

app = Flask(__name__)

@app.route("/api/user")
def get_user():
    user = User(str(uuid.uuid4()), "Benjamin", "HEINTZ", "heintz.benjamin@gmail.com")
    data = json.dumps(user.__dict__)
    return data

if __name__ == '__main__':
      app.run(host='0.0.0.0', port=5300)
