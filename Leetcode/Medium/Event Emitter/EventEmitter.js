class EventEmitter {
  constructor() {
    // Map to store event names and corresponding callback arrays
    this.events = new Map();
  }

  subscribe(eventName, callback) {
    // create an empty array for it
    if (!this.events.has(eventName)) {
      this.events.set(eventName, []);
    }

    // array of callbacks for the event
    const eventListeners = this.events.get(eventName);

    // new callback to the array
    eventListeners.push(callback);

    // index of new callback in the array
    const index = eventListeners.length - 1;

    // return object with an unsubscribe method
    return {
      unsubscribe: () => {
        // remove callback from the array
        eventListeners.splice(index, 1);
      },
    };
  }

  emit(eventName, args = []) {
    if (!this.events.has(eventName)) {
      return [];
    }

    // get array of callbacks for the event
    const eventListeners = this.events.get(eventName);

    // array to store the results of callback executions
    const results = [];

    // collect the results
    for (const callback of eventListeners) {
      results.push(callback(...args));
    }

    return results;
  }
}
