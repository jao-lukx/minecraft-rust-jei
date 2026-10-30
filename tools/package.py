#!/usr/bin/env python3
"""Developer tool: build an installable native .hnmod, not a source installer ZIP."""
from pathlib import Path
import json,zipfile,hashlib,tomllib
root=Path(__file__).resolve().parents[1]
meta=json.loads((root/'mod.json').read_text());mid=meta['id'];folder=root/'native'/mid
native=json.loads((folder/'horizon-native.json').read_text());manifest=tomllib.loads((folder/'horizon-mixin.toml').read_text())
assert native['id']==manifest['mod']['id']==mid
assert native['version']==manifest['mod']['version']==meta['version']
assert native['horizon']==manifest['compatibility']['horizon_revision']
files={'horizon-native.json':(folder/'horizon-native.json').read_bytes()}
for f in folder.iterdir():
 if f.name=='horizon-native.json':continue
 assert f.is_file() and f.suffix in {'.rs','.toml'}
 files[f'mixins/{mid}/{f.name}']=f.read_bytes()
out=root/'target';out.mkdir(exist_ok=True);archive=out/f"{mid}-{meta['version']}.hnmod"
with zipfile.ZipFile(archive,'w',zipfile.ZIP_DEFLATED) as z:
 for name,data in sorted(files.items()):
  info=zipfile.ZipInfo(name,date_time=(2026,10,30,0,0,0));info.compress_type=zipfile.ZIP_DEFLATED;info.external_attr=0o100644<<16;z.writestr(info,data)
print(archive);print(hashlib.sha256(archive.read_bytes()).hexdigest())
