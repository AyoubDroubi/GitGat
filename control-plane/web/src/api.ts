import type { DashboardSummary, Health, ServiceMeta } from './types'

async function readJson<T>(path: string): Promise<T> {
  const response = await fetch(path, {
    headers: { Accept: 'application/json' },
    credentials: 'same-origin',
  })

  if (!response.ok) {
    throw new Error(`${path} returned HTTP ${response.status}`)
  }

  return response.json() as Promise<T>
}

export const controlPlaneApi = {
  meta: () => readJson<ServiceMeta>('/api/v1/meta'),
  health: () => readJson<Health>('/health/ready'),
  summary: () => readJson<DashboardSummary>('/api/v1/dashboard/summary'),
}
