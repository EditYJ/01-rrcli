mod cli;
mod process;
mod utils;

pub use cli::{
    Base64FormatType, Cli, CmdExecutor, CsvFormatType, RCliCommand, TextSignFormatType,
    TextSignOption,
};
pub use process::{
    convert_csv_in_file, gen_pass, generate_key, http_serve, sign_text, verify_text,
};
pub use utils::{get_string_from_path, save_str_in_file, verify_dir};
