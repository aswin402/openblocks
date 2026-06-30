import json
import os
import urllib.request
import urllib.error
import re

# Mappings from Tailblocks categories to OpenBlocks Category enum values
CATEGORY_MAPPING = {
    "blog": "blog",
    "contact": "contact",
    "content": "section",
    "cta": "cta",
    "ecommerce": "ecommerce",
    "feature": "feature",
    "footer": "footer",
    "gallery": "card",
    "header": "navbar",
    "hero": "hero",
    "pricing": "pricing",
    "statistic": "section",
    "step": "section",
    "team": "profile",
    "testimonial": "testimonial"
}

def http_get(url):
    req = urllib.request.Request(
        url, 
        headers={'User-Agent': 'OpenBlocks-Fetcher/1.0'}
    )
    try:
        with urllib.request.urlopen(req) as response:
            return response.read()
    except urllib.error.URLError as e:
        print(f"HTTP Error: {e} for URL: {url}")
        return None

def react_jsx_to_html(code):
    # Find the JSX block inside return ( ... )
    start = code.find("return (")
    if start == -1:
        start = code.find("return")
        if start == -1:
            return None
        jsx = code[start+6:].strip()
    else:
        # Find matching close parenthesis from the end
        end = code.rfind(");")
        if end == -1:
            return None
        jsx = code[start+8:end].strip()
        
    # Convert JSX properties to HTML
    html = jsx
    html = html.replace("className=", "class=")
    
    # Resolve React props.theme interpolation
    # Format 1: bg-${props.theme}-500 -> bg-indigo-500
    html = html.replace("${props.theme}", "indigo")
    html = html.replace("props.theme", "'indigo'")
    
    # Format 2: Handle template literals inside class attribute, e.g. class={`text-gray-600 bg-indigo-500`}
    # Replace class={`...`} with class="..."
    html = re.sub(r'class=\{\s*`([^`]+)`\s*\}', r'class="\1"', html)
    html = re.sub(r'class=\{\s*"([^"]+)"\s*\}', r'class="\1"', html)
    
    # Format 3: Clean remaining backticks inside class attributes
    html = html.replace("`", '"')
    
    # Format 4: Clean up any braces left around class names
    html = re.sub(r'class=\{(.*)\}', r'class=\1', html)
    
    return html

def fetch_tailblocks():
    print("Fetching Tailblocks component directory from GitHub (mertJF/tailblocks)...")
    api_url = "https://api.github.com/repos/mertJF/tailblocks/contents/src/blocks"
    
    response_data = http_get(api_url)
    if not response_data:
        print("Failed to fetch Tailblocks blocks directory.")
        return
        
    categories = json.loads(response_data.decode('utf-8'))
    components = []
    
    # We will limit the fetch to 3 variations per category to keep it fast and under API limits
    MAX_VARIANTS_PER_CAT = 3
    
    for cat in categories:
        if cat['type'] != 'dir':
            continue
            
        cat_name = cat['name']
        openblocks_category = CATEGORY_MAPPING.get(cat_name, "other")
        
        print(f"\nProcessing category: '{cat_name}' (mapping to OpenBlocks: '{openblocks_category}')...")
        
        # Build API URL manually to avoid query parameter issues in cat['url']
        light_url = f"https://api.github.com/repos/mertJF/tailblocks/contents/src/blocks/{cat_name}/light"
        files_data = http_get(light_url)
        if not files_data:
            # Fallback directly in the category
            fallback_url = f"https://api.github.com/repos/mertJF/tailblocks/contents/src/blocks/{cat_name}"
            files_data = http_get(fallback_url)
            if not files_data:
                continue
            
        files = json.loads(files_data.decode('utf-8'))
        
        variants_fetched = 0
        for file in files:
            if not file['name'].endswith('.js') or file['name'] == 'index.js':
                continue
                
            file_name = file['name']
            variant_name = file_name.replace('.js', '').upper()
            
            # Fetch raw code
            raw_url = f"https://raw.githubusercontent.com/mertJF/tailblocks/master/src/blocks/{cat_name}/light/{file_name}"
            raw_content = http_get(raw_url)
            if not raw_content:
                # Try fallback directly in the category
                raw_url = f"https://raw.githubusercontent.com/mertJF/tailblocks/master/src/blocks/{cat_name}/{file_name}"
                raw_content = http_get(raw_url)
                if not raw_content:
                    continue
                
            jsx_code = raw_content.decode('utf-8')
            html_code = react_jsx_to_html(jsx_code)
            
            if not html_code:
                continue
                
            # Format to OpenBlocks component schema
            component = {
                "name": f"Tailblocks {cat_name.title()} Variant {variant_name}",
                "description": f"A clean, responsive {cat_name} component from Tailblocks styled with Tailwind CSS.",
                "category": openblocks_category,
                "framework": "tailwind",
                "code": html_code,
                "tags": [cat_name, "tailblocks", "responsive", f"variant-{variant_name.lower()}"],
                "dependencies": ["tailwindcss"]
            }
            
            components.append(component)
            print(f"  -> Fetched Variant {variant_name}")
            
            variants_fetched += 1
            if variants_fetched >= MAX_VARIANTS_PER_CAT:
                break
            
    # Output to JSON
    os.makedirs("data", exist_ok=True)
    output_path = "data/tailblocks_components.json"
    with open(output_path, "w", encoding="utf-8") as f:
        json.dump(components, f, indent=2, ensure_ascii=False)
        
    print(f"\nSuccess! Fetched {len(components)} components from Tailblocks.")
    print(f"Components saved to '{output_path}'.")
    print("\nTo seed the database with these components, run:")
    print("cargo run -- --db-path openblocks.db seed")

if __name__ == "__main__":
    fetch_tailblocks()
