// Libs
import mongoose, { Mongoose } from "mongoose";

// Class
class DatabaseModel {
  private static user = process.env.MONGO_INITDB_ROOT_USERNAME!;
  private static pass = process.env.MONGO_INITDB_ROOT_PASSWORD!;

  public static connect(): Promise<Mongoose> {
    return mongoose.connect("mailer-db", {
      user: this.user,
      pass: this.pass,
      autoCreate: true
    });
  }
}


// Code
export default DatabaseModel;
