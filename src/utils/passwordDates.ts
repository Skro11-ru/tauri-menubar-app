const HOUR_MS = 60 * 60 * 1000;
const DAY_MS = 24 * HOUR_MS;
const WEEK_MS = 7 * DAY_MS;
const MONTH_MS = 30 * DAY_MS;
const RECENT_WINDOW_MS = 3 * DAY_MS;

function elapsedMs(updatedAt: string, now = new Date()): number | null {
  const updatedAtMs = new Date(updatedAt).getTime();

  if (Number.isNaN(updatedAtMs)) {
    return null;
  }

  return Math.max(0, now.getTime() - updatedAtMs);
}

export function isRecentlyUpdated(updatedAt: string, now = new Date()): boolean {
  const elapsed = elapsedMs(updatedAt, now);

  return elapsed !== null && elapsed <= RECENT_WINDOW_MS;
}

export function formatUpdatedAt(updatedAt: string, now = new Date()): string {
  const elapsed = elapsedMs(updatedAt, now);

  if (elapsed === null) {
    return "Unknown";
  }

  if (elapsed < DAY_MS) {
    return `${Math.max(1, Math.floor(elapsed / HOUR_MS))}h ago`;
  }

  if (elapsed < WEEK_MS) {
    return `${Math.floor(elapsed / DAY_MS)}d ago`;
  }

  if (elapsed < MONTH_MS) {
    return `${Math.floor(elapsed / WEEK_MS)}w ago`;
  }

  return `${Math.floor(elapsed / MONTH_MS)}mo ago`;
}
