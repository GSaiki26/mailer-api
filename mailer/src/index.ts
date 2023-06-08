// Libs
import cluster from "cluster";
import { cpus } from "os";

import ServerModel from "./models/serverModel";
import MigrationsModel from "./models/migrationsModel";
import LoggerModel from "./models/loggerModel";

// Data
const logger = LoggerModel.get("SERVER");

// Functions
async function main(): Promise<void> {
  if (cluster.isPrimary) {
    // Define the routes and run the migrations.
    await MigrationsModel.migrations();
    ServerModel.defineRoutes();

    // Fork the cluster if MULTI_PROCESSING is enabled.
    if (process.env.MULTI_CLUSTER! == "true") {
      for (let i = 0; i < cpus().length; i++) {
        cluster.fork();
      }
      return;
    }
  }

  // Start the server.
  ServerModel.start();
}

// Events
cluster.on("exit", (worker, code) => logger.info(`Cluster exited with code: ${code}`));

// Code
main();
