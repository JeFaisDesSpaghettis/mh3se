#!/bin/python3

from contextlib import asynccontextmanager
from pydantic import BaseModel
from fastapi import FastAPI
from fastapi.responses import RedirectResponse
from fastapi.staticfiles import StaticFiles
import uvicorn

import base64
import json
import os
import platform
import string
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
    Path("tmp").mkdir(parents=True, exist_ok=True)
    yield

app = FastAPI(lifespan=lifespan)

app.mount("/web", StaticFiles(directory="web"), name="web")

os_type = platform.system()

if os_type == "Windows":
    mh3se_exec = "./mh3se.exe"
else:
    mh3se_exec = "./mh3se"

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
        mh3se_exec,
        "decode",
        f"tmp/{binfname}",
        f"tmp/{jsonfname}",
        str(data.slot)
    ]

    res = subprocess.run(decode_cmd, check=True, capture_output=True, text=True)
    if res.returncode == 0:
        with open(f"tmp/{jsonfname}", "r") as f:
            os.remove(f"tmp/{binfname}")
            os.remove(f"tmp/{jsonfname}")
            return {"status": "OK", "payload": f.read()}

    os.remove(f"tmp/{binfname}")
    os.remove(f"tmp/{jsonfname}")
    return  {"status": "ERR", "payload": res.stdout + res.stderr}

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
        mh3se_exec,
        "encode",
        f"tmp/{jsonfname}",
        f"tmp/{inbinfname}",
        f"tmp/{outbinfname}",
        str(data.slot)
    ]

    res = subprocess.run(encode_cmd, check=True, capture_output=True, text=True)
    if res.returncode == 0:
        with open(f"tmp/{outbinfname}", "rb") as f:
            os.remove(f"tmp/{jsonfname}")
            os.remove(f"tmp/{inbinfname}")
            os.remove(f"tmp/{outbinfname}")
            return {"status": "OK", "payload": base64.b64encode(f.read())}

    os.remove(f"tmp/{jsonfname}")
    os.remove(f"tmp/{inbinfname}")
    os.remove(f"tmp/{outbinfname}")
    return  {"status": "ERR", "payload": res.stdout + res.stderr}

if __name__ == "__main__":
    print("mh3 save-editor")
    print("Project link: https://github.com/JeSuisSurGithub/mh3se")
    print("This program is licensed under GPLv3 terms.")
    uvicorn.run(app, port=8000, host="127.0.0.1")