// The small stamp of what the page was built from.
import { memo, useEffect, useState } from 'react';
import Typography from '@mui/material/Typography';
import { fonts } from '../theme';
import { words } from './words';

// What the build script wrote about the page: the commit, the time and the bytes the page downloads, which are the
// weights zipped, the facts and the engine. It is read once for the whole page.
export interface Build { commit: string; built: string; bytes: number }
let asked: Promise<Build | null> | null = null;
function buildOf(): Promise<Build | null> {
  asked ??= fetch('/build.json').then((answer) => (answer.ok ? answer.json() : null)).then((told: unknown) => {
    if (typeof told !== 'object' || told === null) return null;
    const { commit, built, bytes } = told as { commit?: unknown; built?: unknown; bytes?: unknown };
    return { commit: typeof commit === 'string' ? commit : '', built: typeof built === 'string' ? built : '', bytes: typeof bytes === 'number' ? bytes : 0 };
  }).catch(() => null);
  return asked;
}

export function useBuild(): Build | null {
  const [build, setBuild] = useState<Build | null>(null);
  useEffect(() => {
    let live = true;
    void buildOf().then((told) => { if (live) setBuild(told); });
    return () => { live = false; };
  }, []);
  return build;
}

export const megabytes = (bytes: number): string => `${(bytes / 1_048_576).toFixed(1)} MB`;

// The stamp at the corner of the map; a page built without the file shows nothing.
export const BuildStamp = memo(function BuildStamp() {
  const build = useBuild();
  if (!build) return null;
  const stamp = [words.model, build.commit, build.bytes > 0 ? megabytes(build.bytes) : '', build.built].filter(Boolean).join(' · ');
  return <Typography sx={{ position: 'absolute', right: 12, bottom: 8, zIndex: 3, fontFamily: fonts.mono, fontSize: '0.6875rem', color: 'text.secondary', opacity: 0.8, pointerEvents: 'none', maxWidth: 'calc(100% - 24px)', textAlign: 'right', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>{stamp}</Typography>;
});
