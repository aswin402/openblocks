import json
import os
import urllib.request
import urllib.error
import re
import subprocess

# Mapping from Uiverse directory names to OpenBlocks categories
CATEGORY_MAPPING = {
    "Buttons": "other",
    "Cards": "card",
    "Checkboxes": "form",
    "Forms": "form",
    "Inputs": "form",
    "Notifications": "notification",
    "Patterns": "section",
    "Radio-buttons": "form",
    "Toggle-switches": "form",
    "Tooltips": "other",
    "loaders": "loading"
}

def get_github_token():
    # 1. Try environment variable
    token = os.environ.get("GITHUB_TOKEN")
    if token:
        return token
        
    # 2. Try running gh CLI
    try:
        result = subprocess.run(["gh", "auth", "token"], capture_output=True, text=True, check=True)
        tok = result.stdout.strip()
        if tok:
            return tok
    except Exception:
        pass
        
    return None

GITHUB_TOKEN = get_github_token()

def http_get(url):
    headers = {'User-Agent': 'OpenBlocks-Fetcher/1.0'}
    if GITHUB_TOKEN:
        headers['Authorization'] = f'token {GITHUB_TOKEN}'
        
    req = urllib.request.Request(url, headers=headers)
    try:
        with urllib.request.urlopen(req) as response:
            return response.read()
    except urllib.error.URLError as e:
        print(f"HTTP Error: {e} for URL: {url}")
        return None

def react_jsx_to_html(code):
    return code

def fetch_uiverse():
    print("Fetching Uiverse component list from GitHub (uiverse-io/galaxy)...")
    if GITHUB_TOKEN:
        print("Using authenticated GitHub API requests (higher rate limits).")
    else:
        print("Warning: No GITHUB_TOKEN set. Requests might get rate-limited.")
        
    api_url = "https://api.github.com/repos/uiverse-io/galaxy/contents"
    
    response_data = http_get(api_url)
    if not response_data:
        print("Failed to fetch Uiverse repository contents.")
        return
        
    directories = json.loads(response_data.decode('utf-8'))
    components = []
    
    # Fetch 20 variations per category for a much larger dataset!
    MAX_VARIANTS_PER_CAT = 20
    
    for item in directories:
        dir_name = item['name']
        if item['type'] != 'dir' or dir_name not in CATEGORY_MAPPING:
            continue
            
        openblocks_category = CATEGORY_MAPPING[dir_name]
        print(f"\nProcessing Uiverse category: '{dir_name}' (mapping to OpenBlocks: '{openblocks_category}')...")
        
        # Get files inside this directory
        files_url = f"https://api.github.com/repos/uiverse-io/galaxy/contents/{dir_name}"
        files_data = http_get(files_url)
        if not files_data:
            continue
            
        files = json.loads(files_data.decode('utf-8'))
        
        variants_fetched = 0
        for file in files:
            file_name = file['name']
            if not file_name.endswith('.html'):
                continue
                
            # Clean up variant name
            clean_name = file_name.replace('.html', '')
            if '_' in clean_name:
                parts = clean_name.split('_', 1)
                author = parts[0]
                slug = parts[1].replace('-', ' ').title()
                component_name = f"Uiverse {dir_name[:-1] if dir_name.endswith('s') else dir_name}: {slug} (by {author})"
            else:
                slug = clean_name.replace('-', ' ').title()
                component_name = f"Uiverse {dir_name[:-1] if dir_name.endswith('s') else dir_name}: {slug}"
            
            # Fetch raw content
            raw_url = f"https://raw.githubusercontent.com/uiverse-io/galaxy/master/{dir_name}/{file_name}"
            raw_content = http_get(raw_url)
            if not raw_content:
                continue
                
            html_code = raw_content.decode('utf-8')
            
            # Detect framework
            framework = "css"
            dependencies = []
            if "<style>" in html_code:
                framework = "css"
            else:
                if "class=" in html_code:
                    framework = "tailwind"
                    dependencies = ["tailwindcss"]
                    
            # Parse tags
            tags = [dir_name.lower(), "uiverse", "ui-element"]
            tag_match = re.search(r'Tags:\s*([a-zA-Z0-9,\s\-]+)', html_code)
            if tag_match:
                extracted_tags = [t.strip().lower() for t in tag_match.group(1).split(',')]
                tags.extend(extracted_tags)
            
            component = {
                "name": component_name,
                "description": f"A beautiful community-designed {dir_name[:-1] if dir_name.endswith('s') else dir_name} element from Uiverse.io.",
                "category": openblocks_category,
                "framework": framework,
                "code": html_code,
                "tags": tags,
                "dependencies": dependencies
            }
            
            components.append(component)
            print(f"  -> Fetched '{slug}'")
            
            variants_fetched += 1
            if variants_fetched >= MAX_VARIANTS_PER_CAT:
                break
                
    # Save to data directory
    os.makedirs("data", exist_ok=True)
    output_path = "data/uiverse_components.json"
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(components, f, indent=2, ensure_ascii=False)
        
    print(f"\nSuccess! Fetched {len(components)} Uiverse components.")
    print(f"Components saved to '{output_path}'.")

if __name__ == "__main__":
    fetch_uiverse()
