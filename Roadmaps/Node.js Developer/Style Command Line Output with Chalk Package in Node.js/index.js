import chalk from 'chalk';

const log = console.log;

const cpu = {
  totalPercent: 90
};

const ram = {
  used: 40,
  total: 100
};

const disk = {
  used: 70,
  total: 100
};

log(chalk.blue('Hello World!'));
log(chalk.red('Hello world!'));
log(chalk.white('Hello World! Foo bar biz baz'));
log(chalk.green(
  'I am a green line ' +
  chalk.blue.underline.bold('with a blue substring') +
  ' that becomes green again!'
));

log(`
CPU: ${chalk.red('90%')}
RAM: ${chalk.green('40%')}
DISK: ${chalk.yellow('70%')}
`);

log(chalk`
  CPU: {red ${cpu.totalPercent}%}
  RAM: {green ${ram.used / ram.total * 100}%}
  DISK: {rgb(255,131,0) ${disk.used / disk.total * 100}%}
`);

// Use RGB colors in terminal emulators that support it.
log(chalk.rgb(255, 165, 0).underline('Underlined reddish color'));
log(chalk.hex('#808080').bold('Bold gray!'));

// Orange color using hex
log(chalk.hex('#FFA500')('Yay for orange colored text!'));
