import { useEffect, useMemo, useState } from 'react'
import { controlPlaneApi } from './api'
import type {
  CurrentIdentity,
  DashboardSummary,
  OrganizationSummary,
  ServiceMeta,
} from './types'

type LoadState = 'loading' | 'ready' | 'error'

const emptySummary: DashboardSummary = {
  organizations: 0,
  repositories: 0,
  provider_connections: 0,
  active_lock_observations: 0,
  stale_lock_observations: 0,
  pending_force_unlock_requests: 0,
  active_policy_exceptions: 0,
  audit_events: 0,
}

function App() {
  const [summary, setSummary] = useState<DashboardSummary>(emptySummary)
  const [meta, setMeta] = useState<ServiceMeta | null>(null)
  const [identity, setIdentity] = useState<CurrentIdentity | null>(null)
  const [organizations, setOrganizations] = useState<OrganizationSummary[]>([])
  const [organizationId, setOrganizationId] = useState('')
  const [loadState, setLoadState] = useState<LoadState>('loading')
  const [message, setMessage] = useState('Connecting to the Control Plane…')

  async function loadOrganization(id: string) {
    if (!id) {
      setSummary(emptySummary)
      return
    }
    const nextSummary = await controlPlaneApi.summary(id)
    setSummary(nextSummary)
  }

  async function refresh() {
    setLoadState('loading')
    setMessage('Refreshing identity, provider and governance state…')
    try {
      const [nextMeta, health, nextIdentity, nextOrganizations] = await Promise.all([
        controlPlaneApi.meta(),
        controlPlaneApi.health(),
        controlPlaneApi.me(),
        controlPlaneApi.organizations(),
      ])
      setMeta(nextMeta)
      setIdentity(nextIdentity)
      setOrganizations(nextOrganizations)

      const selected =
        nextOrganizations.find((organization) => organization.id === organizationId)?.id ??
        nextOrganizations[0]?.id ??
        ''
      setOrganizationId(selected)
      await loadOrganization(selected)

      setMessage(
        `API ${health.status} · signed in as ${nextIdentity.display_name ?? nextIdentity.email ?? nextIdentity.subject}`,
      )
      setLoadState('ready')
    } catch (error) {
      setMessage(error instanceof Error ? error.message : 'Control Plane is unavailable')
      setLoadState('error')
    }
  }

  async function changeOrganization(id: string) {
    setOrganizationId(id)
    setLoadState('loading')
    try {
      await loadOrganization(id)
      setLoadState('ready')
    } catch (error) {
      setMessage(error instanceof Error ? error.message : 'Organization data is unavailable')
      setLoadState('error')
    }
  }

  useEffect(() => {
    void refresh()
  }, [])

  const cards = useMemo(
    () => [
      ['Repositories', summary.repositories, 'Enrolled and enabled'],
      ['Provider connections', summary.provider_connections, 'GitHub + Azure DevOps'],
      ['Verified locks', summary.active_lock_observations, 'Observed from provider Git LFS'],
      ['Stale locks', summary.stale_lock_observations, 'Needs owner/admin attention'],
      ['Force-unlock requests', summary.pending_force_unlock_requests, 'Pending approval'],
      ['Policy exceptions', summary.active_policy_exceptions, 'Active and time-bound'],
      ['Organizations', summary.organizations, 'Current tenant boundary'],
      ['Audit events', summary.audit_events, 'Durable governance history'],
    ] as const,
    [summary],
  )

  return (
    <div className="app-shell">
      <aside className="sidebar">
        <div className="brand">
          <div className="brand-mark">G</div>
          <div>
            <strong>GitGat</strong>
            <span>Control Plane</span>
          </div>
        </div>

        <nav className="nav" aria-label="Control Plane">
          <button className="nav-item active">Overview</button>
          <button className="nav-item" disabled>Repositories</button>
          <button className="nav-item" disabled>Active locks</button>
          <button className="nav-item" disabled>Policies</button>
          <button className="nav-item" disabled>Force unlock</button>
          <button className="nav-item" disabled>Audit log</button>
          <button className="nav-item" disabled>Provider health</button>
        </nav>

        <div className="sidebar-note">
          <span>Lock authority</span>
          <strong>Provider Git LFS</strong>
          <small>GitGat stores governance and verified observations—not a competing lock database.</small>
        </div>
      </aside>

      <main className="main">
        <header className="topbar">
          <div>
            <p className="eyebrow">Organization governance</p>
            <h1>Overview</h1>
          </div>
          <div className="topbar-actions">
            <select
              aria-label="Organization"
              value={organizationId}
              disabled={organizations.length === 0 || loadState === 'loading'}
              onChange={(event) => void changeOrganization(event.target.value)}
            >
              {organizations.length === 0 && <option value="">No organizations</option>}
              {organizations.map((organization) => (
                <option key={organization.id} value={organization.id}>
                  {organization.name} · {organization.role}
                </option>
              ))}
            </select>
            <button className="refresh" onClick={() => void refresh()} disabled={loadState === 'loading'}>
              {loadState === 'loading' ? 'Refreshing…' : 'Refresh'}
            </button>
          </div>
        </header>

        <section className={`status-banner ${loadState}`}>
          <span className="status-dot" />
          <span>{message}</span>
          {identity && <code>{identity.email ?? identity.subject}</code>}
          {meta && <code>v{meta.version}</code>}
        </section>

        <section className="metrics" aria-label="Governance summary">
          {cards.map(([label, value, hint]) => (
            <article className="metric-card" key={label}>
              <span>{label}</span>
              <strong>{value.toLocaleString()}</strong>
              <small>{hint}</small>
            </article>
          ))}
        </section>

        <section className="panel-grid">
          <article className="panel">
            <div className="panel-heading">
              <div>
                <p className="eyebrow">Enforcement</p>
                <h2>Safety gates</h2>
              </div>
              <span className="badge">RBAC foundation</span>
            </div>
            <div className="gate-list">
              <Gate name="OIDC/JWKS API authentication" state="In progress" />
              <Gate name="Organization-scoped RBAC" state="In progress" />
              <Gate name="Desktop Stage / Commit lock ownership" state="Implemented" />
              <Gate name="Git LFS strict Push verification" state="Implemented" />
              <Gate name="Provider PR lock-policy check" state="In progress" />
              <Gate name="Force-unlock execution" state="Blocked by design" />
            </div>
          </article>

          <article className="panel">
            <div className="panel-heading">
              <div>
                <p className="eyebrow">Operating model</p>
                <h2>Authority boundaries</h2>
              </div>
            </div>
            <div className="authority">
              <div><span>Git & refs</span><strong>Repository provider</strong></div>
              <div><span>Active file locks</span><strong>Git LFS server</strong></div>
              <div><span>Policies & approvals</span><strong>GitGat Control Plane</strong></div>
              <div><span>Audit & observations</span><strong>GitGat PostgreSQL</strong></div>
            </div>
          </article>
        </section>
      </main>
    </div>
  )
}

function Gate({ name, state }: { name: string; state: string }) {
  return (
    <div className="gate">
      <span>{name}</span>
      <strong>{state}</strong>
    </div>
  )
}

export default App
