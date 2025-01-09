import os
import re

def obfuscate_code(file_path):
    with open(file_path, 'r') as file:
        code = file.read()

    # Ersetze Variablennamen
    variables = re.findall(r'\blet\s+(\w+)', code)
    for i, var in enumerate(variables):
        code = re.sub(rf'\b{var}\b', f'var_{i}', code)

    # Ersetze Funktionsnamen
    functions = re.findall(r'fn\s+(\w+)', code)
    for i, func in enumerate(functions):
        code = re.sub(rf'\b{func}\b', f'func_{i}', code)

    with open(file_path, 'w') as file:
        file.write(code)

# Verzeichnis durchlaufen
directory = 'src'
for root, _, files in os.walk(directory):
    for file in files:
        if file.endswith('.rs'):
            obfuscate_code(os.path.join(root, file))