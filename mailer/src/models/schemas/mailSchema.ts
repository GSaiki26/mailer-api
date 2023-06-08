// Libs
import { Schema, SchemaTypes } from "mongoose";

// Schemas
const mailSchema = new Schema({
  id: SchemaTypes.UUID,
  cc: SchemaTypes.Array,
  target: SchemaTypes.Array,
  body: SchemaTypes.String,
  attachs: SchemaTypes.Buffer,
});

// Code
export default mailSchema;
