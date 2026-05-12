# deps

Check for outdated packages and security vulnerabilities in eDEX-UI dependencies.

## Steps

1. **Root dependencies** (build tooling: electron, electron-builder, terser, etc.):
   ```bash
   cd /home/anti/Documents/github/edex-ui && npm audit --audit-level=moderate
   npm outdated
   ```

2. **App dependencies** (`src/` — runtime deps: xterm, node-pty, systeminformation, ssh2, etc.):
   ```bash
   cd /home/anti/Documents/github/edex-ui/src && npm audit --audit-level=moderate
   npm outdated
   ```

3. **Report:**
   - List all high/critical vulnerabilities with CVE numbers and affected package
   - List packages that are significantly outdated (major version behind)
   - Flag any packages with no updates available but known issues (note manually)
   - Highlight Electron version specifically — cross-reference against Electron Security Releases page

4. **Recommended actions:**
   - For each vulnerability: suggest `npm audit fix` or manual version pin
   - Note any updates that would require code changes (breaking API changes)

Source: adapted from [Claude Skills Starter](https://github.com/angakh/claude-skills-starter)
