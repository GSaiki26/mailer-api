// Libs
import mongoose from "mongoose";

// Class
class DatabaseModel {
  private static user = process.env.MONGO_INITDB_ROOT_USERNAME!;
  private static pass = process.env.MONGO_INITDB_ROOT_PASSWORD!;

  public static async connect(): Promise<void> {
    await mongoose.connect("mongodb://mailer-db", {
      user: this.user,
      pass: this.pass,
      autoCreate: true,
    });
  }
}

// Code
export default DatabaseModel;
