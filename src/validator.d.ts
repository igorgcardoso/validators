export enum AppErrorCode {
  InvalidCpfLength = "InvalidCpfLength",
  InvalidCpf = "InvalidCpf",
  InvalidPlate = "InvalidPlate",
}

export class ValidatorError {
  readonly code: AppErrorCode;
  readonly message: string;
}

export function validateCpf(cpf: string): void;
export function validatePlate(plate: string): void;
export function formatCpf(cpf: string): string;
export function isValidCpf(cpf: string): boolean;
export function isValidPlate(plate: string): boolean;
