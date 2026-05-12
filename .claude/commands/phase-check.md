# phase-check

Report the current implementation status against the eDEX-UI feature expansion plan.

## Steps

1. **Read the plan** at `/home/anti/.claude/plans/humming-humming-honey.md` to understand all phases and their expected deliverables.

2. **Check Phase 0 (Tooling):**
   - Do `.claude/commands/electron-run.md`, `electron-audit.md`, and `phase-check.md` exist? ✓/✗
   - Are community skills installed? Check if any skill files from Claude Command Suite or Skills Starter exist in `.claude/commands/`.

3. **Check Phase 1 (Security):**
   - Run `/electron-audit` logic inline: grep for `contextIsolation: false`, `nodeIntegration: true`, `enableRemoteModule`, `@electron/remote` in `src/`
   - Does `src/preload.js` exist?
   - Status: Not started / In progress / Complete

4. **Check Phase 2 (Split Panes):**
   - Does `src/classes/splitPane.class.js` exist?
   - Does `src/assets/css/main_shell.css` contain `.pane-divider`?
   - Does `shortcuts.json` default in `_boot.js` include `SPLIT_PANE`?
   - Status: Not started / In progress / Complete

5. **Check Phase 3 (Alerts):**
   - Does `src/classes/alertManager.class.js` exist?
   - Does `_boot.js` default settings include `alerts.cpuThreshold`?
   - Status: Not started / In progress / Complete

6. **Check Phase 4 (Settings Editor):**
   - Does `_renderer.js` `openSettings()` function contain `settings_tabs`?
   - Status: Not started / In progress / Complete

7. **Check Phase 5 (SSH):**
   - Does `src/classes/sshTerminal.class.js` exist?
   - Is `ssh2` in `src/package.json` dependencies?
   - Status: Not started / In progress / Complete

8. **Git status:**
   - Run `git status` and `git log --oneline -5` to show recent commits
   - List any modified files outside the expected scope for the current phase

9. **Output a summary table:**
   ```
   Phase 0 (Tooling)     ✓ Complete
   Phase 1 (Security)    ○ Not started
   Phase 2 (Split Panes) ○ Not started
   Phase 3 (Alerts)      ○ Not started
   Phase 4 (Settings)    ○ Not started
   Phase 5 (SSH)         ○ Not started

   Current git branch: master
   Modified files: (list)
   Recommended next step: (one sentence)
   ```
