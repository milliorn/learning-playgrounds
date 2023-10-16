const cliProgress = require('cli-progress');

// create a new progress bar instance and use shades_classic theme
const bar1 = new cliProgress.SingleBar({}, cliProgress.Presets.shades_classic);

// start the progress bar with a total value of 200 and start value of 0
bar1.start(200, 0);

// update the current value in your application..
bar1.update(100);

// stop the progress bar
bar1.stop();

// note: you have to install this dependency manually since it's not required by cli-progress
const colors = require('ansi-colors');

// create new progress bar
const b1 = new cliProgress.SingleBar({
  format: 'CLI Progress |' + colors.cyan('{bar}') + '| {percentage}% || {value}/{total} Chunks || Speed: {speed}',
  barCompleteChar: '\u2588',
  barIncompleteChar: '\u2591',
  hideCursor: true
});

// initialize the bar - defining payload token "speed" with the default value "N/A"
b1.start(200, 0, {
  speed: "N/A"
});

// update values
b1.increment();
b1.update(20);

// stop the bar
b1.stop();

// create new container
const multibar = new cliProgress.MultiBar({
  clearOnComplete: false,
  hideCursor: true,
  format: ' {bar} | {filename} | {value}/{total}',
}, cliProgress.Presets.shades_grey);

// add bars
const b2 = multibar.create(1000, 0);

// control bars
b1.increment();
b2.update(20, { filename: "test1.txt" });
b1.update(20, { filename: "helloworld.txt" });

// stop all bars
multibar.stop();