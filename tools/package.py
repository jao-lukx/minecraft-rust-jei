#!/usr/bin/env python3
import hashlib, json, zipfile
from pathlib import Path
root=Path(__file__).resolve().parents[1]
meta=json.loads((root/'mod.json').read_text())
out=root/'target';out.mkdir(exist_ok=True)
archive=out/f"{meta['id']}-{meta['version']}-source.zip"
with zipfile.ZipFile(archive,'w',zipfile.ZIP_DEFLATED) as z:
 for path in sorted(root.rglob('*')):
  if not path.is_file() or any(p in {'.git','target','downloads','__pycache__'} for p in path.relative_to(root).parts):continue
  info=zipfile.ZipInfo(str(path.relative_to(root)),date_time=(2026,1,1,0,0,0));info.compress_type=zipfile.ZIP_DEFLATED
  info.external_attr=0o100644<<16;z.writestr(info,path.read_bytes())
print(archive)
print(hashlib.sha256(archive.read_bytes()).hexdigest())
