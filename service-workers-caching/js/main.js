if ("serviceWorker" in navigator) {
  console.log("serviceWorker found in navigator");

  window.addEventListener("load", () => {
    navigator.serviceWorker
      .register("../sw.js")
      .then((registration) => console.log("Service Worker is registered"))
      .catch((err) => console.error(`Service Worker has an error: ${err}`));
  });
}
