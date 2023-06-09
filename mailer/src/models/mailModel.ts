// Libs
import { Document, model } from "mongoose";
import { Attachment } from "nodemailer/lib/mailer";
import { Logger } from "winston";

import mailSchema from "./schemas/mailSchema";
import { SendMailReq } from "../proto/mailer_pb";

// Class
class MailModel {
  private static model = model("mails", mailSchema);

  /**
   * A method to save some mail into the database.
   *
   */
  public static async saveMail(
    logger: Logger,
    request: SendMailReq,
    attachs: Attachment[],
    wasSended: boolean
  ): Promise<Document | void> {
    logger.info("Trying to save the mail in the database...");
    const entry = new this.model({
      ccsList: request.getCcsList(),
      targetsList: request.getTargetsList(),
      subject: request.getSubject(),
      body: request.getBody(),
      attachsList: attachs,
      sended: wasSended,
    });

    try {
      await entry.save();
      logger.info("The mail was successfully saved into the database.");
      return entry;
    } catch (err) {
      logger.error("Couldn't save into the database. " + err);
      return;
    }
  }
}

// Code
export default MailModel;
