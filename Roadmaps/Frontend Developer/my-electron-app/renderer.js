const func = async () => {
  const response = await window.versions.ping()
  console.log(response) // prints out 'pong'
}

func()

const setButton = document.getElementById('btn')
const titleInput = document.getElementById('title')
setButton.addEventListener('click', () => {
  const title = titleInput.value
  window.electronAPI.setTitle(title)
})