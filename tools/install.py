#!/usr/bin/env python3
"""Development tool: apply/remove source mod; never launches or downloads a game."""
import argparse, json, os, tempfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
p=argparse.ArgumentParser(description=__doc__)
p.add_argument('source',type=Path,help='Extracted horizon-offline-profiles source root')
p.add_argument('--remove',action='store_true')
a=p.parse_args(); source=a.source.resolve()
meta=json.loads((ROOT/'mod.json').read_text()); mid=meta['id']
patches=json.loads((ROOT/'patches.json').read_text())
game='crates/horizon-client/src/game.rs'
marker=f'// source-mod: {mid}\nmod {mid}_core;\nmod {mid}_runtime;\n'
payload={source/f'crates/horizon-client/src/game/{mid}_{n}.rs':(ROOT/f'mixins/{mid}/{f}.rs').read_text() for n,f in [('core','core'),('runtime','runtime')]}
files={}
for patch in patches:
 path=source/patch['path']
 files.setdefault(path,path.read_text())
main=source/game; files.setdefault(main,main.read_text())
for path,text in payload.items():
 if path.exists() and path.read_text()!=text: raise SystemExit(f'Refusing to overwrite changed payload {path}')
installed=marker in files[main]
if installed==a.remove:
 for patch in patches:
  path=source/patch['path']; old,new=patch['old'],patch['new']
  if a.remove:
   if patch['path'].endswith('/native_hooks.rs'):
    old,new=new[len(old):],''
   else: old,new=new,old
  if files[path].count(old)!=1: raise SystemExit(f'Unsupported or modified source: anchor in {path}')
  files[path]=files[path].replace(old,new,1)
 if a.remove: files[main]=files[main].replace('\n'+marker,'',1)
 else: files[main]+= '\n'+marker
else:
 print('Already removed.' if a.remove else 'Already installed.'); raise SystemExit(0)
for path,text in payload.items():
 if path.exists() and path.read_text()!=text: raise SystemExit(f'Refusing to overwrite changed payload {path}')
# All compatibility/ownership checks precede writes. Replace each file atomically.
for path,text in (files if a.remove else {**files,**payload}).items():
 fd,tmp=tempfile.mkstemp(dir=path.parent,prefix='.source-mod-')
 os.chmod(tmp, (path.stat().st_mode & 0o777) if path.exists() else 0o644)
 with os.fdopen(fd,'w') as stream: stream.write(text)
 os.replace(tmp,path)
if a.remove:
 for path in payload: path.unlink(missing_ok=True)
print(('Removed ' if a.remove else 'Installed ')+mid+'. Recompile horizon-client.')
