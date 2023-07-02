from io import BytesIO
import json
import os
import tarfile
from urllib.request import Request, urlopen
from zipfile import ZipFile

f = open(os.path.join(os.path.dirname(__file__),
         '../pages/engine-directory.json'))
data = json.load(f)

for engine in data['engines']:
    for binary in engine['binaries']:
        print(f"Checking {binary['zip']}")

        resp = urlopen(Request(binary['zip'], headers={
                       'User-Agent': 'https://github.com/fitztrev/lichess-tauri/actions'}))

        if binary['zip'].endswith('.zip'):
            zip = ZipFile(BytesIO(resp.read()))
            assert binary['binary_filename'] in zip.namelist(), f"\033[91m Binary {binary['binary_filename']} not found in {binary['zip']} \033[0m"
        else:
            files = tarfile.open(fileobj=BytesIO(resp.read()))
            assert binary['binary_filename'] in files.getnames(), f"\033[91m Binary {binary['binary_filename']} not found in {binary['zip']} \033[0m"

        print(f"\033[92m ✓ Found {binary['binary_filename']} \033[0m")

# Make sure each engine offers a "default" architecture binary for each OS
for engine in data['engines']:
    os_with_defaults = []
    for binary in engine['binaries']:
        if binary['architecture'] == 'default':
            os_with_defaults.append(binary['os'])
    os_with_defaults.sort()
    assert os_with_defaults == ['linux', 'macos', 'windows'], f"\033[91m Engine {engine['name']} does not offer a default binary for each OS \033[0m"
    print(f"\033[92m ✓ default architecture option provided for each OS for {engine['name']} \033[0m")
