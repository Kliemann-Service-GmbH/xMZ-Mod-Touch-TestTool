error_chain!{
    foreign_links {
        Fmt(::std::fmt::Error);
        Gpio(::sysfs_gpio::Error) #[cfg(unix)];
        Io(::std::io::Error) #[cfg(unix)];
        SerdeJson(::serde_json::Error);
        Modbus(::libmodbus_rs::Error);
    }
}
