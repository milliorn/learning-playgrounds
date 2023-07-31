/**
 * @param {any} o1
 * @param {any} o2
 * @return {boolean}
 */
function areDeeplyEqual(o1, o2) {
  let a = Array.isArray(o1);
  let b = Array.isArray(o2);

  if (a && b) {
    if (o1.length !== o2.length) {
      return false;
    }

    for (let i = 0; i < o1.length; i++) {
      if (!areDeeplyEqual(o1[i], o2[i])) {
        return false;
      }
    }
    return true;
  }

  if (a || b) {
    return false;
  }

  let x = typeof o1 === "object" && o1 !== null;
  let y = typeof o2 === "object" && o2 !== null;

  if (!x && !y) {
    return o1 === o2;
  }

  if (x !== y) {
    return false;
  }

  if (Object.keys(o1).length !== Object.keys(o2).length) {
    return false;
  }

  for (const key of Object.keys(o1)) {
    if (o2[key] === undefined) {
      return false;
    }

    if (!areDeeplyEqual(o1[key], o2[key])) {
      return false;
    }
  }

  return true;
}
