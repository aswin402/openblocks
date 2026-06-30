#!/usr/bin/env python3
"""
Extract CSS box shadows from getcssscan.com saved HTML to OpenBlocks seed format.
"""
import json
import re
import subprocess
import sys
import tempfile

SHADOW_HTML = '/tmp/css_boxes.html'
OUTPUT = 'data/css_shadows.json'


def extract_shadows_via_node():
    """Use Node.js to properly parse the JS array."""
    js_code = """
const fs = require('fs');
const html = fs.readFileSync('/tmp/css_boxes.html', 'utf8');
const idx = html.indexOf('const shadows =');
const start = html.indexOf('[', idx);
let depth = 1, i = start + 1;
while (i < html.length && depth > 0) {
    if (html[i] === '[') depth++;
    else if (html[i] === ']') depth--;
    i++;
}
const clean = html.substring(start, i).replace(/\\/\\/.*$/gm, '');
const shadows = eval('(' + clean + ')');
console.log(JSON.stringify(shadows));
"""
    result = subprocess.run(['node', '-e', js_code], capture_output=True, text=True, timeout=30)
    if result.returncode != 0:
        print(f"Node error: {result.stderr}", file=sys.stderr)
        sys.exit(1)
    return json.loads(result.stdout)


def make_component(shadow_val, index):
    if isinstance(shadow_val, str):
        box_shadow = shadow_val
        credits = 'CSS Scan'
    else:
        box_shadow = shadow_val.get('boxShadow', '')
        credits = shadow_val.get('credits', 'Unknown')

    if not box_shadow:
        return None

    css = f"/* Shadow {index} */\n.shadow-{index} {{\n  box-shadow: {box_shadow};\n}}"
    html = f'<div class="shadow-{index}">Shadow {index}</div>'

    brand = re.sub(r'[^a-z0-9]', '-', credits.lower()).strip('-')
    name = f"CSS Box Shadow {index}: {credits}"
    
    code = f"{html}\n<style>\n{css}\n</style>"

    return {
        "name": name,
        "description": f"A CSS box shadow example by {credits} with box-shadow: {box_shadow[:80]}...",
        "category": "other",
        "framework": "css",
        "code": code,
        "tags": ["box-shadow", "css", "css-shadow", brand, "shadow"],
        "dependencies": []
    }


def main():
    shadows = extract_shadows_via_node()
    print(f"Found {len(shadows)} shadows")

    components = []
    for i, shadow in enumerate(shadows, 1):
        comp = make_component(shadow, i)
        if comp:
            components.append(comp)

    with open(OUTPUT, 'w') as f:
        json.dump(components, f, indent=2)

    print(f"Created {OUTPUT} with {len(components)} components")


if __name__ == '__main__':
    main()
