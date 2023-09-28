// https://www.electronjs.org/docs/latest/tutorial/quick-start#opening-your-web-page-in-a-browser-window
// // include the Node.js 'path' module at the top of your file
const path = require('node:path')
const { app, BrowserWindow, ipcMain } = require('electron')

// https://www.electronjs.org/docs/latest/tutorial/ipc#1-listen-for-events-with-ipcmainon
function handleSetTitle(event, title) {
  const webContents = event.sender
  const win = BrowserWindow.fromWebContents(webContents)
  win.setTitle(title)
}

const createWindow = () => {
  // Create the browser window.
  const win = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'), // use a preload script
    }
  })

  // https://www.electronjs.org/docs/latest/tutorial/ipc#pattern-1-renderer-to-main-one-way
  ipcMain.on('set-title', (event, title) => {
    const webContents = event.sender
    const win = BrowserWindow.fromWebContents(webContents)
    win.setTitle(title)
  })

  // and load the index.html of the app.
  win.loadFile('index.html')
  // Open the DevTools.
  // mainWindow.webContents.openDevTools()

  // https://www.electronjs.org/docs/latest/tutorial/process-model#window-management
  // win.loadURL('https://github.com')
  // const contents = win.webContents
  // console.log(contents)
}

// This method will be called when Electron has finished
// initialization and is ready to create browser windows.
// Some APIs can only be used after this event occurs.
app.whenReady().then(() => {
  ipcMain.on('set-title', handleSetTitle)
  ipcMain.handle('ping', () => 'pong')
  createWindow()

  // https://www.electronjs.org/docs/latest/tutorial/quick-start#open-a-window-if-none-are-open-macos
  app.on('activate', () => {
    // On macOS it's common to re-create a window in the app when the
    // dock icon is clicked and there are no other windows open.
    if (BrowserWindow.getAllWindows().length === 0) createWindow()
  })
})

// https://www.electronjs.org/docs/latest/tutorial/quick-start#manage-your-windows-lifecycle
// Quit when all windows are closed, except on macOS. There, it's common
// for applications and their menu bar to stay active until the user quits
// explicitly with Cmd + Q.
app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') app.quit()
})

// In this file you can include the rest of your app's specific main process
// code. You can also put them in separate files and require them here.