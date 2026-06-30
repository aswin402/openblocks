#!/usr/bin/env python3
"""
Extract CSS shapes from getcssscan.com saved HTML and convert to OpenBlocks seed format.

Parses the `const shapes = [...]` JavaScript array from the saved HTML file.
Each shape has `{name, css}`.
"""
import json
import re
import sys

SHAPES_HTML = '/tmp/css_shapes.html'
OUTPUT = 'data/css_shapes.json'

def extract_shapes():
    with open(SHAPES_HTML) as f:
        html = f.read()

    m = re.search(r'const shapes = (\[.*?\]);', html, re.DOTALL)
    if not m:
        print("ERROR: Could not find shapes array", file=sys.stderr)
        sys.exit(1)

    array_text = m.group(1)

    # Parse each {name, css} object
    shapes = []
    depth = 0
    start = None
    in_template = False
    for i, ch in enumerate(array_text):
        if ch == '`':
            in_template = not in_template
        if in_template:
            continue
        if ch == '{':
            if depth == 0:
                start = i
            depth += 1
        elif ch == '}':
            depth -= 1
            if depth == 0 and start is not None:
                shapes.append(array_text[start:i+1])
                start = None

    return shapes


def extract_field(obj_text, field):
    """Extract a template-literal or string field value."""
    m = re.search(rf'{re.escape(field)}:\s*`', obj_text)
    if m:
        start = m.end()
        depth = 0
        i = start
        while i < len(obj_text):
            if obj_text[i] == '`' and (i == 0 or obj_text[i-1] != '\\') and depth == 0:
                return obj_text[start:i]
            elif obj_text[i] == '$' and i + 1 < len(obj_text) and obj_text[i+1] == '{':
                depth += 1
            elif obj_text[i] == '}' and depth > 0:
                depth -= 1
            i += 1
    m = re.search(rf"{re.escape(field)}:\s*'([^']*)'", obj_text)
    if m:
        return m.group(1)
    return None


def clean_indent(text):
    """Remove common leading whitespace from all lines."""
    lines = text.split('\n')
    indent = None
    for line in lines:
        stripped = line.lstrip()
        if stripped:
            leading = len(line) - len(stripped)
            if indent is None or leading < indent:
                indent = leading
    if indent is None or indent == 0:
        return text
    return '\n'.join(line[indent:] if len(line) >= indent else line for line in lines)


def make_component(shape_obj, index):
    name = extract_field(shape_obj, 'name')
    css = extract_field(shape_obj, 'css')

    if not name or not css:
        return None

    css_fixed = css.replace('{index}', str(index))
    css_fixed = clean_indent(css_fixed)

    html = f"""<div class="shape-{index}"></div>"""

    full_code = f"""{html}
<style>
{css_fixed}
</style>"""

    brand = re.sub(r'[^a-z0-9]', '-', name.lower()).strip('-')

    return {
        "name": f"CSS Shape {index}: {name}",
        "description": f"A CSS shape: {name}. Pure CSS shape created with borders and transforms.",
        "category": "other",
        "framework": "css",
        "code": full_code,
        "tags": ["shape", "css", "css-shape", brand],
        "dependencies": []
    }


def main():
    shapes = extract_shapes()
    print(f"Found {len(shapes)} shapes")

    components = []
    for i, shape in enumerate(shapes, 1):
        comp = make_component(shape, i)
        if comp:
            components.append(comp)

    with open(OUTPUT, 'w') as f:
        json.dump(components, f, indent=2)

    print(f"Created {OUTPUT} with {len(components)} components")


if __name__ == '__main__':
    main()
