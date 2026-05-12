# electron-run

Start eDEX-UI in development mode and report any security warnings or startup errors.

## Steps

1. Run `npm run start` from the project root directory with a 15-second timeout to capture startup output.

2. While reviewing the output, watch specifically for:
   - Any Electron deprecation warnings mentioning `remote`, `enableRemoteModule`, `contextIsolation`, or `nodeIntegration`
   - Uncaught exceptions or crash messages
   - WebSocket connection failures (port already in use, etc.)
   - `signale` error/fatal log lines

3. Report:
   - Whether the app launched successfully
   - Any security-related warnings (copy the exact warning text)
   - Any startup errors with file + line number
   - The final line(s) of stdout before the process stabilized

## Notes

- The app runs fullscreen by default. Use `--nointro` (already in the start script) to skip the boot animation.
- If the app hangs on startup, check that port 3000 is not already in use: `lsof -i :3000`
- The process will not self-terminate; use Bash timeout or kill it after capturing initial output.
