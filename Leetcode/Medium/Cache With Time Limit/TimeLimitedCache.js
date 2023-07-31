class TimeLimitedCache {
  constructor() {
    this.cache = new Map(); // Use Map to store key-value pairs and their expiration times
  }

  set(key, value, duration) {
    const expirationTime = Date.now() + duration;
    const exists = this.cache.has(key);

    this.cache.set(key, { value, expirationTime });

    return exists;
  }

  get(key) {
    if (this.cache.has(key)) {
      const { value, expirationTime } = this.cache.get(key);
      if (expirationTime > Date.now()) {
        return value;
      } else {
        this.cache.delete(key); // Remove expired key
      }
    }
    return -1;
  }

  count() {
    let count = 0;
    const currentTime = Date.now();
    for (const [key, { expirationTime }] of this.cache) {
      if (expirationTime > currentTime) {
        count++;
      } else {
        this.cache.delete(key); // Remove expired key
      }
    }
    return count;
  }
}
