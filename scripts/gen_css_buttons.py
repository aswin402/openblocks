#!/usr/bin/env python3
"""
Extract CSS buttons from the getcssscan.com page HTML and convert to OpenBlocks seed format.

Parses the `const buttons = [...]` JavaScript array from the saved HTML file.
"""

import json
import re
import sys
import textwrap

def parse_buttons_array(html_content: str):
    """Extract the buttons array from the HTML and parse each button."""
    # Find the buttons array
    match = re.search(r'const buttons = (\[.*?\]);', html_content, re.DOTALL)
    if not match:
        print("ERROR: Could not find buttons array in HTML", file=sys.stderr)
        sys.exit(1)

    array_text = match.group(1)
    buttons = []

    # Parse each button object from the array
    # Strategy: find each top-level {...} object
    depth = 0
    start = None
    i = 0
    while i < len(array_text):
        ch = array_text[i]
        if ch == '{':
            if depth == 0:
                start = i
            depth += 1
        elif ch == '}':
            depth -= 1
            if depth == 0 and start is not None:
                obj_text = array_text[start:i+1]
                button = parse_button_obj(obj_text)
                if button:
                    buttons.append(button)
                start = None
        i += 1

    return buttons


def extract_backtick_field(text: str, field_name: str):
    """Extract the value of a backtick-delimited field like `css: `...```"""
    pattern = rf"{re.escape(field_name)}:\s*`"
    match = re.search(pattern, text)
    if not match:
        return None
    start = match.end()  # position after the opening backtick
    depth = 0
    i = start
    while i < len(text):
        if text[i] == '`' and (i == 0 or text[i-1] != '\\'):
            if depth == 0:
                return text[start:i]
        elif text[i] == '$' and i + 1 < len(text) and text[i+1] == '{':
            depth += 1
        elif text[i] == '}' and depth > 0:
            depth -= 1
        i += 1
    return None


def extract_string_field(text: str, field_name: str):
    """Extract a simple string field like `credits: 'Dribbble'`"""
    pattern = rf"{re.escape(field_name)}:\s*'([^']*)'"
    match = re.search(pattern, text)
    if match:
        return match.group(1)
    pattern2 = rf'{re.escape(field_name)}:\s*"([^"]*)"'
    match2 = re.search(pattern2, text)
    if match2:
        return match2.group(1)
    return None


def extract_li_background(text: str):
    """Extract liBackground field value"""
    for field in ['liBackground', 'li_background']:
        pattern = rf"{re.escape(field)}:\s*'([^']*)'"
        match = re.search(pattern, text)
        if match:
            return match.group(1)
    return None


def parse_button_obj(obj_text: str):
    """Parse a single button object from the buttons array."""
    credits = extract_string_field(obj_text, 'credits')
    css = extract_backtick_field(obj_text, 'css')
    html = extract_backtick_field(obj_text, 'html')
    li_bg = extract_li_background(obj_text)

    if not credits or not css:
        return None

    return {
        'credits': credits,
        'css': css,
        'html': html,
        'liBackground': li_bg,
    }


def make_component_name(credits: str, index: int):
    """Generate a clean component name from credits."""
    # Clean up special chars
    name = credits.strip()
    # Remove domain-like suffixes
    name = re.sub(r'\.(com|io|app|art|earth|tech)$', '', name)
    # Capitalize each word
    name = ' '.join(w.capitalize() for w in name.split())
    return f"CSS Button {index}: {name}"


def clean_indent(text: str) -> str:
    """Remove common leading whitespace from all lines."""
    lines = text.split('\n')
    # Find minimum indentation among non-empty lines
    indent = None
    for line in lines:
        stripped = line.lstrip()
        if stripped:  # non-empty line
            leading = len(line) - len(stripped)
            if indent is None or leading < indent:
                indent = leading
    if indent is None or indent == 0:
        return text
    # Remove that indentation from each line
    result = '\n'.join(line[indent:] if len(line) >= indent else line for line in lines)
    return result


def button_to_component(button: dict, index: int):
    """Convert a parsed button to OpenBlocks NewComponent format."""
    credits = button['credits']
    css = button['css']
    html = button.get('html')
    li_bg = button.get('liBackground')

    # Replace {index} placeholder with actual button number
    css_fixed = css.replace('{index}', str(index))
    css_fixed = clean_indent(css_fixed)
    class_name = f"button-{index}"
    
    # Default HTML if not specified
    if html:
        html_fixed = html.replace('{index}', str(index))
        html_fixed = clean_indent(html_fixed)
    else:
        html_fixed = f'<button class="{class_name}">Button {index}</button>'

    # Wrap code: HTML + <style> block
    code = f'{html_fixed}\n<style>\n{css_fixed}\n</style>'

    # Create clean brand tag from credits
    brand_tag = credits.lower().strip()
    brand_tag = re.sub(r'[^a-z0-9]', '-', brand_tag)
    brand_tag = re.sub(r'-+', '-', brand_tag).strip('-')

    # Description
    desc = f"A CSS button inspired by {credits}'s design system with hover/active/focus states."
    if li_bg:
        desc += f" Background color: {li_bg}."

    template_desc = f"CSS button from {credits} with custom styling."

    return {
        "name": make_component_name(credits, index),
        "description": desc,
        "category": "other",
        "framework": "css",
        "code": code,
        "tags": ["button", "css", "css-button", brand_tag],
        "dependencies": []
    }


def main():
    html_path = '/tmp/css_buttons_page.html'

    try:
        with open(html_path, 'r') as f:
            html_content = f.read()
    except FileNotFoundError:
        print(f"ERROR: HTML file not found at {html_path}", file=sys.stderr)
        print("Please save the page content first.", file=sys.stderr)
        sys.exit(1)

    print("Parsing buttons array from HTML...")
    buttons = parse_buttons_array(html_content)
    print(f"Found {len(buttons)} buttons")

    if len(buttons) != 92:
        print(f"WARNING: Expected 92 buttons, found {len(buttons)}", file=sys.stderr)

    # Convert to components
    components = []
    for i, btn in enumerate(buttons, 1):
        comp = button_to_component(btn, i)
        components.append(comp)

    # Save
    output_path = 'data/css_buttons.json'
    with open(output_path, 'w') as f:
        json.dump(components, f, indent=2)

    print(f"Created {output_path} with {len(components)} components")

    # Print summary
    for c in components:
        print(f"  - {c['name']} [{c['category']}]")


if __name__ == '__main__':
    main()
