use std::collections::HashMap;

use anyhow::Result;
use csv::Reader;
use serde_json::Value;

use crate::{save_str_in_file, CsvFormatType};

fn convert_csv(input_path: String, format_type: CsvFormatType) -> Result<String> {
    let mut result = Vec::with_capacity(128);
    let mut toml_result = HashMap::new();

    let mut input_file = Reader::from_path(input_path)?;
    let headers = input_file.headers()?.clone();

    for row in input_file.records() {
        let row_data = row?;
        let json_value = headers.iter().zip(row_data.iter()).collect::<Value>();
        result.push(json_value.clone());
    }
    toml_result.insert("players", &result);

    let res_str = match format_type {
        CsvFormatType::Json => serde_json::to_string_pretty(&result)?,
        CsvFormatType::Yaml => serde_yaml::to_string(&result)?,
        CsvFormatType::Toml => toml::to_string_pretty(&toml_result)?,
    };

    Ok(res_str)
}

// 转换csv文件至其他格式的文件
pub fn convert_csv_in_file(
    input_path: String,
    save_path: String,
    format_type: CsvFormatType,
) -> Result<()> {
    let res_str = convert_csv(input_path, format_type)?;
    save_str_in_file(save_path, res_str)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    // 测试转换csv文件至json格式的文件
    #[test]
    fn test_convert_csv_json() -> Result<()> {
        // 准备测试数据
        let input_path = "./fixtures/process_csv/test.csv".to_string();
        let expected_result = fs::read_to_string("./fixtures/process_csv/test.json")?;

        // 调用函数进行转换
        let result = convert_csv(input_path, CsvFormatType::Json)?;

        // 验证转换结果是否符合预期
        assert_eq!(format!("{}\n", result), expected_result);

        Ok(())
    }

    // 测试转换csv文件至yaml格式的文件
    #[test]
    fn test_convert_csv_yaml() -> Result<()> {
        // 准备测试数据
        let input_path = "./fixtures/process_csv/test.csv".to_string();
        let expected_result = fs::read_to_string("./fixtures/process_csv/test.yaml")?;

        // 调用函数进行转换
        let result = convert_csv(input_path, CsvFormatType::Yaml)?;

        // 验证转换结果是否符合预期
        assert_eq!(format!("{}", result), expected_result);
        Ok(())
    }

    // 测试转换csv文件至toml格式的文件
    #[test]
    fn test_convert_csv_toml() -> Result<()> {
        // 准备测试数据
        let input_path = "./fixtures/process_csv/test.csv".to_string();
        let expected_result = fs::read_to_string("./fixtures/process_csv/test.toml")?;

        // 调用函数进行转换
        let result = convert_csv(input_path, CsvFormatType::Toml)?;

        // 验证转换结果是否符合预期
        assert_eq!(format!("{}", result), expected_result);

        Ok(())
    }
}
