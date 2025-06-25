use crate::std_logger::*;
// #[cfg(feature = "std")]
pub mod std_logger {
    use std::sync::Mutex;
    #[repr(i32)]
    #[derive(Clone, PartialEq, Debug, Default)]
    pub enum LoggingType {
        Error = -1,
        Mark = 0,
        #[default]
        Info = 1,
        Warning = 2
    }

    #[derive(Clone, PartialEq, Debug, Default)]
    pub struct Log {
        pub info: Vec<String>,
        pub info_id: Vec<i32>,
        pub info_type: Vec<LoggingType>,
    }

    impl Log {
        pub const fn new() -> Self {
            Self {
                info: vec![],
                info_id: vec![],
                info_type: vec![],
            }
        }
    }

    pub static LOGGER: Mutex<Log> = Mutex::new(Log::new());

    #[macro_export]
    macro_rules! mark {
        ($info:expr) => {{
            
            let mut logger_guard = LOGGER.lock().unwrap();
            logger_guard.info.push($info);
            logger_guard.info_id.push(0);
            logger_guard.info_type.push(LoggingType::Mark);
        }};
    }
    #[macro_export]
    macro_rules! info {
        ($info:expr, $info_id:expr) => {{
            let mut logger_guard = LOGGER.lock().unwrap();
            logger_guard.info.push($info);
            logger_guard.info_id.push($info_id);
            logger_guard.info_type.push(LoggingType::Info);
        }};
    }
    #[macro_export]
    macro_rules! warning {
        ($info:expr, $info_id:expr) => {{
            let mut logger_guard = LOGGER.lock().unwrap();
            logger_guard.info.push($info);
            logger_guard.info_id.push($info_id);
            logger_guard.info_type.push(LoggingType::Warning);
        }};
    }
    #[macro_export]
    macro_rules! error {
        ($info:expr, $info_id:expr) => {{
            let mut logger_guard = LOGGER.lock().unwrap();
            logger_guard.info.push($info);
            logger_guard.info_id.push($info_id);
            logger_guard.info_type.push(LoggingType::Error);
        }};
    }

    #[macro_export]
    macro_rules! parse_log {
        () => {{
            let logger_guard = LOGGER.lock().unwrap();
            let mut last_error: Option<usize> = None;
            for (idx, &id) in logger_guard.info_id.iter().enumerate() {
                match logger_guard.info_type[idx] {
                    LoggingType::Mark =>  {
                        println!("[MARK]: Mark: {}", logger_guard.info[idx]);
                    }
                    LoggingType::Info => {
                        println!("[INFO]: Info: {}; Info ID: {}",
                            logger_guard.info[idx], id
                        );
                    },
                    LoggingType::Warning => {
                        eprintln!("[WARNING]: Warning: {}; Warning ID: {}",
                            logger_guard.info[idx], id
                        );
                    },
                    LoggingType::Error => {
                        last_error = Some(idx as usize);
                        eprintln!("[ERROR]: Error: {}; Error ID: {}",
                            logger_guard.info[idx], id
                        );
                    }
                }
            } 
            if let Some(idx) = last_error {
                panic!("[ERROR]: Final Error: Error: {}; Error ID: {}",
                    logger_guard.info[idx], logger_guard.info_id[idx]
                )
            }
        }};
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
        mark!("Marker".to_string());
        info!("Info".to_string(), 0);
        warning!("Warning".to_string(), 0);
        error!("Error".to_string(), 0);
        parse_log!();
    }
}