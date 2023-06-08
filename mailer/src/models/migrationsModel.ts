// Libs
import * as grpc from "@grpc/grpc-js";

import MailModel from "./mailModel";

// Class
class MigrationsModel {
  /**
   * A method to create/define the mongoose models on the classes.
   */
  public static async migrations(): Promise<void> {
    await MailModel.defineModel();
  }
}

// Code
export default MigrationsModel;
