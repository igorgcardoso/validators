use regex::Regex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum AppErrorCode {
    InvalidCpfLength,
    InvalidCpf,
    InvalidPlate,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct ValidationError {
    code: AppErrorCode,
    message: String,
}

#[wasm_bindgen]
impl ValidationError {
    pub fn code(&self) -> AppErrorCode {
        self.code
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }
}

#[wasm_bindgen(js_name = validateCpf)]
pub fn validate_cpf(cpf: &str) -> Result<(), ValidationError> {
    let cpf = cpf.replace(".", "").replace("-", "");
    let cpf_len = cpf.len();
    if cpf_len != 11 {
        return Err(ValidationError {
            code: AppErrorCode::InvalidCpfLength,
            message: format!(
                "CPF deve conter 11 dígitos (comprimento atual: {})",
                cpf_len
            ),
        });
    }

    if cpf
        .chars()
        .all(|c| c.is_ascii_digit() && c == cpf.chars().next().unwrap())
    {
        return Err(ValidationError {
            code: AppErrorCode::InvalidCpf,
            message: format!(
                "CPF inválido: {}.{}.{}-{}",
                &cpf[0..3],
                &cpf[3..6],
                &cpf[6..9],
                &cpf[9..11]
            ),
        });
    }

    let cpf = cpf
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let dv1 = {
        let result = cpf
            .iter()
            .take(9)
            .enumerate()
            .fold(0, |acc, (i, &n)| acc + n * (10 - i as u32))
            * 10
            % 11;

        if result >= 10 {
            0
        } else {
            result
        }
    };
    let dv2 = {
        let result = cpf
            .iter()
            .take(10)
            .enumerate()
            .fold(0, |acc, (i, &n)| acc + n * (11 - i as u32))
            * 10
            % 11;

        if result >= 10 {
            0
        } else {
            result
        }
    };
    if dv1 == cpf[9] && dv2 == cpf[10] {
        Ok(())
    } else {
        Err(ValidationError {
            code: AppErrorCode::InvalidCpf,
            message: format!(
                "CPF inválido: {}",
                cpf.iter().map(|&n| n.to_string()).collect::<String>()
            ),
        })
    }
}

#[wasm_bindgen(js_name = validatePlate)]
pub fn validate_plate(plate: &str) -> Result<(), ValidationError> {
    let pattern = Regex::new(r"^[A-Z]{3}((\d[A-Z]\d{2})|(-?\d{4}))$")
        .expect("Erro ao compilar a expressão regular");

    if pattern.is_match(plate) {
        Ok(())
    } else {
        Err(ValidationError {
            code: AppErrorCode::InvalidPlate,
            message: format!("Placa inválida: {}", plate),
        })
    }
}

#[wasm_bindgen(js_name = isCpfValid)]
pub fn is_cpf_valid(cpf: &str) -> bool {
    validate_cpf(cpf).is_ok()
}

#[wasm_bindgen(js_name = isPlateValid)]
pub fn is_plate_valid(plate: &str) -> bool {
    validate_plate(plate).is_ok()
}

#[wasm_bindgen(js_name = formatCpf)]
pub fn format_cpf(cpf: &str) -> String {
    let cpf = cpf.replace(".", "").replace("-", "");
    format!(
        "{}.{}.{}-{}",
        &cpf[0..3],
        &cpf[3..6],
        &cpf[6..9],
        &cpf[9..11]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_cpf() {
        assert!(validate_cpf("123.456.789-09").is_ok(), "123.456.789-09");
        assert!(validate_cpf("529.982.247-25").is_ok(), "529.982.247-25");
        assert!(validate_cpf("52998224725").is_ok(), "52998224725");
        assert!(validate_cpf("12345678909").is_ok(), "12345678909");
        assert!(validate_cpf("871.178.930-73").is_err(), "871.178.930-73");
        assert!(validate_cpf("529.982.247-26").is_err(), "529.982.247-26");
        assert!(validate_cpf("52998224726").is_err(), "52998224726");
        assert!(validate_cpf("529.982.247-2").is_err(), "529.982.247-2");
        assert!(validate_cpf("529.982.247-256").is_err(), "529.982.247-256");
        assert!(validate_cpf("529.982.247-256").is_err(), "529.982.247-256");
        assert!(validate_cpf("529.982.247-25a").is_err(), "529.982.247-25a");
        assert!(validate_cpf("111.111.111-11").is_err(), "111.111.111-11");
    }

    #[test]
    fn test_validate_plate() {
        assert!(validate_plate("ABC1234").is_ok(), "ABC1234");
        assert!(validate_plate("ABC-1234").is_ok(), "ABC-1234");
        assert!(validate_plate("ABC1A23").is_ok(), "ABC1A23");
        assert!(validate_plate("ABC-123").is_err(), "ABC-123");
        assert!(validate_plate("ABC-12345").is_err(), "ABC-12345");
        assert!(validate_plate("ABC-1234A").is_err(), "ABC-1234A");
    }

    #[test]
    fn test_is_cpf_valid() {
        assert!(is_cpf_valid("123.456.789-09"), "123.456.789-09");
        assert!(is_cpf_valid("529.982.247-25"), "529.982.247-25");
        assert!(is_cpf_valid("52998224725"), "52998224725");
        assert!(is_cpf_valid("12345678909"), "12345678909");
        assert!(!is_cpf_valid("871.178.930-73"), "871.178.930-73");
        assert!(!is_cpf_valid("529.982.247-26"), "529.982.247-26");
        assert!(!is_cpf_valid("52998224726"), "52998224726");
        assert!(!is_cpf_valid("529.982.247-2"), "529.982.247-2");
        assert!(!is_cpf_valid("529.982.247-256"), "529.982.247-256");
        assert!(!is_cpf_valid("529.982.247-256"), "529.982.247-256");
        assert!(!is_cpf_valid("529.982.247-25a"), "529.982.247-25a");
        assert!(!is_cpf_valid("111.111.111-11"), "111.111.111-11");
    }

    #[test]
    fn test_is_plate_valid() {
        assert!(is_plate_valid("ABC1234"), "ABC1234");
        assert!(is_plate_valid("ABC-1234"), "ABC-1234");
        assert!(is_plate_valid("ABC1A23"), "ABC1A23");
        assert!(!is_plate_valid("ABC-123"), "ABC-123");
        assert!(!is_plate_valid("ABC-12345"), "ABC-12345");
        assert!(!is_plate_valid("ABC-1234A"), "ABC-1234A");
    }

    #[test]
    fn test_format_cpf() {
        assert_eq!(format_cpf("12345678909"), "123.456.789-09", "12345678909");
        assert_eq!(
            format_cpf("123.456.789-09"),
            "123.456.789-09",
            "123.456.789-09"
        );
        assert_eq!(format_cpf("52998224725"), "529.982.247-25", "52998224725");
        assert_eq!(
            format_cpf("529.982.247-25"),
            "529.982.247-25",
            "529.982.247-25"
        );
    }
}
