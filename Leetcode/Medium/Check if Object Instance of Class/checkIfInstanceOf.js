function checkIfInstanceOf(obj, classFunction) {
  if (obj === undefined) {
    return false;
  }
  while (obj !== null) {
    if (obj.constructor === classFunction) {
      return true;
    }
    obj = Object.getPrototypeOf(obj);
  }
  return false;
}
