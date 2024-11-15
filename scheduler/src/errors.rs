use lettre::transport::smtp;
use models::errors::ModelError;

pub enum SendMailError {
    MessageCreationError(ModelError),
    TransportError(smtp::Error),
}

impl From<ModelError> for SendMailError {
    fn from(e: ModelError) -> Self {
        SendMailError::MessageCreationError(e)
    }
}

impl From<smtp::Error> for SendMailError {
    fn from(e: smtp::Error) -> Self {
        SendMailError::TransportError(e)
    }
}
