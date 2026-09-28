import type {
  CurrentIdentity,
  DashboardSummary,
  Health,
  OrganizationSummary,
  ServiceMeta,
} from './types'

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
  me: () => readJson<CurrentIdentity>('/api/v1/me'),
  organizations: () => readJson<OrganizationSummary[]>('/api/v1/organizations'),
  summary: (organizationId: string) =>
    readJson<DashboardSummary>(
      `/api/v1/organizations/${encodeURIComponent(organizationId)}/dashboard/summary`,
    ),
}
