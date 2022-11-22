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
