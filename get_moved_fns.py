import re
import glob

def get_moved():
    fns = []
    for file in glob.glob('src/session/helper/*.rs'):
        with open(file, 'r') as f:
            for line in f:
                if line.startswith('pub fn ') or line.startswith('pub async fn '):
                    # extract the name
                    m = re.match(r'pub (?:async )?fn ([a-zA-Z0-9_]+)', line)
                    if m:
                        fns.append(m.group(1))
    return fns

print(" , ".join(f'"{f}"' for f in get_moved()))
