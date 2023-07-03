export type UciOption = { option: string; value: string }

export interface EngineListing {
  name: string
  description: string
  website: string
  icon: string
  license: string
  version: string
  updated_at: string
  uci_options: UciOption[]
  binaries: {
    os: string
    architecture: string
    zip: string
    binary_filename: string
  }
}
