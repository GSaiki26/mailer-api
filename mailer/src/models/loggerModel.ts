// Libs
import { createLogger, format, Logger, transports } from "winston";

// Data
const { colorize, combine, printf, timestamp } = format;

// Class
class LoggerModel {
  public static get(owner: string): Logger {
    return createLogger({
      transports: [new transports.Console()],
      format: combine(
        colorize(),
        timestamp(),
        printf(
          (info) =>
            `[${info.timestamp}] (${info.level}) ${owner}#${process.pid} - ${info.message}`
        )
      ),
    });
  }
}

// Code
export default LoggerModel;
