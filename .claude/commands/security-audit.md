# Security Audit Command

Perform comprehensive security assessment of the eDEX-UI codebase.

## Instructions

Perform a systematic security audit following these steps:

1. **Environment Setup**
   - Identify the technology stack (Electron 12, Node.js, xterm.js, node-pty, WebSockets)
   - Check for existing security tools and configurations
   - Review `webPreferences` in `src/_boot.js`

2. **Dependency Security**
   - Run `npm audit` in both root and `src/` directories
   - Check Electron version against known CVEs
   - Flag any high/critical severity findings

3. **Electron-Specific Security**
   - Verify `contextIsolation: true` in BrowserWindow config
   - Verify `nodeIntegration: false`
   - Verify `enableRemoteModule: false` (or absent)
   - Check for `src/preload.js` existence and contextBridge usage
   - Scan for `require('@electron/remote')` in renderer files
   - Check `webSecurity` is not set to `false`

4. **Input Validation & Sanitization**
   - Review `window._escapeHtml`, `window._purifyCSS` usage in `_renderer.js`
   - Check all places where user input enters the DOM (innerHTML, etc.)
   - Review shell command construction in terminal spawning

5. **Data Protection**
   - Check that passwords (SSH) are never written to settings.json
   - Verify WebSocket connections are localhost-only (`127.0.0.1`, not `0.0.0.0`)

6. **Secrets Management**
   - Scan for hardcoded credentials or tokens

7. **Reporting**
   - Document all findings with severity: Critical / High / Medium / Low
   - Include file path and line number for each finding
   - Provide specific remediation steps

Source: adapted from [Claude Command Suite](https://github.com/qdhenry/Claude-Command-Suite)
