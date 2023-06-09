// Libs
import { Schema, SchemaTypes } from "mongoose";

// Schemas
const mailSchema = new Schema({
  ccsList: {
    type: [SchemaTypes.String],
    required: true,
  },
  targetsList: {
    type: [SchemaTypes.String],
    required: true,
  },
  subject: {
    type: SchemaTypes.String,
    required: true,
  },
  body: {
    type: SchemaTypes.String,
    required: true,
  },
  attachsList: {
    type: [
      {
        type: SchemaTypes.Map,
        of: {
          content: SchemaTypes.Buffer,
          filename: SchemaTypes.String,
        },
      },
    ],
    required: true,
  },
  sended: {
    type: SchemaTypes.Boolean,
    required: true,
  },
});

// Code
export default mailSchema;
