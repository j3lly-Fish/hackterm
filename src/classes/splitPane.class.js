class SplitPane {
    constructor() {
        this.active = false;
        this.leftTerm = 0;
        this.rightTerm = null;
        this.currentFocus = "left";
        this.ratio = window.settings.splitRatio || 0.5;
        this._dragCleanup = null;
    }

    toggle() {
        this.active ? this.disable() : this.enable();
    }

    enable() {
        if (this.active) return;

        // Find an existing second terminal
        let rightIndex = null;
        for (let i = 1; i <= 4; i++) {
            if (window.term[i] && typeof window.term[i] === "object") {
                rightIndex = i;
                break;
            }
        }

        if (rightIndex === null) {
            this._spawnAndEnable();
        } else {
            this._doEnable(rightIndex);
        }
    }

    _spawnAndEnable() {
        let slot = null;
        for (let i = 1; i <= 4; i++) {
            if (typeof window.term[i] === "undefined") {
                slot = i;
                break;
            }
        }
        if (slot === null) return;

        window.term[slot] = null;
        ipc.send("ttyspawn", "true");
        ipc.once("ttyspawn-reply", (e, r) => {
            if (r.startsWith("SUCCESS")) {
                let port = Number(r.substr(9));
                window.term[slot] = new Terminal({
                    role: "client",
                    parentId: "terminal" + slot,
                    port
                });
                window.term[slot].onclose = () => {
                    document.getElementById("terminal" + slot).innerHTML = "";
                    window.term[slot].term.dispose();
                    delete window.term[slot];
                    if (this.active) this.disable();
                };
                window.term[slot].onprocesschange = p => {
                    const tab = document.getElementById("shell_tab" + slot);
                    if (tab) tab.innerHTML = `<p>#${slot + 1} - ${p}</p>`;
                };
                setTimeout(() => this._doEnable(slot), 500);
            }
        });
    }

    _doEnable(rightIndex) {
        this.active = true;
        this.leftTerm = 0;
        this.rightTerm = rightIndex;

        const container = document.getElementById("main_shell_innercontainer");

        // Hide the tab bar
        const tabBar = document.getElementById("main_shell_tabs");
        if (tabBar) tabBar.style.display = "none";

        // Grab terminal elements before clearing the container
        const leftEl = document.getElementById("terminal" + this.leftTerm);
        const rightEl = document.getElementById("terminal" + this.rightTerm);

        const others = [];
        for (let i = 0; i <= 4; i++) {
            if (i !== this.leftTerm && i !== this.rightTerm) {
                const el = document.getElementById("terminal" + i);
                if (el) others.push(el);
            }
        }

        // Build pane structure
        const leftPane = document.createElement("div");
        leftPane.className = "pane";
        leftPane.id = "pane_left";

        const divider = document.createElement("div");
        divider.className = "pane-divider";
        divider.id = "pane_divider";

        const rightPane = document.createElement("div");
        rightPane.className = "pane";
        rightPane.id = "pane_right";

        leftPane.style.flex = String(this.ratio);
        rightPane.style.flex = String(1 - this.ratio);

        // Hidden holder for inactive terminals
        const hidden = document.createElement("div");
        hidden.id = "pane_hidden";
        hidden.style.display = "none";
        others.forEach(el => { el.className = ""; hidden.appendChild(el); });

        container.innerHTML = "";
        container.classList.add("split");

        container.appendChild(hidden);
        container.appendChild(leftPane);
        container.appendChild(divider);
        container.appendChild(rightPane);
        leftPane.appendChild(leftEl);
        rightPane.appendChild(rightEl);

        leftPane.addEventListener("click", () => this._focusPane("left"), true);
        rightPane.addEventListener("click", () => this._focusPane("right"), true);

        this._setupDrag(divider, leftPane, rightPane, container);

        // Focus left pane by default
        this.currentFocus = "left";
        window.currentTerm = this.leftTerm;
        window.term[this.leftTerm].fit();
        window.term[this.leftTerm].term.focus();
        window.term[this.rightTerm].fit();
    }

    _focusPane(side) {
        this.currentFocus = side;
        const termIndex = side === "left" ? this.leftTerm : this.rightTerm;
        window.currentTerm = termIndex;
        window.term[termIndex].term.focus();
        window.term[termIndex].resendCWD();
    }

    _setupDrag(divider, leftPane, rightPane, container) {
        let dragging = false;

        const onDown = e => {
            dragging = true;
            divider.classList.add("dragging");
            e.preventDefault();
        };
        const onMove = e => {
            if (!dragging) return;
            const rect = container.getBoundingClientRect();
            let newRatio = (e.clientX - rect.left) / rect.width;
            newRatio = Math.max(0.2, Math.min(0.8, newRatio));
            this.ratio = newRatio;
            leftPane.style.flex = String(newRatio);
            rightPane.style.flex = String(1 - newRatio);
        };
        const onUp = () => {
            if (!dragging) return;
            dragging = false;
            divider.classList.remove("dragging");
            window.term[this.leftTerm].fit();
            window.term[this.rightTerm].fit();
        };
        const onDblClick = () => {
            this.ratio = 0.5;
            leftPane.style.flex = "1";
            rightPane.style.flex = "1";
            window.term[this.leftTerm].fit();
            window.term[this.rightTerm].fit();
        };

        divider.addEventListener("mousedown", onDown);
        document.addEventListener("mousemove", onMove);
        document.addEventListener("mouseup", onUp);
        divider.addEventListener("dblclick", onDblClick);

        this._dragCleanup = () => {
            divider.removeEventListener("mousedown", onDown);
            document.removeEventListener("mousemove", onMove);
            document.removeEventListener("mouseup", onUp);
            divider.removeEventListener("dblclick", onDblClick);
        };
    }

    disable() {
        if (!this.active) return;
        this.active = false;

        if (this._dragCleanup) {
            this._dragCleanup();
            this._dragCleanup = null;
        }

        const container = document.getElementById("main_shell_innercontainer");

        // Collect all terminal elements from wherever they live in the DOM
        const termEls = [];
        for (let i = 0; i <= 4; i++) {
            termEls[i] = document.getElementById("terminal" + i);
        }

        container.innerHTML = "";
        container.classList.remove("split");

        for (let i = 0; i <= 4; i++) {
            if (termEls[i]) {
                termEls[i].className = "";
                container.appendChild(termEls[i]);
            }
        }

        const tabBar = document.getElementById("main_shell_tabs");
        if (tabBar) tabBar.style.display = "";

        window.focusShellTab(window.currentTerm);
    }
}

module.exports = { SplitPane };
