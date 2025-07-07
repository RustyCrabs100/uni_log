// ! uni_log, a logger for all your needs!
// !
// ! The way this crate works is by storing all your logging attempts, only logging once the parse_log!() macro has been called.
// ! There are multiple methods of which you can choose. Here are said methods:
// ! Basic - Bare Minimum Logging.
// ! Async - Same thing as Basic, but this time Async.
// ! Full - A logging implementation with timing, settings, file writing, colored messages, etc.
// ! Full Async - Same thing as Full, but this time Async.
// ! no_std - Same thing as Basic, but with minimal memory footprint, as well as a much smaller runtime cost.

/// The main container of all the logging methods
pub mod log {
    /// An enum of all the different logging types.
    /// All types have their own i32 variants
    #[repr(i32)]
    #[derive(Default, PartialEq, Clone, Debug)]
    pub enum LoggingType {
        /// The Error logging type, used for an UNRECOVERABLE error in your program.
        /// This type is equal to -1
        Error = -1,
        /// The Warning logging type, used for when a potentially hazardous event happens in your program.
        /// This type is equal to 0
        Warning = 0,
        /// The Info logging type, the default type, used for basic logging of an event in your program.
        /// This type is equal to 1
        #[default]
        Info = 1,
        /// The Mark logging type, for logging when an event happens in your program.
        /// This type is equal to 2
        Mark = 2,
    }

    /// A logging implementation including only the bare minimum
    /// This implementation includes:
    /// Logging Errors, Warnings, basic messages, or simply when an event happens.
    #[cfg(feature = "basic")]
    pub mod basic_log {
        use std::sync::Mutex;

        /// The struct containing the basic logger.
        #[derive(Default, PartialEq, Clone, Debug)]
        pub struct Logger {
            /// The data of a log
            pub info: String,
            /// The id of a log
            pub id: i32,
            /// What kind of log is this?
            /// See LoggingType
            pub logging_type: crate::log::LoggingType,
        }

        impl Logger {
            pub const fn new(info: String, id: i32, logging_type: crate::log::LoggingType) -> Self {
                Self {
                    info,
                    id,
                    logging_type,
                }
            }
        }

        /// The static variable containing a Mutex of a Vector of Logger's
        pub static LOGGER_REGISTRY: Mutex<Vec<Logger>> = Mutex::new(Vec::new());

        #[macro_export]
        macro_rules! error {
            ($name:expr, $id:expr) => {
                let mut logger_registry = crate::log::basic_log::LOGGER_REGISTRY.lock().unwrap();
                logger_registry.push(crate::log::basic_log::Logger::new(
                    $name,
                    $id,
                    crate::log::LoggingType::Error,
                ));
            };
        }

        #[macro_export]
        macro_rules! warn {
            ($name:expr, $id:expr) => {
                let mut logger_registry = crate::log::basic_log::LOGGER_REGISTRY.lock().unwrap();
                logger_registry.push(crate::log::basic_log::Logger::new(
                    $name,
                    $id,
                    crate::log::LoggingType::Warning,
                ));
            };
        }

        #[macro_export]
        macro_rules! info {
            ($name:expr, $id:expr) => {
                let mut logger_registry = crate::log::basic_log::LOGGER_REGISTRY.lock().unwrap();
                logger_registry.push(crate::log::basic_log::Logger::new(
                    $name,
                    $id,
                    crate::log::LoggingType::Info,
                ));
            };
        }

        #[macro_export]
        macro_rules! mark {
            ($name:expr, $id:expr) => {
                let mut logger_registry = crate::log::basic_log::LOGGER_REGISTRY.lock().unwrap();
                logger_registry.push(crate::log::basic_log::Logger::new(
                    $name,
                    $id,
                    crate::log::LoggingType::Mark,
                ));
            };
        }
        #[macro_export]
        macro_rules! parse_log {
            () => {
                use crate::log::LoggingType as LT;
                let mut last_error: Option<&crate::log::basic_log::Logger> = None;

                let binding = crate::log::basic_log::LOGGER_REGISTRY
                    .lock()
                    .unwrap()
                    .clone();

                for items in &*binding {
                    match items.logging_type {
                        LT::Mark => println!("[MARK]: Info: {}; ID: {}", items.info, items.id),
                        LT::Info => println!("[INFO]: Info: {}; ID: {}", items.info, items.id),
                        LT::Warning => {
                            eprintln!("[WARNING]: Info: {}; ID: {}", items.info, items.id)
                        }
                        LT::Error => {
                            last_error = Some(items);
                            eprintln!("[ERROR]: Info: {}; ID: {}", items.info, items.id);
                        }
                    }
                }

                match last_error {
                    Some(logger) => {
                        panic!("[FINAL ERROR]: Info: {}; ID: {}", logger.info, logger.id);
                    }
                    None => {
                        println!("Logging Attempt Completed!");
                    }
                }
            };
        }
    }

    /// A logging implementation full of features, such as
    /// - Timing of calls
    /// - File writing
    /// - Log settings
    /// and more!
    /// NOTE:
    /// This implementation is not async.
    /// If your looking for an async logger, look at async_log
    /// If you need a feature-full async logger, look at full_async_log
    #[cfg(feature = "full")]
    pub mod full_log {}

    /// An async implementation of the basic feature.
    /// 
    #[cfg(feature = "async")]
    pub mod async_log {}

    #[cfg(feature = "full_async")]
    pub mod full_async_log {}

    #[cfg(feature = "no-std")]
    pub mod no_std_log {}
}

#[cfg(test)]
#[cfg(feature = "basic")]
mod basic_tests {
    use super::*;
    #[test]
    #[should_panic]
    fn basic_logging_works() {
        mark!("AAAAAAAAAA".to_string(), 0);
        info!("AAAAAAAAAAAAAA".to_string(), 0);
        warn!("AAAAAAAAAAAAAAAAAA".to_string(), 0);
        error!("AAAAAAAAAAAAAAAAAAAA".to_string(), 0);
        parse_log!();
    }
}

#[cfg(test)]
#[cfg(feature = "async")]
mod async_tests {}

#[cfg(test)]
#[cfg(feature = "full")]
mod full_tests {}

#[cfg(test)]
#[cfg(feature = "full_async")]
mod full_async_tests {}

#[cfg(test)]
#[cfg(feature = "no-std")]
mod no_std_tests {}