// Available liquidity, read from analytics.datamine.network's dashboard page: the latest point of
// each token's series, in dollars. The request is same-origin (/api), proxied to the analytics
// host, because that host answers no cross-origin reads. This is the one thing the page fetches
// beyond its own files; the reader itself never calls out.
import { create } from 'zustand';

export interface TokenLiquidity {
  token: string;
  dollars: number;
  at: number;
}

export interface Milestone {
  target: number;
  reached: boolean;
}

interface LiquidityState {
  tokens: TokenLiquidity[];
  total: number;
  loadedAt: number;
  failure: string;
  load: () => Promise<void>;
}

interface Series {
  seriesName: string;
  points: [number, number][];
}

interface Dashboard {
  widgets: { liquidityChart: { series: Series[] } };
}

const dashboardPath = '/api/page/dashboard';

export const dollars = new Intl.NumberFormat('en-US', { style: 'currency', currency: 'USD', maximumFractionDigits: 0 });

// The campaign's milestones, in dollars, in the order they are reached.
export const milestoneTargets = [
  300_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000, 10_000_000_000, 100_000_000_000, 1_000_000_000_000,
] as const;

export function milestones(total: number): Milestone[] {
  return milestoneTargets.map((target) => ({ target, reached: total >= target }));
}

// The milestone the campaign is working toward: the first not yet reached, or the last one.
export function currentMilestone(total: number): Milestone {
  const listed = milestones(total);
  return listed.find((milestone) => !milestone.reached) ?? listed[listed.length - 1];
}

// The share of the current milestone covered, from the previous milestone up, so a fresh
// milestone starts near empty rather than most of the way there.
export function progress(total: number): number {
  const listed = milestones(total);
  const at = listed.findIndex((milestone) => !milestone.reached);
  if (at < 0) return 1;
  const floor = at === 0 ? 0 : listed[at - 1].target;
  return Math.max(0, Math.min(1, (total - floor) / (listed[at].target - floor)));
}

// The payload is checked field by field before any of it is shown: a wrong shape is a failure,
// never a NaN on screen.
function latest(series: unknown): TokenLiquidity | null {
  if (typeof series !== 'object' || series === null) return null;
  const { seriesName, points } = series as Partial<Series>;
  if (typeof seriesName !== 'string' || seriesName === '' || !Array.isArray(points)) return null;
  const last = points[points.length - 1];
  if (!Array.isArray(last) || last.length < 2) return null;
  const [at, dollars] = last;
  if (typeof at !== 'number' || typeof dollars !== 'number' || !Number.isFinite(dollars) || dollars < 0) return null;
  return { token: seriesName, dollars, at };
}

export const useLiquidityStore = create<LiquidityState>()((set) => ({
  tokens: [],
  total: 0,
  loadedAt: 0,
  failure: '',
  load: async () => {
    try {
      const response = await fetch(dashboardPath, { credentials: 'omit', referrerPolicy: 'no-referrer' });
      if (!response.ok) throw new Error(`${dashboardPath}: ${response.status}`);
      const page = (await response.json()) as Partial<Dashboard>;
      const series = page.widgets?.liquidityChart?.series;
      if (!Array.isArray(series)) throw new Error(`${dashboardPath}: unexpected shape`);
      const tokens = series.map(latest).filter((token): token is TokenLiquidity => token !== null);
      if (tokens.length === 0) throw new Error(`${dashboardPath}: no series`);
      const total = tokens.reduce((sum, token) => sum + token.dollars, 0);
      set({ tokens, total, loadedAt: Date.now(), failure: '' });
    } catch (failure) {
      set({ failure: failure instanceof Error ? failure.message : String(failure) });
    }
  },
}));
