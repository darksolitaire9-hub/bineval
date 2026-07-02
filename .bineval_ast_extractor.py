import os
import sys
import ast
import json

def extract_imports(repo_path):
    ast_imports = set()
    for root, _, files in os.walk(os.path.join(repo_path, 'scripts')):
        for f in files:
            if f.endswith('.py'):
                path = os.path.join(root, f)
                try:
                    with open(path, 'r', encoding='utf-8') as pf:
                        tree = ast.parse(pf.read())
                    for node in ast.walk(tree):
                        if isinstance(node, ast.Import):
                            for alias in node.names:
                                ast_imports.add(alias.name)
                        elif isinstance(node, ast.ImportFrom):
                            if node.module:
                                parts = node.module.split('.')
                                for i in range(1, len(parts)+1):
                                    ast_imports.add('.'.join(parts[:i]))
                except Exception:
                    pass
    return list(ast_imports)

if __name__ == '__main__':
    if len(sys.argv) > 1:
        repo_path = sys.argv[1]
    else:
        repo_path = "."
    print(json.dumps(extract_imports(repo_path)))
