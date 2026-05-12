# electron-audit

Scan the eDEX-UI source for Electron security anti-patterns and report findings with file + line numbers.

## Steps

1. **Scan for security anti-patterns** in `src/` (excluding `node_modules`):

   Run these greps and report every match with filename and line number:
   ```
   grep -rn "contextIsolation: false" src/
   grep -rn "nodeIntegration: true" src/
   grep -rn "enableRemoteModule: true" src/
   grep -rn "require('@electron/remote')" src/
   grep -rn 'require("@electron/remote")' src/
   grep -rn "allowRunningInsecureContent: true" src/
   grep -rn "webSecurity: false" src/
   grep -rn "eval(" src/ --include="*.js"
   grep -rn "shell.openExternal" src/
   ```

2. **Check Electron version vulnerability:**
   - Read `package.json` and note the `electron` version
   - Run `npm audit --audit-level=moderate` from the project root
   - Report any high/critical severity findings

3. **Check for missing preload script:**
   - Look for `preload:` in `src/_boot.js` BrowserWindow config
   - If absent, flag as missing

4. **Summarize:**
   - Total count of anti-pattern matches
   - Pass/Fail verdict: PASS only if zero anti-pattern matches AND npm audit has no high/critical findings
   - List all findings grouped by file

## Expected clean state (after Phase 1)

- Zero matches for all grep patterns
- `src/preload.js` exists and is referenced in `_boot.js`
- `contextIsolation: true`, `nodeIntegration: false` in BrowserWindow config
