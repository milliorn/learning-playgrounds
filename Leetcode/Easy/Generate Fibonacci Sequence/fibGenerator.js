function* fibGenerator() {
  let [prev, curr] = [0, 1];

  while (true) {
    yield prev;
    [prev, curr] = [curr, prev + curr];
  }
}
