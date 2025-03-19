// validator.ts

/**
 * Error codes for validation failures
 */
export enum AppErrorCode {
  InvalidCpfLength = 0,
  InvalidCpf = 1,
  InvalidPlate = 2,
}

/**
 * Custom validation error with code and message
 */
export class ValidationError extends Error {
  constructor(
    public readonly code: AppErrorCode,
    message: string,
  ) {
    super(message);
    this.name = "ValidationError";
  }
}

/**
 * Validates a CPF (Brazilian individual taxpayer registry number)
 * @param cpf The CPF to validate
 * @throws {ValidationError} If the CPF is invalid
 */
export function validateCpf(cpf: string): void {
  const cleanCpf = cpf.replace(/[.-]/g, "");
  const cpfLen = cleanCpf.length;

  if (cpfLen !== 11) {
    throw new ValidationError(
      AppErrorCode.InvalidCpfLength,
      `CPF deve conter 11 dígitos (comprimento atual: ${cpfLen})`,
    );
  }

  // Check if all digits are the same
  if (/^(\d)\1+$/.test(cleanCpf)) {
    throw new ValidationError(
      AppErrorCode.InvalidCpf,
      `CPF inválido: ${formatCpf(cleanCpf)}`,
    );
  }

  // Convert to array of digits
  const digits = cleanCpf.split("").map((d) => Number.parseInt(d, 10));

  // Calculate first verification digit
  const dv1 = calculateDigit(digits.slice(0, 9), 10);

  // Calculate second verification digit
  const dv2 = calculateDigit([...digits.slice(0, 9), dv1], 11);

  if (dv1 !== digits[9] || dv2 !== digits[10]) {
    throw new ValidationError(
      AppErrorCode.InvalidCpf,
      `CPF inválido: ${cleanCpf}`,
    );
  }
}

/**
 * Helper function to calculate verification digits
 */
function calculateDigit(digits: number[], factor: number): number {
  const sum = digits.reduce((acc, digit, index) => {
    return acc + digit * (factor - index);
  }, 0);

  const remainder = (sum * 10) % 11;
  return remainder >= 10 ? 0 : remainder;
}

/**
 * Validates a Brazilian license plate
 * @param plate The plate to validate
 * @throws {ValidationError} If the plate is invalid
 */
export function validatePlate(plate: string): void {
  const pattern = /^[A-Z]{3}((\d[A-Z]\d{2})|(-?\d{4}))$/;

  if (!pattern.test(plate)) {
    throw new ValidationError(
      AppErrorCode.InvalidPlate,
      `Placa inválida: ${plate}`,
    );
  }
}

/**
 * Checks if a CPF is valid
 * @param cpf The CPF to validate
 * @returns true if the CPF is valid, false otherwise
 */
export function isCpfValid(cpf: string): boolean {
  try {
    validateCpf(cpf);
    return true;
  } catch (error) {
    return false;
  }
}

/**
 * Checks if a plate is valid
 * @param plate The plate to validate
 * @returns true if the plate is valid, false otherwise
 */
export function isPlateValid(plate: string): boolean {
  try {
    validatePlate(plate);
    return true;
  } catch (error) {
    return false;
  }
}

/**
 * Formats a CPF with dots and dash
 * @param cpf The CPF to format
 * @returns Formatted CPF string
 */
export function formatCpf(cpf: string): string {
  const cleanCpf = cpf.replace(/[.-]/g, "");
  return `${cleanCpf.slice(0, 3)}.${cleanCpf.slice(3, 6)}.${cleanCpf.slice(6, 9)}-${cleanCpf.slice(9, 11)}`;
}

// For Node.js compatibility
export default {
  AppErrorCode,
  ValidationError,
  validateCpf,
  validatePlate,
  isCpfValid,
  isPlateValid,
  formatCpf,
};
