import { invoke } from '@tauri-apps/api'

type SystemInfo = {
  total_memory: number
  used_memory: number
  total_swap: number
  used_swap: number
  name: string
  kernel_version: string
  os_version: string
  long_os_version: string
  host_name: string
  distribution_id: string
  cpus_len: number
  cpu_cpu_usage: string
  cpu_brand: string
  cpu_frequency: string
  cpu_vendor_id: string
  cpu_name: string
}

export async function sysinfo() {
  let sysinfo = await invoke<SystemInfo>('get_sysinfo')

  return sysinfo
}

export type MaxHashOption = {
  megabytes: number
  label: string
}

export function generateMaxHashOptions(systemMemory: number): MaxHashOption[] {
  const memoryLabels = new Map([
    [16, '16 MB'],
    [32, '32 MB'],
    [64, '64 MB'],
    [128, '128 MB'],
    [256, '256 MB'],
    [512, '512 MB'],
    [1024, '1 GB'],
    [2048, '2 GB'],
    [4096, '4 GB'],
    [8192, '8 GB'],
    [16384, '16 GB'],
    [32768, '32 GB'],
    [65536, '64 GB'],
    [131072, '128 GB'],
    [262144, '256 GB'],
    [524288, '512 GB'],
  ])

  let maxHashOptions: MaxHashOption[] = []

  for (let i = 16; i <= systemMemory * 0.7; i *= 2) {
    maxHashOptions.push({
      megabytes: i,
      label: memoryLabels.get(i) || `${i} MB`,
    })
  }

  return maxHashOptions
}

if (import.meta.vitest) {
  const { it, expect, vi } = import.meta.vitest

  it('generates hash options for a 4GB system', () => {
    expect(generateMaxHashOptions(4096)).toStrictEqual([
      { megabytes: 16, label: '16 MB' },
      { megabytes: 32, label: '32 MB' },
      { megabytes: 64, label: '64 MB' },
      { megabytes: 128, label: '128 MB' },
      { megabytes: 256, label: '256 MB' },
      { megabytes: 512, label: '512 MB' },
      { megabytes: 1024, label: '1 GB' },
      { megabytes: 2048, label: '2 GB' },
    ])
  })

  it('generates hash options for a 64GB system', () => {
    expect(generateMaxHashOptions(65536).length).toBe(12)
    expect(generateMaxHashOptions(65536).at(-1)).toStrictEqual({
      megabytes: 32768,
      label: '32 GB',
    })
  })
}
