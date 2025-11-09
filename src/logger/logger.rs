use crate::errors::error::CartError;

pub enum Logger {
    LogOk(String),
    LogErr(String),
    LogWarn(String),
    LogAbort(i32),
}

impl Logger {
    pub fn log(&self) -> Result<(), CartError> {
        match self {
            Logger::LogOk(msg) => {
                println!("[OK]: {}", msg);
                Ok(())
            }
            Logger::LogErr(msg) => {
                eprintln!("[ERROR]: {}", msg);
                Ok(())
            }
            Logger::LogWarn(msg) => {
                eprintln!("[WARN]: {}", msg);
                Ok(())
            }
            Logger::LogAbort(code) => {
                eprintln!("[ABORT]: left with code {}", code);
                std::process::exit(*code);
            }
        }
    }

    pub fn log_option(option: &str, message: &str) -> Result<(), CartError> {
        match option.to_lowercase().as_str() {
            "ok" => Logger::LogOk(message.to_string()).log(),
            "error" | "err" => Logger::LogErr(message.to_string()).log(),
            "warn" => Logger::LogWarn(message.to_string()).log(),
            "abort" => {
                eprintln!("[ABORT]: {}", message);
                std::process::exit(1);
            }
            _ => Err(CartError::Exception(format!("Whats that log? : '{}'", option))),
        }
    }

    pub fn log_enum(log: Logger) -> Result<(), CartError> {
        log.log()
    }
}
