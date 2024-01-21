import uuid
from fastapi import FastAPI
from models.user import User

app = FastAPI()

@app.get("/")
def index():
    return {"name": "Benchmark.Python"}

@app.get("/api/user")
def get_user():
    user = User(str(uuid.uuid4()), "Benjamin", "HEINTZ", "heintz.benjamin@gmail.com")
    return user
