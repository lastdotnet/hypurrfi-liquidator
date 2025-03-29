// Define a bunch of simple wrapper macros allowing
// for disabling info and debug log levels
// at compile time
#[cfg(not(feature = "no-log"))]
macro_rules! info {
    ($($arg: tt)*) => { tracing::info!($($arg)*) }
}

#[cfg(feature = "no-log")]
macro_rules! info {
    ($($arg: tt)*) => {
        ()
    };
}

#[cfg(not(feature = "no-log"))]
macro_rules! debug {
    ($($arg: tt)*) => { tracing::debug!($($arg)*) }
}

#[cfg(feature = "no-log")]
macro_rules! debug {
    ($($arg: tt)*) => { 
        () 
    }
}

pub(crate) use debug;
pub(crate) use info;