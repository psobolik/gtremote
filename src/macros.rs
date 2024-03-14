/*
 * Copyright (c) 2024 Paul Sobolik
 * Created 2024-03-15
 */

#[macro_export]
macro_rules! print_info {
        ($($arg:tt)*) => {{
            let message = format!($($arg)*);
            println!("ℹ️ {message}")
        }};
    }

#[macro_export]
macro_rules! print_success {
        ($($arg:tt)*) => {{
            let message = format!($($arg)*);
            println!("✔️ {message}")
        }};
    }

#[macro_export]
macro_rules! print_error {
        ($($arg:tt)*) => {{
            let message = format!($($arg)*);
            eprintln!("💥 {message}")
        }};
    }