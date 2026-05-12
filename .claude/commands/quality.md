# quality

Run code quality checks for eDEX-UI (lint, syntax validation, structure checks).

## Steps

eDEX-UI has no configured linter or test suite, so quality checks are structural:

1. **JavaScript syntax check** (catches parse errors before running):
   ```bash
   find src -name "*.js" -not -path "*/node_modules/*" -not -path "*/vendor/*" | xargs node --check
   ```
   Report any files that fail to parse.

2. **JSON validation** (themes, keyboard layouts, settings defaults):
   ```bash
   find src/assets/themes src/assets/kb_layouts -name "*.json" | while read f; do
     node -e "JSON.parse(require('fs').readFileSync('$f'))" 2>&1 && echo "OK: $f" || echo "FAIL: $f"
   done
   ```

3. **Electron security check** (inline `/electron-audit`):
   ```bash
   grep -rn "contextIsolation: false\|nodeIntegration: true\|enableRemoteModule: true\|require('@electron/remote')" src/ --include="*.js"
   ```
   Any matches = FAIL.

4. **Dead code / unused require check:**
   Scan new/modified files for `require(` statements in renderer-side class files — after Phase 1 these should all be gone.

5. **Interval leak check:**
   For any new class file, verify every `setInterval` has a corresponding `clearInterval` path.

6. **Summary:** PASS / FAIL with counts per category.

Source: adapted from [Claude Skills Starter](https://github.com/angakh/claude-skills-starter)
