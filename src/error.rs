use colored::*;

// Error types
pub enum Error {
    InvalidEndpoint,
    InvalidUrl,
    InvalidRelativePath,
    InvalidConfigFile,
    ServerResponseParseError,
    ConnectionsUnretrieveable,
    ConnectionDoesNotExist,
    CannotCreateInvitation,
    CannotCreateQrCode,
    InvalidInvitationConfiguration,
}

// Error handler (Should not panic but print a custom error and exit)
pub fn throw(error: Error) -> ! {
    match error {
        // The path used for the configuration file is incorrect
        Error::InvalidRelativePath => log("Invalid configuration file path (it MUST be relative)"),
        // The configuration file does not have the correct fields
        Error::InvalidConfigFile => log("Invalid Configuration file structure"),
        // The endpoint in the configuration file is invalid
        Error::InvalidEndpoint => log("Invalid Endpoint"),
        // The url created from the base + endpoint is invalid
        Error::InvalidUrl => log("Invalid Url"),
        // Could not parse the response from the server
        Error::ServerResponseParseError => log("Unable to parse response from server"),
        // The connection does not exist on the agent
        Error::ConnectionDoesNotExist => log("Connection does not exist"),
        // The connection list is unretrieveable (Could this even happen?)
        Error::ConnectionsUnretrieveable => log("Connection is unretrieveable"),
        // Could not create an invitation
        Error::CannotCreateInvitation => log("Could not create an invitation"),
        // Could not create a qr code for the invitation
        Error::CannotCreateQrCode => log("Could not create a qrcode for the invitation"),
        // The json configuration is missing the invitation configuration which is required if the
        // connection id is not specified
        Error::InvalidInvitationConfiguration => log("Connection invitation config is invalid"),
    }
}

fn log(string: &str) -> ! {
    eprintln!("{}: {}", "Error".red(), String::from(string));
    std::process::exit(1)
}