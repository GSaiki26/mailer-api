// Libs
import { readFileSync } from "fs";

import * as grpc from "@grpc/grpc-js";

import LoggerModel from "./loggerModel";
import services from "../proto/mailer_grpc_pb";


// Class
class ServerModel {
  private static logger = LoggerModel.get("SERVER");
  private static server = new grpc.Server();
  private static creds = grpc.ServerCredentials.createSsl(
    readFileSync("./certs/ca.pem"),
    [{
      private_key: readFileSync("./certs/mailer.pem.key"),
      cert_chain: readFileSync("./certs/mailer.pem")
    }]
  );

  /**
   * A method to define the routes from the project.
   */
  public static defineRoutes(): void {
    this.server.addService(services.mailerServiceService as any, {
      sendMail: () => {}
    });
  }

  public static start(): void {
    this.server.bindAsync("0.0.0.0:3000", this.creds, (err, port) => {
      if (err) {
        this.logger.error("Couldn\'t start the server. " + err);
        return;
      }

      this.server.start();
      this.logger.info("Server started on port: " + port);
    });
  }
}

// Code
export default ServerModel;
