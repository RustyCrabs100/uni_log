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
            return;
        };
    }
    #[macro_export]
    macro_rules! info {
        ($info:expr, $info_id:expr) => {
            let mut logger_guard = LOGGER.lock().unwrap();
            logger_guard.info.push($info);
            logger_guard.info_id.push($info_id);
            logger_guard.info_type.push(LoggingType::Info);
            return;
        };
    }
    #[macro_export]
    macro_rules! warning {
        ($info:expr, $info_id:expr) => {
            let mut logger_guard = LOGGER.lock().unwrap();
            logger_guard.info.push($info);
            logger_guard.info_id.push($info_id);
            logger_guard.info_type.push(LoggingType::Warning);
            return;
        };
    }
    #[macro_export]
    macro_rules! error {
        ($info:expr, $info_id:expr) => {
            let mut logger_guard = LOGGER.lock().unwrap();
            logger_guard.info.push($info);
            logger_guard.info_id.push($info_id);
            logger_guard.info_type.push(LoggingType::Error);
            return;
        };
    }

    #[macro_export]
    macro_rules! parse_log {
        () => {
            let logger_guard = LOGGER.lock().unwrap();
            let mut counter: usize = 0;
            let mut last_error: Option<usize> = None;
            for i in logger_guard.info_id.clone() {
                match logger_guard.info_type[counter] {
                    LoggingType::Mark =>  {
                        println!("[MARK]: Mark: {}", logger_guard.info[counter]);
                        counter += 1; 
                    }
                    LoggingType::Info => {
                        println!("[INFO]: Info: {}; Info ID: {}",
                            logger_guard.info[counter], logger_guard.info_id[counter]
                        );
                        counter+=1;
                    },
                    LoggingType::Warning => {
                        eprintln!("[WARNING]: Warning: {}; Warning ID: {}",
                            logger_guard.info[counter], logger_guard.info_id[counter]
                        );
                        counter+=1;
                    },
                    LoggingType::Error => {
                        last_error = Some(i as usize);
                        eprintln!("[ERROR]: Error: {}; Error ID: {}",
                            logger_guard.info[counter], logger_guard.info_id[counter]
                        );
                        counter+=1;
                    }
                }
            } 
            if let Some(idx) = last_error {
                panic!("[ERROR]: Final Error: Error: {}; Error ID: {}",
                    logger_guard.info[idx], logger_guard.info_id[idx]
                )
            }
            return;
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