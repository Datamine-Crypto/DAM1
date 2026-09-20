// His stats beside the map: where he stands, what he holds, the overview, the stack and the analytics.
import { memo, useState } from 'react';
import Box from '@mui/material/Box';
import Tab from '@mui/material/Tab';
import Tabs from '@mui/material/Tabs';
import Typography from '@mui/material/Typography';
import { colorOf } from '../emoji';
import { useChatStore } from '../store';
import { fonts, palette } from '../theme';
import { words } from './words';
import { megabytes, useBuild } from './BuildStamp';
import { partsOf, singular, articleFlags, pictures, type Frame, type GameTurn } from './replay';
import { useShown } from './you';
import { focusEvent } from './layout';

const Named = memo(function Named({ name }: { name: string }) {
  const { nameOf, pictureFor } = useShown();
  const picture = pictureFor(name);
  return (
    <Box component="span" sx={{ color: name === words.nothing ? 'text.secondary' : colorOf(name), fontFamily: fonts.mono, fontSize: '0.9375rem' }}>
      {picture ? `${picture} ${nameOf(name)}` : nameOf(name)}
    </Box>
  );
});

const Stat = memo(function Stat({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', gap: 1, borderBottom: `1px solid ${palette.divider}`, py: 0.5 }}>
      <Typography sx={{ color: 'text.secondary', fontSize: '0.8125rem' }}>{label}</Typography>
      <Box sx={{ whiteSpace: 'nowrap', flexShrink: 0 }}>{children}</Box>
    </Box>
  );
});

// What he memorized, as one tree: paths that start the same share their branch, so the relations of a thing
// stand together under it.
interface MemoryNode {
  name: string;
  fresh: boolean;
  // True for what a described world holds that he has not seen yet.
  unseen: boolean;
  children: MemoryNode[];
}

function memoryTree(paths: string[], fresh: string[], hidden: string[], notes: Map<string, string[]> = new Map()): MemoryNode[] {
  const roots: MemoryNode[] = [];
  for (const path of paths) {
    const bright = fresh.includes(path);
    const unseen = hidden.includes(path);
    let level = roots;
    // An article flag, a thing said with a or the, is not shown: it says nothing of the world.
    const names = partsOf(path).filter((name, at, all) => !(articleFlags.includes(name) && all[at + 1] === 'true') && !(name === 'true' && articleFlags.includes(all[at - 1] ?? '')));
    // A thing's owner written under it, when the owner already stands above it on the path, says nothing new.
    const ownerAt = names.indexOf('{owner}');
    const shown = ownerAt > 0 && names.slice(0, ownerAt).includes(names[ownerAt + 1] ?? '') ? names.slice(0, ownerAt) : names;
    for (const name of shown) {
      let node = level.find((one) => one.name === name);
      if (!node) { node = { name, fresh: bright, unseen, children: [] }; level.push(node); }
      if (bright) node.fresh = true;
      if (!unseen) node.unseen = false;
      // The notes the map draws on the thing, went or a count, stand under it once.
      for (const note of notes.get(singular(name)) ?? []) {
        if (!node.children.some((one) => one.name === note)) node.children.push({ name: note, fresh: bright, unseen, children: [] });
      }
      level = node.children;
    }
  }
  return roots;
}

const Branch = memo(function Branch({ node, depth }: { node: MemoryNode; depth: number }) {
  const relation = node.name.startsWith('{');
  const { nameOf, pictureFor } = useShown();
  const picture = relation ? null : pictureFor(node.name);
  return (
    <Box sx={{ ml: depth === 0 ? 0 : 1.25, pl: depth === 0 ? 0 : 1, borderLeft: depth === 0 ? 'none' : `1px solid ${palette.divider}` }}>
      <Typography onClick={relation ? undefined : () => window.dispatchEvent(new CustomEvent(focusEvent, { detail: node.name }))} sx={{ cursor: relation ? 'default' : 'pointer', '&:hover': relation ? {} : { textDecoration: 'underline' }, fontFamily: fonts.mono, fontSize: '0.8125rem', overflowWrap: 'anywhere', color: relation ? palette.linkInk : colorOf(node.name), opacity: node.unseen ? 0.45 : node.fresh ? 1 : 0.7, fontWeight: node.fresh ? 700 : 400, fontStyle: node.unseen ? 'italic' : 'normal' }}>
        {relation ? node.name.replace(/[{}]/g, '') : picture ? `${picture} ${nameOf(node.name)}` : nameOf(node.name)}
        {node.unseen && depth === 0 ? ` ${pictures.unseen}` : ''}
      </Typography>
      {node.children.map((child) => <Branch key={child.name} node={child} depth={depth + 1} />)}
    </Box>
  );
});

// His stats, on the right of the map: where he is, what he holds, what he sees and all he memorized.
// How long ago a time was, in the largest unit that fits.
export function ago(then: number, now: number): string {
  const seconds = Math.max(Math.round((now - then) / 1000), 0);
  if (seconds < 45) return 'just now';
  const units: [number, string][] = [[86_400, 'day'], [3_600, 'hour'], [60, 'minute']];
  for (const [size, name] of units) {
    const count = Math.round(seconds / size);
    if (seconds >= size * 0.75) return `${count} ${name}${count === 1 ? '' : 's'} ago`;
  }
  return 'just now';
}

export const speeds = [1, 2, 4, 8];
export const instantKey = 'mind-instant';
const stackInks = { cursor: palette.mutedInk, word: '#fff', move: palette.linkInk, pointed: '#ffd166' } as const;

export const Stats = memo(function Stats({ frame, turns, pick, onPick }: { frame: Frame; turns: GameTurn[]; pick?: number; onPick?: (picked: number) => void }) {
  const engine = useChatStore((state) => state.agent);
  const build = useBuild();
  const scene = frame.scene;
  const [own, setOwn] = useState(0);
  // The game may hold the tabs itself, on a phone where the map is one of them.
  const tab = pick ?? own;
  const setTab = onPick ?? setOwn;
  const open = tab === 1;
  const memorized = turns.slice(0, frame.kept).flatMap((turn) => turn.wrote);
  // What a described world wrote stays not seen until the thing its path starts from has been seen on the map.
  const unseenNames = scene.places.filter((place) => place.unseen).map((place) => singular(place.name));
  const hidden = turns.slice(0, frame.kept).filter((turn) => turn.world).flatMap((turn) => turn.wrote).filter((path) => partsOf(path).some((name) => unseenNames.includes(singular(name))));
  // The paths of the turn just kept are drawn bright, so what he learned last stands out in the tree.
  const fresh = turns[frame.kept - 1]?.world ? [] : turns[frame.kept - 1]?.wrote ?? [];
  // The notes the map draws on each tile, by the name they belong to.
  const noted = new Map(scene.places.filter((place) => place.tags.length > 0).map((place) => [singular(place.name), place.tags]));
  return (
    <Box sx={{ border: `1px solid ${palette.divider}`, borderRadius: 2, p: 1.5, display: 'flex', flexDirection: 'column', gap: 1, minWidth: 0, minHeight: 0, overflowY: 'auto' }}>
      <Typography sx={{ fontSize: '0.9375rem', fontWeight: 500 }}>{words.stats}</Typography>
      <Box>
        <Stat label={words.stands}><Named name={scene.at === null ? words.home : scene.places[scene.at].name} /></Stat>
        <Stat label={words.holds}><Named name={scene.carries === null ? words.nothing : scene.places[scene.carries].name} /></Stat>
        <Stat label={words.pointsAt}><Named name={scene.points === null ? words.nothing : scene.places[scene.points].name} /></Stat>
        <Stat label={words.notes}>
          <Box sx={{ display: 'flex', gap: 0.5, flexWrap: 'wrap', justifyContent: 'flex-end' }}>
            {scene.notes.length === 0 ? <Named name={words.nothing} /> : scene.notes.map((note, nth) => <Named key={`${note}-${nth}`} name={note} />)}
          </Box>
        </Stat>
      </Box>
      <Tabs value={tab} onChange={(_, picked: number) => setTab(picked)} variant="fullWidth" sx={{ display: pick === undefined ? 'flex' : 'none', minHeight: 36, '& .MuiTab-root': { minHeight: 36, minWidth: 0, px: 0.5, textTransform: 'none', fontSize: '0.8125rem', whiteSpace: 'nowrap' } }}>
        <Tab label={words.memorized} />
        <Tab label={words.analytics} />
        <Tab label={words.counts} />
      </Tabs>
      <Box sx={{ display: tab === 0 ? 'block' : 'none' }}>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 0.5 }}>
          {memorized.length === 0 && <Typography sx={{ color: 'text.secondary', fontSize: '0.8125rem' }}>{words.memorizedNone}</Typography>}
          {memoryTree(memorized, fresh, hidden, noted).map((node) => <Branch key={node.name} node={node} depth={0} />)}
        </Box>
      </Box>
      <Box>
        {open && (
          <Box>
            <Typography sx={{ color: 'text.secondary', fontSize: '0.75rem', mt: 0.5, mb: 0.5 }}>{frame.answers ? words.stackWhole : words.stackNow}</Typography>
            <Box sx={{ display: 'flex', flexDirection: 'column-reverse', gap: 0.5, mb: 1 }}>
              {frame.stack.map((item, nth) => (
                <Box key={`${item.text}-${nth}`} sx={{ fontFamily: fonts.mono, fontSize: '0.75rem', px: 1, py: 0.25, borderRadius: 1, border: `1px solid ${stackInks[item.kind]}`, color: stackInks[item.kind], overflowWrap: 'anywhere' }}>
                  {item.kind === 'pointed' ? `points at: ${item.text}` : item.text}
                </Box>
              ))}
            </Box>
          </Box>
        )}
        {tab === 2 && (
          <Box sx={{ mt: 0.5 }}>
            <Stat label={words.turnsSaid}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{turns.length}</Typography></Stat>
            <Stat label={words.questions}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{turns.filter((turn) => turn.asks).length}</Typography></Stat>
            <Stat label={words.heard}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{frame.heardSoFar}</Typography></Stat>
            <Stat label={words.moved}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{frame.movedSoFar}</Typography></Stat>
            <Stat label={words.things}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{scene.places.length}</Typography></Stat>
            <Stat label={words.relations}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{scene.links.length}</Typography></Stat>
            <Stat label={words.groupsShown}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{new Set(scene.places.map((place) => place.region)).size}</Typography></Stat>
            <Stat label={words.stackSize}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{frame.stack.length}</Typography></Stat>
            <Typography sx={{ color: 'text.secondary', fontSize: '0.75rem', letterSpacing: '0.04em', textTransform: 'uppercase', mt: 2, mb: 0.5 }}>{words.network}</Typography>
            <Stat label={words.voters}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{engine.settings?.voters ?? 1}</Typography></Stat>
            <Stat label={words.weights}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{engine.weightCount.toLocaleString('en-US')}</Typography></Stat>
            <Stat label={words.downloadSize}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{build && build.bytes > 0 ? megabytes(build.bytes) : ''}</Typography></Stat>
            <Stat label={words.weightSize}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{megabytes(engine.weightBytes)}</Typography></Stat>
            <Stat label={words.engineSize}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{megabytes(engine.modelBytes)}</Typography></Stat>
            <Stat label={words.knowledgeSize}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{megabytes(engine.stateBytes)}</Typography></Stat>
            <Stat label={words.knowledge}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{engine.nodes.toLocaleString('en-US')}</Typography></Stat>
            <Stat label={words.movesKnown}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{engine.settings?.classes.length ?? 0}</Typography></Stat>
            <Stat label={words.hiddenWidth}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{engine.settings?.hidden ?? 0}</Typography></Stat>
            <Stat label={words.stackSlots}><Typography sx={{ fontFamily: fonts.mono, fontSize: '0.875rem' }}>{engine.settings?.slots ?? 0}</Typography></Stat>
          </Box>
        )}
      </Box>
    </Box>
  );
});
