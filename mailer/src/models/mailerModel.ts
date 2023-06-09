// Libs
import nodemailer from "nodemailer";
import { Attachment } from "nodemailer/lib/mailer";
import { Logger } from "winston";

// Types
import { SendMailReq } from "../proto/mailer_pb";

// Class
class MailerModel {
  private static user = process.env.MAILER_USER!;
  private static pass = process.env.MAILER_PASS!;
  private static service = process.env.MAILER_SERVICE!;
  private static transport = nodemailer.createTransport({
    service: this.service,
    auth: {
      user: this.user,
      pass: this.pass,
    },
  });

  /**
   * A method to send some mail.
   * @param logger - The logger param to track all events.
   * @param target - The target. Who'll receive the mail. Can be multiples receivers.
   * @param ccs - All people who'll receive a copy from the mail.
   * @param subject - the mail's subject.
   * @param body - The mail's body. Can be empty (In case of just sending a attach).
   * @param attachs - The attachs. Can be empty.
   * @returns - Returns a boolean with the information if the mail was sent.
   */
  public static async send(
    logger: Logger,
    request: SendMailReq,
    attachs: Attachment[]
  ): Promise<boolean> {
    // Create and configure the mail.
    const mail: nodemailer.SendMailOptions = {
      to: request.getTargetsList().join(", "),
      cc: request.getCcsList().join(", "),
      subject: request.getSubject(),
    };

    if (request.getBody()) mail.html = request.getBody();
    if (request.getAttachsList()) mail.attachments = attachs;

    // Try to send the mail.
    logger.info("Trying to send the mail.");
    try {
      await this.transport.sendMail(mail);
    } catch (err) {
      logger.error("Couldn't send the mail. " + err);
      return false;
    }

    logger.info("The mail has been succsessfully sended to the targets.");
    return true;
  }
}

// Code
export default MailerModel;
