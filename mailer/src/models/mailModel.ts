// Libs
import { Model } from "mongoose";
import DatabaseModel from "./databaseModel";

import mailSchema from "./schemas/mailSchema";

// Class
class MailModel {
  private static model: Model<typeof mailSchema>;

  /**
   * A method to define the model.
   */
  public static async defineModel(): Promise<void> {
    const conn = DatabaseModel.connect();
    this.model = conn.model("mails", mailSchema) as any;
    await conn.close();
  }
}

// Code
export default MailModel;
