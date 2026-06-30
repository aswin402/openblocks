#!/usr/bin/env python3
"""
Extract CSS checkboxes from getcssscan.com external index.js to OpenBlocks seed format.
"""
import json
import re
import subprocess
import sys

CHECKBOXES_JS = '/tmp/checkboxes_data.js'
OUTPUT = 'data/css_checkboxes.json'


def extract_checkboxes_via_node():
    """Use Node.js to properly parse the checkboxes array."""
    js_code = """
const fs = require('fs');
const js = fs.readFileSync('/tmp/checkboxes_data.js', 'utf8');
const match = js.match(/const checkboxes = (\\[[\\s\\S]*?\\])\\s*\\n\\s*async function/);
if (!match) { console.log('[]'); process.exit(1); }
const checkboxes = eval('(' + match[1] + ')');
// Transform to simpler form without template literals
const simplified = checkboxes.map(c => ({
    credits: c.credits || '',
    html: c.html || '',
    css: c.css || '',
    forDarkMode: c.forDarkMode || false
}));
console.log(JSON.stringify(simplified));
"""
    result = subprocess.run(['node', '-e', js_code], capture_output=True, text=True, timeout=30)
    if result.returncode != 0:
        print(f"Node error: {result.stderr}", file=sys.stderr)
        sys.exit(1)
    return json.loads(result.stdout)


def make_component(item, index):
    credits = item['credits']
    css = item['css']
    html = item['html']
    for_dark = item.get('forDarkMode', False)

    if not credits or not css:
        return None

    css_fixed = css.replace('{index}', str(index))
    html_fixed = html.replace('{index}', str(index)) if html else f'<input type="checkbox" id="cb-{index}">'

    brand = re.sub(r'[^a-z0-9]', '-', credits.lower()).strip('-')
    name = f"CSS Checkbox {index}: {credits}"
    full_html = f'{html_fixed}\n<style>\n{css_fixed}\n</style>'

    desc = f"A CSS checkbox styled component inspired by {credits}'s design."
    if for_dark:
        desc += " Supports dark mode."

    tags = ["checkbox", "css", "css-checkbox", brand]
    if for_dark:
        tags.append("dark-mode")

    return {
        "name": name,
        "description": desc,
        "category": "other",
        "framework": "css",
        "code": full_html,
        "tags": tags,
        "dependencies": []
    }


def main():
    items = extract_checkboxes_via_node()
    print(f"Found {len(items)} checkboxes")

    components = []
    for i, item in enumerate(items, 1):
        comp = make_component(item, i)
        if comp:
            components.append(comp)

    with open(OUTPUT, 'w') as f:
        json.dump(components, f, indent=2)

    print(f"Created {OUTPUT} with {len(components)} components")


if __name__ == '__main__':
    main()
