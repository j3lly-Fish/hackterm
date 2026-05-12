class AlertManager {
    constructor() {
        this._lastFired = {};
        this._panelIds = {
            CPU:  "mod_cpuinfo",
            RAM:  "mod_ramwatcher",
            TEMP: "mod_cpuinfo"
        };
    }

    check(metric, value) {
        const alerts = window.settings.alerts;
        if (!alerts || !alerts.enabled) return;

        let threshold;
        switch (metric) {
            case "CPU":  threshold = alerts.cpuThreshold;  break;
            case "RAM":  threshold = alerts.ramThreshold;  break;
            case "TEMP": threshold = alerts.tempThreshold; break;
            default: return;
        }

        if (typeof threshold !== "number") return;
        if (value >= threshold) this._trigger(metric, value, threshold);
    }

    _trigger(metric, value, threshold) {
        const cooldown = ((window.settings.alerts && window.settings.alerts.cooldownSeconds) || 30) * 1000;
        const now = Date.now();

        if (this._lastFired[metric] && (now - this._lastFired[metric]) < cooldown) return;
        this._lastFired[metric] = now;

        // Flash the relevant panel
        const panelEl = document.getElementById(this._panelIds[metric]);
        if (panelEl) {
            panelEl.classList.add("alert-flash");
            setTimeout(() => panelEl.classList.remove("alert-flash"), 1500);
        }

        // Audio alarm
        if (window.audioManager && window.audioManager.alarm) {
            window.audioManager.alarm.play();
        }

        // Alert modal
        const unit = metric === "TEMP" ? "°C" : "%";
        new Modal({
            type: "warning",
            title: `${metric} Alert`,
            message: `${metric} is at <strong>${Math.round(value)}${unit}</strong>, exceeding the configured threshold of ${threshold}${unit}.`
        });

        // Log to main process
        ipc.send("log", "warn", `AlertManager: ${metric} threshold exceeded (${Math.round(value)}${unit} >= ${threshold}${unit})`);
    }
}

module.exports = { AlertManager };
