// Libs
import * as grpc from "@grpc/grpc-js";
import { Attachment } from "nodemailer/lib/mailer";

import LoggerModel from "../models/loggerModel";
import MailerModel from "../models/mailerModel";

import messages, { Attach } from "../proto/mailer_pb";
import MailModel from "../models/mailModel";

// Class
class MailerService {
  /**
   * The service to send some mail.
   */
  public static async sendMail(
    call: grpc.ServerUnaryCall<messages.SendMailReq, any>,
    cb: grpc.sendUnaryData<messages.SendMailRes>
  ): Promise<void> {
    const logger = LoggerModel.get(call.getPeer());
    logger.info("Request on: " + call.getPath());

    // Check if the request has a target.
    logger.info("Checking if the request has a target...");
    if (!call.request.getTargetsList().length) {
      logger.warn("Any target found. Returning...");
      return cb({
        code: 3,
        message: "Any target found.",
      });
    }

    // Check if the request has a body or attach.
    const attachs = this.treatAttachs(call.request.getAttachsList());
    if (!call.request.getBody() && !attachs.length) {
      logger.warn(
        "The email has not a valid body and attachments. Returning..."
      );
      return cb({
        code: 3,
        message: "No body and attachments valid.",
      });
    }

    // Try to send and save the call into the database.
    const wasSended = await MailerModel.send(logger, call.request, attachs);
    await MailModel.saveMail(logger, call.request, attachs, wasSended);

    // Return the response to the caller.
    logger.info("Returning the response to the client...");
    const res = new messages.SendMailRes().setStatus(
      wasSended ? "Sended" : "Not sended"
    );
    cb(null, res);
  }

  /**
   * A method to treat the attachs from some email.
   * @param attachs - The attachs to be treated.
   */
  private static treatAttachs(attachs: Attach[]): Attachment[] {
    // Treat the attachments and return it.
    const treatedAttachs: Attachment[] = [];
    for (const attach of attachs) {
      treatedAttachs.push({
        filename: attach.getFilename(),
        content: Buffer.from(attach.getContent_asU8()),
      });
    }

    return treatedAttachs;
  }
}

// Code
export default MailerService;
