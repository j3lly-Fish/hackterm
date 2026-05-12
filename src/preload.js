const { contextBridge, ipcRenderer, webFrame } = require('electron');

// Synchronously fetch app info before the renderer initializes.
// sendSync is justified here: it runs once at startup before any UI exists.
const appInfo = ipcRenderer.sendSync('preload-get-app-info');

contextBridge.exposeInMainWorld('edex', {
    // Synchronous app info (avoids async startup complexity)
    userData:   appInfo.userData,
    appVersion: appInfo.version,
    argv:       appInfo.argv,

    // App lifecycle — renderer cannot access `app` directly without remote
    app: {
        focus:   () => ipcRenderer.send('app-focus'),
        quit:    () => ipcRenderer.send('app-quit'),
        relaunch:() => ipcRenderer.send('app-relaunch'),
    },

    // Window state queries and commands
    win: {
        isFullScreen:      ()       => ipcRenderer.invoke('win-isFullScreen'),
        setFullScreen:     (v)      => ipcRenderer.invoke('win-setFullScreen', v),
        getSize:           ()       => ipcRenderer.invoke('win-getSize'),
        setSize:           (w, h)   => ipcRenderer.invoke('win-setSize', w, h),
        isMaximized:       ()       => ipcRenderer.invoke('win-isMaximized'),
        unmaximize:        ()       => ipcRenderer.send('win-unmaximize'),
        minimize:          ()       => ipcRenderer.send('win-minimize'),
        toggleDevTools:    ()       => ipcRenderer.send('win-toggleDevTools'),
        // Subscribe to window events forwarded from main
        onResize:          (fn)     => ipcRenderer.on('window-resize', fn),
        onLeaveFullscreen: (fn)     => ipcRenderer.on('window-leave-fullscreen', fn),
    },

    // Screen info (main-process only)
    screen: {
        getAllDisplays: () => ipcRenderer.invoke('screen-getAllDisplays'),
    },

    // Shell operations (main-process only since Electron 9)
    shell: {
        openPath:     (p)   => ipcRenderer.invoke('shell-openPath', p),
        openExternal: (url) => ipcRenderer.invoke('shell-openExternal', url),
    },

    // Global keyboard shortcuts — callbacks can't cross the bridge,
    // so we send the shortcut list to main and receive fired events back.
    shortcuts: {
        sync:         (cuts) => ipcRenderer.send('shortcuts-sync', cuts),
        unregisterAll:()     => ipcRenderer.send('shortcuts-unregisterAll'),
        onFired:      (fn)   => ipcRenderer.on('shortcut-fired', (e, info) => fn(info)),
    },

    // webFrame is renderer-side but exposed here for a clean API surface
    webFrame: {
        setVisualZoomLevelLimits: (min, max) => webFrame.setVisualZoomLevelLimits(min, max),
    },
});
