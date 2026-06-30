import json
import sqlite3
import urllib.request
import urllib.error
import os
import uuid
from datetime import datetime

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

def fetch_and_seed_webgradients():
    print("Fetching WebGradients list from GitHub (itmeo/webgradients)...")
    url = "https://raw.githubusercontent.com/itmeo/webgradients/master/gradients-parsed.json"
    
    data = http_get(url)
    if not data:
        print("Failed to fetch WebGradients JSON.")
        return
        
    raw_gradients = json.loads(data.decode('utf-8'))
    print(f"Loaded {len(raw_gradients)} gradients from raw dataset.")
    
    # Establish connection to sqlite database
    db_path = "openblocks.db"
    if not os.path.exists(db_path):
        print(f"Database {db_path} does not exist. Please compile and run OpenBlocks first to create it.")
        return
        
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()
    
    inserted_count = 0
    for item in raw_gradients:
        name = item.get("name")
        deg = item.get("deg", 135)
        stops = item.get("gradient", [])
        
        if not name or not stops:
            continue
            
        # Reconstruct linear-gradient CSS
        stops_css = []
        colors = []
        for stop in stops:
            color = stop.get("color")
            pos = stop.get("pos", 0)
            if color:
                stops_css.append(f"{color} {pos}%")
                colors.append(color)
                
        css_string = f"linear-gradient({deg}deg, {', '.join(stops_css)})"
        
        # Tags
        tags = ["webgradients", name.lower().replace(" ", "-")]
        for c in colors:
            tags.append(c.lower())
            
        colors_json = json.dumps(colors)
        tags_json = json.dumps(tags)
        
        # Check if gradient name already exists in database
        cursor.execute("SELECT COUNT(*) FROM gradients WHERE name = ?", (name,))
        if cursor.fetchone()[0] == 0:
            uid = str(uuid.uuid4())
            created_at = datetime.utcnow().isoformat() + "Z"
            
            cursor.execute(
                "INSERT INTO gradients (id, name, css, colors, tags, created_at) VALUES (?, ?, ?, ?, ?, ?)",
                (uid, name, css_string, colors_json, tags_json, created_at)
            )
            inserted_count += 1
            
    conn.commit()
    conn.close()
    
    print(f"Successfully seeded {inserted_count} new gradients from WebGradients into {db_path}!")

if __name__ == "__main__":
    fetch_and_seed_webgradients()
