// #[cfg(feature = "std")]
pub mod std_logger {
    use std::sync::Mutex;
    #[derive(Clone, PartialEq, Debug, Default)]
    pub enum LoggingType {
        Error = -1,
        Mark = 0,
        #[default]
        Info = 1,
        Warning = 2
    }

    unsafe impl Send for LoggingType {}
    unsafe impl Sync for LoggingType {}

    #[derive(Clone, PartialEq, Debug, Default)]
    pub struct Log {
        pub info: Vec<&'static str>,
        pub info_id: Vec<i32>,
        pub info_type: Vec<LoggingType>,
    }

    impl Log {
        pub const fn new() -> Self {
            Self {
                info: Vec::new(),
                info_id: Vec::new(),
                info_type: Vec::new(),
            }
        }
    }

    unsafe impl Send for Log {}
    unsafe impl Sync for Log {}

    pub static LOGGER: Mutex<Log> = Mutex::new(Log::new());

    #[macro_export]
    macro_rules! mark {
        ($info:expr) => {
            use crate::std_logger::*;
            let mut logger_guard = LOGGER.lock().unwrap();
            logger_guard.info.push($info);
            logger_guard.info_id.push(0);
            logger_guard.info_type.push(LoggingType::Mark);
        };
    }
    #[macro_export]
    macro_rules! info {
        ($info:expr, $info_id:expr) => {
            let mut logger_guard = LOGGER.lock().unwrap();
            logger_guard.info.push($info);
            logger_guard.info_id.push($info_id);
            logger_guard.info_type.push(LoggingType::Info);
        };
    }
    #[macro_export]
    macro_rules! warning {
        ($info:expr, $info_id:expr) => {
            let mut logger_guard = LOGGER.lock().unwrap();
            logger_guard.info.push($info);
            logger_guard.info_id.push($info_id);
            logger_guard.info_type.push(LoggingType::Warning);
        };
    }
    #[macro_export]
    macro_rules! error {
        ($info:expr, $info_id:expr) => {
            let mut logger_guard = LOGGER.lock().unwrap();
            logger_guard.info.push($info);
            logger_guard.info_id.push($info_id);
            logger_guard.info_type.push(LoggingType::Error);
        };
    }

    #[macro_export]
    macro_rules! parse_log {
        () => {
            let logger_guard = LOGGER.lock().unwrap();
            let mut last_error = None;
            for i in logger_guard.info_id {
                match logger_guard.info_type[i] {
                    LoggingType::Mark => println!("[MARK]: Mark: {}", logger_guard.info[i]),
                    LoggingType::Info => println!("[INFO]: Info: {}; Info ID: {}",
                        logger_guard.info[i], logger_guard.info_id[i]
                    ),
                    LoggingType::Warning => eprintln!("[WARNING]: Warning: {}; Warning ID: {}",
                        logger_guard.info[i], logger_guard.info_id[i]
                    ),
                    LoggingType::Error => {
                        last_error = Some(i);
                        eprintln!("[ERROR]: Error: {}; Error ID: {}",
                            logger_guard.info[i], logger_guard.info_id[i]
                        );
                    }
                }
            } 
            if let Some(idx) = last_error {
                panic!("[ERROR]: Final Error: Error: {}; Error ID: {}",
                    logger_guard.info[i], logger_guard.info_id[i]
                )
            }
        };
    }

}

// #[cfg(feature = "no_std")]
pub mod logger {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn works() {
        mark!("Marker");
        info!("Info", 0);
        warning!("Warning", 0);
        error!("Error", 0);
        parse_log!();
    }
}