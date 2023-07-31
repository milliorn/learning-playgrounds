function jsonToMatrix(arr) {
  const record = [];
  let idxArr = [];

  // step 1: build the index map
  arr.forEach((item) => {
    const mp = buildIndexMp(item);
    idxArr = idxArr.concat([...Object.keys(mp)]);
    record.push(mp);
  });

  // step 2: remove duplicates and sort the indexes
  idxArr = Array.from(new Set(idxArr)).sort();

  const res = [idxArr];

  // step 3: check row by row
  for (let i = 0; i < record.length; ++i) {
    const cur = [];

    for (let j = 0; j < idxArr.length; ++j) {
      if (record[i][idxArr[j]] !== undefined) {
        cur.push(record[i][idxArr[j]]);
      } else {
        cur.push("");
      }
    }

    res.push(cur);
  }

  return res;
}

// function to check whether an object is a Value
const isValue = (val) =>
  val === null || ["string", "boolean", "number"].includes(typeof val);

// build the index map
function buildIndexMp(item) {
  const mp = {};

  const flatten = (obj, preIdx) => {
    for (const key in obj) {
      const curIdx = preIdx ? `${preIdx}.${key}` : key;
      if (!isValue(obj[key])) {
        flatten(obj[key], curIdx);
      } else {
        mp[curIdx] = obj[key];
      }
    }
  };

  flatten(item, "");
  return mp;
}
