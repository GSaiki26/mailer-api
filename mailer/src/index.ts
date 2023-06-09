// Libs
import cluster from "cluster";
import { cpus } from "os";

import DatabaseModel from "./models/databaseModel";
import LoggerModel from "./models/loggerModel";
import ServerModel from "./models/serverModel";
import mongoose from "mongoose";

// Data
const logger = LoggerModel.get("SERVER");

// Functions
async function main(): Promise<void> {
  await DatabaseModel.connect();

  if (cluster.isPrimary) {
    // Fork the cluster if MULTI_PROCESSING is enabled.
    if (process.env.MULTI_CLUSTER! == "true") {
      for (let i = 0; i < cpus().length; i++) {
        cluster.fork();
      }
      return;
    }
  }

  // Start the server.
  ServerModel.defineRoutes();
  ServerModel.start();
}

// Events
cluster.on("exit", async (worker, code) => {
  console.warn(`Cluster#${worker.process.pid} exited with code: ${code}`);
  await mongoose.connection.close();
});

// Code
main();
