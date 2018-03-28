use serde_json::Error as SerdeError;
use std::error::Error;
use std::fmt;
use std::fmt::Error as FmtError;
use std::io::Error as IoError;
use sysfs_gpio::Error as GpioError;


#[derive(Debug)]
pub enum TestToolError {
    GpioError(GpioError),
}

impl fmt::Display for TestToolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestToolError::GpioError(ref err) => write!(f, "GPIO Error: {}", err),
        }
    }
}

impl Error for TestToolError {
    fn description(&self) -> &str {
        match *self {
            TestToolError::GpioError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            TestToolError::GpioError(ref err) => Some(err),
        }
    }
}


impl From<GpioError> for TestToolError {
    fn from(err: GpioError) -> TestToolError {
        TestToolError::GpioError(err)
    }
}
