# Code Review

Perform a comprehensive code quality review of the current changes or a specified file/feature.

## Instructions

Follow these steps to conduct a thorough code review:

1. **Scope**
   - If called with an argument (e.g. `/code-review src/classes/alertManager.class.js`), review that file/directory
   - Otherwise, review all files modified in the current git branch vs master: `git diff master --name-only`

2. **Code Quality Assessment**
   - Scan for code smells, anti-patterns, and potential bugs
   - Check for consistent style with the existing codebase (ES6 classes, no TypeScript, CommonJS `require`)
   - Identify unused variables or dead code
   - Review error handling — does it match existing patterns (signale logging, Modal error display)?

3. **Security Review (Electron-specific)**
   - Check for any new `require()` calls in renderer-side code (should use `window.edex.*` after Phase 1)
   - Look for unsafe innerHTML usage without `_escapeHtml` or `_purifyCSS`
   - Verify new IPC channels have input validation in the main process handler

4. **Performance Analysis**
   - Flag any synchronous file I/O in the renderer (use async alternatives)
   - Check for missing `clearInterval`/`removeEventListener` cleanup that could cause memory leaks
   - Review polling intervals — should be consistent with existing patterns (500ms–3s)

5. **Architecture & Design**
   - Does new code follow the existing class-per-component pattern in `src/classes/`?
   - Are new settings fields added to both the default creation block in `_boot.js` AND the settings modal in `_renderer.js`?
   - Do new IPC channels follow the `kebab-case-name` naming convention?

6. **Recommendations**
   - Prioritize issues: Critical → High → Medium → Low
   - Provide specific file paths and line numbers
   - Suggest the minimal fix, not a refactor

Source: adapted from [Claude Command Suite](https://github.com/qdhenry/Claude-Command-Suite)
