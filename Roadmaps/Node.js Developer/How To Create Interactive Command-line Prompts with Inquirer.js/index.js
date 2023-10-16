const inquirerPromise = import('inquirer');

inquirerPromise.then(async ({ default: inquirer }) => {
  inquirer
    .prompt([
      {
        name: 'faveReptile',
        message: 'What is your favorite reptile?',
        default: 'Alligators'
      },
      {
        name: 'faveColor',
        message: 'What is your favorite color?',
        default: '#008f68'
      },
    ])
    .then(answers => {
      console.info('Answers:', answers);
    });
});
