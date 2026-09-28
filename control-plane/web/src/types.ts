export type ServiceMeta = {
  service: string
  version: string
  api_version: string
  active_lock_authority: string
}

export type DashboardSummary = {
  organizations: number
  repositories: number
  provider_connections: number
  active_lock_observations: number
  stale_lock_observations: number
  pending_force_unlock_requests: number
  active_policy_exceptions: number
  audit_events: number
}

export type Health = {
  status: string
  service: string
  version: string
}
