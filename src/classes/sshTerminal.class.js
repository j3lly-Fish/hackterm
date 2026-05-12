class SshTerminal {
    constructor(opts) {
        const { Client }   = require("ssh2");
        const WsServer     = require("ws").Server;
        const ipc          = require("electron").ipcMain;
        const fs           = require("fs");
        const os           = require("os");

        this.port     = opts.port;
        this._closed  = false;
        this._stream  = null;
        this._conn    = new Client();
        this._renderer = null;

        this.onclosed      = () => {};
        this.onopened      = () => {};
        this.onresized     = () => {};
        this.ondisconnected = () => {};

        // Build auth config
        const authConfig = {
            host:         opts.host,
            port:         opts.sshPort || 22,
            username:     opts.user,
            readyTimeout: 10000
        };
        if (opts.password) {
            authConfig.password = opts.password;
        } else if (opts.keyPath) {
            const resolved = opts.keyPath.replace(/^~/, os.homedir());
            authConfig.privateKey = fs.readFileSync(resolved);
            if (opts.passphrase) authConfig.passphrase = opts.passphrase;
        }

        // IPC: renderer startup notification + resize passthrough
        ipc.on("terminal_channel-" + this.port, (e, ...args) => {
            switch (args[0]) {
                case "Renderer startup":
                    this._renderer = e.sender;
                    // Report the SSH host as the running "process" so the tab label updates
                    this._renderer.send("terminal_channel-" + this.port, "New process", "ssh:" + opts.host);
                    break;
                case "Resize":
                    if (this._stream) {
                        const cols = Number(args[1]);
                        const rows = Number(args[2]);
                        try { this._stream.setWindow(rows, cols, 0, 0); } catch(e) {}
                        this.onresized(cols, rows);
                    }
                    break;
                default:
                    break;
            }
        });

        // WebSocket server — same pattern as Terminal server role
        this.wss = new WsServer({ port: this.port, clientTracking: true });

        this.wss.on("connection", ws => {
            this.onopened();

            this._conn
                .on("ready", () => {
                    this._conn.shell(
                        { term: "xterm-256color", cols: 80, rows: 24 },
                        (err, stream) => {
                            if (err) {
                                try { ws.send("\r\nSSH shell error: " + err.message + "\r\n"); } catch(e) {}
                                ws.close();
                                return;
                            }
                            this._stream = stream;

                            stream.on("data", data => {
                                try { ws.send(data); } catch(e) {}
                            });
                            stream.stderr.on("data", data => {
                                try { ws.send(data); } catch(e) {}
                            });
                            stream.on("close", () => {
                                this._closed = true;
                                ws.close();
                                this._conn.end();
                                this.onclosed();
                            });

                            ws.on("message", msg => {
                                try { stream.write(msg); } catch(e) {}
                            });
                            ws.on("close", () => {
                                try { stream.close(); } catch(e) {}
                                this._conn.end();
                                this.ondisconnected();
                            });
                        }
                    );
                })
                .on("error", err => {
                    try { ws.send("\r\nSSH connection failed: " + err.message + "\r\n"); } catch(e) {}
                    ws.close();
                    this._closed = true;
                    this.onclosed();
                })
                .connect(authConfig);
        });

        this.close = () => {
            this._closed = true;
            try { if (this._stream) this._stream.close(); } catch(e) {}
            try { this._conn.end(); } catch(e) {}
            ipc.removeAllListeners("terminal_channel-" + this.port);
        };
    }
}

module.exports = { SshTerminal };
