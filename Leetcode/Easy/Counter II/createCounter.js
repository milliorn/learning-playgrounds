/**
 * @param {integer} init
 * @return { increment: Function, decrement: Function, reset: Function }
 */
function createCounter(init) {
  const counter = {
    currentValue: init,
    increment() {
      return ++this.currentValue;
    },
    decrement() {
      return --this.currentValue;
    },
    reset() {
      this.currentValue = init;
      return this.currentValue;
    },
  };
  return counter;
}
