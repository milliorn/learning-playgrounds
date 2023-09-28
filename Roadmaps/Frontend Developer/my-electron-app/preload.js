const { contextBridge, ipcRenderer } = require('electron')
// https://www.electronjs.org/docs/latest/tutorial/quick-start#access-nodejs-from-the-renderer-with-a-preload-script
// All the Node.js APIs are available in the preload process.
// It has the same sandbox as a Chrome extension.
window.addEventListener('DOMContentLoaded', () => {
  const replaceText = (selector, text) => {
    const element = document.getElementById(selector)
    if (element) element.innerText = text
  }

  for (const dependency of [ 'chrome', 'node', 'electron' ]) {
    replaceText(`${dependency}-version`, process.versions[ dependency ])
  }
})

// https://www.electronjs.org/docs/latest/tutorial/tutorial-preload#communicating-between-processes
contextBridge.exposeInMainWorld('versions', {
  node: () => process.versions.node,
  chrome: () => process.versions.chrome,
  electron: () => process.versions.electron,
  ping: () => ipcRenderer.invoke('ping')
  // we can also expose variables, not just functions
})

contextBridge.exposeInMainWorld('electronAPI', {
  setTitle: (title) => ipcRenderer.send('set-title', title)
})