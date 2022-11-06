#!/usr/bin/env python3

import argparse
import requests
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument("--host")
parser.add_argument("--token")
parser.add_argument("--work-id")
parser.add_argument("--binary-path")
parser.add_argument("--fen")
parser.add_argument("--moves")

args = parser.parse_args()


def start_stockfish():
    stockfish = subprocess.Popen(args.binary_path, shell=True, stdin=subprocess.PIPE,
                                 stdout=subprocess.PIPE, bufsize=1, universal_newlines=True)
    stockfish.stdin.write(
        "position fen " + args.fen + " moves " + args.moves + "\n")
    stockfish.stdin.write("go depth 20\n")
    while True:
        line = stockfish.stdout.readline()
        result = line.strip()

        if "info depth 20" in line:
            answerLichess(result)
            break


def answerLichess(line):
    reqUrl = args.host + "/api/external-engine/work/" + args.work_id
    requests.request("POST", reqUrl, data=line,  headers={
        "Authorization": "Bearer " + args.token,
        "Content-Type": "text/plain" 
    })

start_stockfish()
