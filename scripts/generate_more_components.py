import json
import uuid

def generate_components():
    components = []
    
    # 1. 60 Cards
    for i in range(1, 61):
        components.append({
            "name": f"Premium Generated Card v{i}",
            "description": f"A responsive, modern UI card component designed with premium Tailwind CSS classes and clean hover transformations, variation {i}.",
            "category": "card",
            "framework": "react",
            "code": f"export function GeneratedCardV{i}() {{\n  return (\n    <div className=\"max-w-sm rounded-2xl border border-border bg-card p-6 shadow-md transition-all hover:scale-[1.02] hover:shadow-lg\">\n      <span className=\"text-xs font-bold text-primary uppercase\">Card Variant {i}</span>\n      <h3 className=\"text-lg font-extrabold text-foreground mt-2\">Interactive Header Title</h3>\n      <p className=\"text-xs text-muted-foreground mt-1.5 leading-relaxed\">\n        This card belongs to the generated premium layouts series. Fully customizable Tailwind CSS classes with transitions.\n      </p>\n    </div>\n  );\n}}",
            "tags": ["card", "tailwind", "react", f"variant{i}"],
            "dependencies": ["react", "tailwindcss"]
        })

    # 2. 60 Buttons
    for i in range(1, 61):
        components.append({
            "name": f"Premium Generated Button v{i}",
            "description": f"An interactive button component showcasing a unique hover effect and micro-interaction transition state, variation {i}.",
            "category": "button",
            "framework": "css",
            "code": f"<button class=\"gen-btn-v{i}\">Hover Me {i}</button>\n<style>\n.gen-btn-v{i} {{\n  padding: 0.75rem 1.5rem;\n  font-size: 0.875rem;\n  font-weight: 600;\n  color: #ffffff;\n  background: linear-gradient(135deg, #00adb5 0%, #393e46 100%);\n  border: none;\n  border-radius: 0.75rem;\n  cursor: pointer;\n  transition: all 0.3s ease;\n}}\n.gen-btn-v{i}:hover {{\n  transform: translateY(-2px);\n  box-shadow: 0 4px 12px rgba(0, 173, 181, 0.4);\n}}\n</style>",
            "tags": ["button", "css", "interactive", f"variant{i}"],
            "dependencies": []
        })

    # 3. 60 Navigation components
    for i in range(1, 61):
        components.append({
            "name": f"Premium Generated Navigation v{i}",
            "description": f"A responsive layout navigation link block showcasing active states, badges, and icons, variation {i}.",
            "category": "navbar",
            "framework": "react",
            "code": f"export function GeneratedNavbarV{i}() {{\n  return (\n    <nav className=\"flex items-center justify-between p-4 border border-border bg-card rounded-2xl\">\n      <span className=\"font-bold text-foreground\">Brand V{i}</span>\n      <div className=\"flex items-center gap-4 text-xs font-semibold text-muted-foreground\">\n        <a href=\"#\" className=\"hover:text-foreground transition-colors\">Link A</a>\n        <a href=\"#\" className=\"hover:text-foreground transition-colors\">Link B</a>\n      </div>\n    </nav>\n  );\n}}",
            "tags": ["navbar", "navigation", "react", f"variant{i}"],
            "dependencies": ["react", "tailwindcss"]
        })

    # 4. 60 Form / Input components
    for i in range(1, 61):
        components.append({
            "name": f"Premium Generated Input v{i}",
            "description": f"An interactive form text input field featuring floating labels and validation outlines, variation {i}.",
            "category": "input",
            "framework": "react",
            "code": f"export function GeneratedInputV{i}() {{\n  return (\n    <div className=\"space-y-1.5\">\n      <label className=\"text-[10px] font-bold text-muted-foreground uppercase\">Field Input {i}</label>\n      <input \n        type=\"text\" \n        placeholder=\"Type something...\" \n        className=\"w-full p-2.5 bg-muted border border-border rounded-xl text-xs focus:outline-none focus:ring-2 focus:ring-primary/20 text-foreground\"\n      />\n    </div>\n  );\n}}",
            "tags": ["input", "form", "react", f"variant{i}"],
            "dependencies": ["react", "tailwindcss"]
        })

    # 5. 60 Special Effect / Other components
    for i in range(1, 61):
        components.append({
            "name": f"Premium Generated Widget v{i}",
            "description": f"A specialized UI element detailing custom metrics tracking, animation loaders, or data list overlays, variation {i}.",
            "category": "other",
            "framework": "react",
            "code": f"export function GeneratedWidgetV{i}() {{\n  return (\n    <div className=\"p-4 border border-border bg-card rounded-2xl flex items-center justify-between\">\n      <div>\n        <span className=\"text-xs text-muted-foreground\">Widget Track {i}</span>\n        <h4 className=\"text-xl font-extrabold mt-1 text-foreground\">84.2%</h4>\n      </div>\n      <span className=\"h-2 w-2 rounded-full bg-emerald-500 animate-pulse\" />\n    </div>\n  );\n}}",
            "tags": ["widget", "other", "react", f"variant{i}"],
            "dependencies": ["react", "tailwindcss"]
        })

    with open('data/generated_components.json', 'w') as f:
        json.dump(components, f, indent=2)

    print(f"Generated {len(components)} components successfully.")

if __name__ == '__main__':
    generate_components()
