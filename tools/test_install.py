#!/usr/bin/env python3
import json,subprocess,sys,tempfile
from pathlib import Path
root=Path(__file__).resolve().parents[1];original=Path(sys.argv[1])
patches=json.loads((root/'patches.json').read_text())
paths={v['path'] for v in patches}|{'crates/horizon-client/src/game.rs'}
with tempfile.TemporaryDirectory() as d:
 dest=Path(d)
 for path in paths:
  target=dest/path;target.parent.mkdir(parents=True,exist_ok=True);target.write_bytes((original/path).read_bytes())
 before={p:(dest/p).read_bytes() for p in paths}
 def run(*args):subprocess.run([sys.executable,str(root/'tools/install.py'),str(dest),*args],check=True)
 run();installed={p:(dest/p).read_bytes() for p in paths};run();assert installed=={p:(dest/p).read_bytes() for p in paths}
 run('--remove');assert before=={p:(dest/p).read_bytes() for p in paths}
 run('--remove')
 print('PASS: installation, idempotence, byte-for-byte removal')
