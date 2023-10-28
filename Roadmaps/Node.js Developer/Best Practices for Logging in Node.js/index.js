const { logger, customLogger, transportLogger, formatLogger } = require('./custom_levels');

logger.info("System launch"); // {"message":"System launch","level":"info"}
logger.fatal("A critical failure!"); // {"message":"A critical failure!","level":"fatal"}

customLogger.info("System launch"); // {"message":"System launch","level":"info"}
customLogger.fatal("A critical failure!"); // {"message":"A critical failure!","level":"fatal"}

transportLogger.info("System launch"); // {"message":"System launch","level":"info"}
transportLogger.fatal("A critical failure!"); // {"message":"A critical failure!","level":"fatal"}

formatLogger.info("System launch"); // {"message":"System launch","level":"info"}
formatLogger.fatal("A critical failure!"); // {"message":"A critical failure!","level":"fatal"}

const ctx = {
  userId: "090121",
  productId: "creme-de-la-creme",
};

logger.child({ context: ctx }).info('Order "1234" was processed successfully');
// {"context":{"userId":"090121","productId":"creme-de-la-creme"},"message":"Order \"1234\" was processed successfully","level":"info","service":"billing-service","timestamp":"2021-07-29T12:20:13.249Z"}

logger.profile("test", { level: "info" });

const profiler = logger.startTimer();

setTimeout(() => {
  profiler.done({ message: "Done profiling" });
}, 1000);

