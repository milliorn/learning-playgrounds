// cache individual pages
const cacheName = "cache_1";

const cacheAssets = [
  "./about.html",
  "./index.html",
  "./css/style.css",
  "./js/main.js",
];

// create events
self.addEventListener("install", (e) => {
  console.log("Service Worker has been installed");

  e.waitUntil(
    caches.open(cacheName).then((cache) => {
      console.log("Service Worker is caching files");
      cache.addAll(cacheAssets);
    })
  );
});

self.addEventListener("activate", (e) => {
  console.log("Service Worker has activated");
  e.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.map((cache) => {
          if (cache !== cacheName) {
            console.log("Service Worker is clearing old cache.");
            return caches.delete(cache);
          }
        })
      );
    })
  );
});

// this will clear the cache if we are offline
self.addEventListener("fetch", (e) => {
  console.log("Service Worker is fetching...");
  e.respondWith(fetch(e.request).catch(() => caches.match(e.request)));
});
