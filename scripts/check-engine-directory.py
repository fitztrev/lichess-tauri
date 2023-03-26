from io import BytesIO
import json
import os
from urllib.request import Request, urlopen
from zipfile import ZipFile

f = open(os.path.join(os.path.dirname(__file__),
         '../pages/engine-directory.json'))
data = json.load(f)

for engine in data['engines']:
    for binary in engine['binaries']:
        print(f"Checking {binary['zip']}")

        resp = urlopen(Request(binary['zip'], headers={
                       'User-Agent': 'lichess-tauri ci'}))
        zip = ZipFile(BytesIO(resp.read()))

        assert binary['binary_filename'] in zip.namelist(
        ), f"Binary {binary['binary_filename']} not found in {binary['zip']}"

        print(f"\033[92m ✓ Found {binary['binary_filename']} \033[0m")
