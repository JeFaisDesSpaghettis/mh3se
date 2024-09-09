#!/bin/python3

from contextlib import asynccontextmanager
from pydantic import BaseModel
from fastapi import FastAPI
from fastapi.responses import RedirectResponse
from fastapi.staticfiles import StaticFiles
import uvicorn

import base64
import json
import string
import os
import subprocess
import random
from pathlib import Path

class Save2Json(BaseModel):
    binfile: str
    slot: int

class Json2Save(BaseModel):
    binfile: str
    jsonfile: str
    slot: int

@asynccontextmanager
async def lifespan(app: FastAPI):
    yield

BASE_DIR = Path(__file__).parent

Path("tmp").mkdir(parents=True, exist_ok=True)

app = FastAPI(lifespan=lifespan)

app.mount("/web", StaticFiles(directory="web"), name="web")

def genid(size=8, chars=string.ascii_uppercase + string.digits):
    return ''.join(random.choice(chars) for _ in range(size))

@app.get("/")
def root():
    return RedirectResponse(url="/web/index.html")

@app.post("/save2json")
async def save2json(data: Save2Json):
    jsonfname = genid()
    binfname = genid()

    with open(f"tmp/{binfname}", "wb+") as f:
        f.write(base64.b64decode(data.binfile))

    decode_cmd = [
        "./mh3se",
        "decode",
        f"tmp/{binfname}",
        f"tmp/{jsonfname}",
        str(data.slot)
    ]

    subprocess.run(decode_cmd)

    with open(f"tmp/{jsonfname}", "r") as f:
        return {"status": "OK", "payload": f.read()}

    return  {"status": "ERR", "payload": None}

@app.post("/json2save")
async def json2save(data: Json2Save):
    jsonfname = genid()
    inbinfname = genid()
    outbinfname = genid()

    with open(f"tmp/{jsonfname}", "w+") as f:
        f.write(data.jsonfile)

    with open(f"tmp/{inbinfname}", "wb+") as f:
        f.write(base64.b64decode(data.binfile))

    open(f"tmp/{outbinfname}", "xb")

    encode_cmd = [
        "./mh3se",
        "encode",
        f"tmp/{jsonfname}",
        f"tmp/{inbinfname}",
        f"tmp/{outbinfname}",
        str(data.slot)
    ]

    subprocess.run(encode_cmd)

    with open(f"tmp/{outbinfname}", "rb") as f:
        return {"status": "OK", "payload": base64.b64encode(f.read())}

if __name__ == "__main__":
    uvicorn.run(app, port=8000, host="127.0.0.1")