#!/usr/bin/env python3.10

import http.client
import subprocess
import sys

# args: id, fen, moves

def start_stockfish():
    stockfish = subprocess.Popen("/home/trevor/Downloads/stockfish_15_linux_x64/stockfish_15_x64", shell=True, stdin=subprocess.PIPE, stdout=subprocess.PIPE, bufsize=1, universal_newlines=True)
    stockfish.stdin.write("position fen " + sys.argv[2] + " moves " + sys.argv[3] + "\n")
    stockfish.stdin.write("go depth 20\n")
    while True:
        line = stockfish.stdout.readline()
        result = line.strip()
        with open("/home/trevor/code/lichess-tauri/stockfish.log", "a") as f:
            f.write(str(result))
            f.write("\n")
        if "bestmove" in line:
            break
        if "info depth 20" in line:
            answerLichess(result)

def answerLichess(line):
    conn = http.client.HTTPSConnection("9666-lichessorg-lilagitpod-ewnvbqhjl6a.ws-us73.gitpod.io")

    conn.request("POST", "/api/external-engine/work/" + sys.argv[1], line, {
        "Authorization": "Bearer lip_vSMsPHtKznbEHZS8rg7o",
        "Content-Type": "text/plain" 
    })
    response = conn.getresponse()
    result = response.read()

    with open("/home/trevor/code/lichess-tauri/stockfish.log", "a") as f:
        f.write(str(result))
        f.write("\n")

# with open("/home/trevor/code/lichess-tauri/stockfish.log", "a") as f:
#     f.write(str(sys.argv))
#     f.write("\n")

start_stockfish()
