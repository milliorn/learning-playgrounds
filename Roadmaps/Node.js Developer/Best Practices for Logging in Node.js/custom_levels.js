const { createLogger, format, transports, log } = require('winston');
const winston = require('winston/lib/winston/config');

const logLevels = {
  fatal: 0,
  error: 1,
  warn: 2,
  info: 3,
  debug: 4,
  trace: 5,
};

const customLevels = {
  levels: logLevels,
  colors: {
    fatal: 'red',
    error: 'red',
    warn: 'yellow',
    info: 'green',
    debug: 'blue',
    trace: 'magenta',
  },
};

const logger = createLogger({
  levels: logLevels,
  transports: [ new transports.Console() ],
});

const customLogger = createLogger({
  levels: customLevels.levels,
  transports: [ new transports.Console() ],
});

const transportLogger = createLogger({
  level: "warn",
  levels: logLevels,
  transports: [ new transports.Console({ level: "info" }) ],
});

const formatLogger = createLogger({
  level: "info",
  levels: logLevels,
  transports: [ new transports.Console({ level: "info" }) ],
  format: format.combine(
    format.timestamp(),
    format.simple(),
    format.printf((info) => {
      return `${info.timestamp} ${info.level}: ${info.message}`;
    })
  ),
});

const fileLogger = createLogger({
  transports: [ new transports.File({ filename: "file.log" }) ],
  exceptionHandlers: [ new transports.File({ filename: "exceptions.log" }) ],
  rejectionHandlers: [ new transports.File({ filename: "rejections.log" }) ],
});


module.exports = { logger, customLogger, transportLogger, formatLogger, fileLogger };
