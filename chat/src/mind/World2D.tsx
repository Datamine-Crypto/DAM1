// The map: the groups, the tiles, the lines between them and the walker.
import { memo, useCallback, useEffect, useLayoutEffect, useRef, useState } from 'react';
import Box from '@mui/material/Box';
import IconButton from '@mui/material/IconButton';
import Typography from '@mui/material/Typography';
import AddIcon from '@mui/icons-material/Add';
import FullscreenIcon from '@mui/icons-material/Fullscreen';
import RemoveIcon from '@mui/icons-material/Remove';
import CenterFocusStrongIcon from '@mui/icons-material/CenterFocusStrong';
import { colorOf, type Region } from '../emoji';
import { fonts, palette } from '../theme';
import { Mascot } from '../components/Mark';
import { words, regionNames, regionTints, regionInks } from './words';
import { lineGap, worldInk, pictures, type Scene } from './replay';
import { useShown } from './you';
import { House } from './House';
import { type Register, type Moved, type OnMove, groupKey, bendsOf,  sizeLeast, focusEvent, type Line, layoutPrefix, plainView, layoutOf, sizeKey, gridStep, zoomStep, wheelStep, zoomLeast, zoomMost } from './layout';
import { Tile } from './Tile';
import { Lines } from './Lines';

// The map: four regions around home, each holding the tiles that belong to it. The walker's place is
// measured from the tile he stands at, so he crosses the map both ways and stands on a thing inside another.
export const World2D = memo(function World2D({ scene, says, doing, answers, beat, layoutKey, output, question }: { scene: Scene; says: string[]; doing: string | null; answers: boolean; beat: number; layoutKey: string; output: string | null; question: string | null }) {
  // Going on to the next word is a nod of his own, not a picture.
  const nods = doing === pictures.next;
  const cool = doing === pictures.kept;
  const map = useRef<HTMLDivElement | null>(null);
  const home = useRef<HTMLDivElement | null>(null);
  const tiles = useRef(new Map<number, HTMLElement>());
  const [spot, setSpot] = useState<{ x: number; y: number } | null>(null);
  const [aim, setAim] = useState<{ x: number; y: number } | null>(null);
  const [moved, setMoved] = useState<Moved>(() => layoutOf(layoutKey).moved);
  // The view of the world: how much it is zoomed and how far it is moved, so a person lays it out as they like.
  const [view, setView] = useState(() => layoutOf(layoutKey).view);
  // The layout a person made, what they dragged and how they zoomed, is kept for each game in this browser.
  const loaded = useRef(layoutKey);
  useEffect(() => {
    if (loaded.current === layoutKey) return;
    loaded.current = layoutKey;
    const kept = layoutOf(layoutKey);
    setMoved(kept.moved);
    setView(kept.view);
  }, [layoutKey]);
  useEffect(() => {
    if (loaded.current !== layoutKey) return;
    try {
      window.localStorage.setItem(layoutPrefix + layoutKey, JSON.stringify({ moved, view }));
    } catch {
      // A browser that keeps nothing still plays the game.
    }
  }, [layoutKey, moved, view]);
  const zoomBy = useCallback((by: number) => setView((was) => ({ ...was, zoom: Math.min(Math.max(was.zoom * by, zoomLeast), zoomMost) })), []);
  const pan = useRef<{ x: number; y: number; fromX: number; fromY: number } | null>(null);
  // The fingers on the map, so two of them zoom it by the space between them while one moves the world.
  const touches = useRef<Map<number, { x: number; y: number }>>(new Map());
  const apart = useRef<number | null>(null);
  const spread = (): number | null => {
    const [one, two] = [...touches.current.values()];
    return one && two ? Math.hypot(one.x - two.x, one.y - two.y) : null;
  };
  const onPanStart = (event: React.PointerEvent<HTMLElement>): void => {
    if (event.target instanceof Element && event.target.closest('button')) return;
    event.currentTarget.setPointerCapture(event.pointerId);
    touches.current.set(event.pointerId, { x: event.clientX, y: event.clientY });
    if (touches.current.size > 1) { pan.current = null; apart.current = spread(); return; }
    pan.current = { x: event.clientX, y: event.clientY, fromX: view.x, fromY: view.y };
  };
  const onPanned = (event: React.PointerEvent<HTMLElement>): void => {
    if (touches.current.has(event.pointerId)) touches.current.set(event.pointerId, { x: event.clientX, y: event.clientY });
    if (touches.current.size > 1) {
      const now = spread();
      if (now !== null && apart.current !== null && apart.current > 0) zoomBy(now / apart.current);
      apart.current = now;
      return;
    }
    const from = pan.current;
    if (from) setView((was) => ({ ...was, x: from.fromX + event.clientX - from.x, y: from.fromY + event.clientY - from.y }));
  };
  const onPanEnd = (event: React.PointerEvent<HTMLElement>): void => {
    touches.current.delete(event.pointerId);
    if (touches.current.size < 2) apart.current = null;
    pan.current = null;
  };
  // Full screen takes the whole page, the game with its input, and the same button leaves it.
  const toggleFull = (): void => {
    if (document.fullscreenElement) void document.exitFullscreen(); else void document.documentElement.requestFullscreen().catch(() => undefined);
  };
  // A group is dragged as a tile is, with all its tiles, and is remembered under a key no thing's name can be.
  const groupDrag = useRef<{ key: string; x: number; y: number; fromX: number; fromY: number } | null>(null);
  const onGroupDown = (event: React.PointerEvent<HTMLElement>, region: Region): void => {
    event.stopPropagation();
    event.currentTarget.setPointerCapture(event.pointerId);
    const shift = moved[groupKey(region)] ?? { x: 0, y: 0 };
    groupDrag.current = { key: groupKey(region), x: event.clientX, y: event.clientY, fromX: shift.x, fromY: shift.y };
  };
  const onGroupDragged = (event: React.PointerEvent<HTMLElement>): void => {
    const from = groupDrag.current;
    if (from) onMove(from.key, from.fromX + (event.clientX - from.x) / view.zoom, from.fromY + (event.clientY - from.y) / view.zoom);
  };
  const onGroupUp = (): void => { groupDrag.current = null; };
  // The corner of a group sizes it: the size is kept with the layout, under a key of its own.
  const onSizeDown = (event: React.PointerEvent<HTMLElement>, region: Region): void => {
    event.stopPropagation();
    event.currentTarget.setPointerCapture(event.pointerId);
    const box = event.currentTarget.parentElement;
    groupDrag.current = { key: sizeKey(region), x: event.clientX, y: event.clientY, fromX: box?.offsetWidth ?? 0, fromY: box?.offsetHeight ?? 0 };
  };
  const onSizeDragged = (event: React.PointerEvent<HTMLElement>): void => {
    const from = groupDrag.current;
    if (!from) return;
    event.stopPropagation();
    onMove(from.key, Math.max(from.fromX + (event.clientX - from.x) / view.zoom, sizeLeast), Math.max(from.fromY + (event.clientY - from.y) / view.zoom, sizeLeast));
  };
  // The wheel zooms the world, a small step a notch; the game fills the window, so there is no page to scroll.
  const onWheel = (event: React.WheelEvent<HTMLElement>): void => zoomBy(event.deltaY < 0 ? wheelStep : 1 / wheelStep);
  const onMove = useCallback<OnMove>((name, x, y) => setMoved((before) => ({ ...before, [name]: { x, y } })), []);
  const [lines, setLines] = useState<Line[]>([]);
  const [shown, setShown] = useState<number | null>(null);
  useEffect(() => {
    const onFocus = (event: Event): void => {
      const name = (event as CustomEvent<string>).detail;
      const nth = scene.places.findIndex((place) => place.name === name);
      const tile = nth < 0 ? undefined : tiles.current.get(nth);
      const frame = map.current?.parentElement;
      if (!tile || !frame) return;
      const tileBox = tile.getBoundingClientRect();
      const frameBox = frame.getBoundingClientRect();
      const dx = tileBox.left + tileBox.width / 2 - (frameBox.left + frameBox.width / 2);
      const dy = tileBox.top + tileBox.height / 2 - (frameBox.top + frameBox.height / 2);
      setView((was) => ({ ...was, x: was.x - dx, y: was.y - dy }));
      setShown(nth);
      window.setTimeout(() => setShown((now) => (now === nth ? null : now)), 1600);
    };
    window.addEventListener(focusEvent, onFocus);
    return () => window.removeEventListener(focusEvent, onFocus);
  }, [scene]);
  const register = useCallback<Register>((nth, element) => {
    if (element) tiles.current.set(nth, element); else tiles.current.delete(nth);
  }, []);

  const measure = useCallback(() => {
    // Offsets, not the drawn box: a tile that is still popping up is drawn small, and its place must not be.
    const stood = scene.at === null ? home.current : tiles.current.get(scene.at) ?? home.current;
    if (!map.current || !stood) return;
    const placeOf = (from: HTMLElement): { x: number; y: number } => {
      let left = 0;
      let top = 0;
      for (let element: HTMLElement | null = from; element && element !== map.current; element = element.offsetParent as HTMLElement | null) {
        left += element.offsetLeft;
        top += element.offsetTop;
      }
      return { x: left, y: top };
    };
    let { x, y } = placeOf(stood);
    x += stood.offsetWidth / 2;
    const aimed = scene.points === null ? undefined : tiles.current.get(scene.points);
    const corner = aimed ? placeOf(aimed) : null;
    setAim(aimed && corner ? { x: corner.x + aimed.offsetWidth / 2, y: corner.y + aimed.offsetHeight / 2 } : null);
    const middle = (nth: number): { x: number; y: number; w: number; h: number } | null => {
      const tile = tiles.current.get(nth);
      if (!tile) return null;
      const at = placeOf(tile);
      return { x: at.x + tile.offsetWidth / 2, y: at.y + tile.offsetHeight / 2, w: tile.offsetWidth / 2 + lineGap, h: tile.offsetHeight / 2 + lineGap };
    };
    // How far along the line from a tile's middle its border is, as a part of the whole line.
    const border = (box: { w: number; h: number }, dx: number, dy: number): number => Math.min(dx === 0 ? Infinity : box.w / Math.abs(dx), dy === 0 ? Infinity : box.h / Math.abs(dy));
    // A line bends through a point beside the straight way between two tiles, and leaves and meets each tile on
    // the way from its middle to that point, so its arrow aims at the middle of the tile it ends at.
    setLines(scene.links.flatMap((link, nth) => {
      const a = middle(link.from);
      const b = middle(link.to);
      if (!a || !b) return [];
      const dx = b.x - a.x;
      const dy = b.y - a.y;
      const long = Math.hypot(dx, dy) || 1;
      const bend = (nth % 2 === 0 ? 1 : -1) * Math.min(14 + 10 * Math.floor(nth / 2), long / 3);
      const cx = (a.x + b.x) / 2 - (dy / long) * bend;
      const cy = (a.y + b.y) / 2 + (dx / long) * bend;
      const key = `${scene.places[link.from].name}>${scene.places[link.to].name}`;
      const bends = bendsOf(moved, key);
      // A line with bends leaves its tile toward the first bend and meets the other from the last; with none it is the curve.
      const first = bends.length > 0 ? bends[0] : { x: cx, y: cy };
      const last = bends.length > 0 ? bends[bends.length - 1] : { x: cx, y: cy };
      const start = border(a, first.x - a.x, first.y - a.y);
      const end = border(b, last.x - b.x, last.y - b.y);
      const from = Math.min(start, 1);
      const to = Math.min(end, 1);
      const x1 = a.x + (first.x - a.x) * from;
      const y1 = a.y + (first.y - a.y) * from;
      const x2 = b.x + (last.x - b.x) * to;
      const y2 = b.y + (last.y - b.y) * to;
      // A line shorter than this is between two tiles that touch, where it would say nothing.
      if (bends.length === 0 && Math.hypot(x2 - x1, y2 - y1) < 10) return [];
      return [{ key, x1, y1, x2, y2, cx, cy, ax: a.x, ay: a.y, bx: b.x, by: b.y, bends, label: link.label, past: link.past === true }];
    }));
    // Home is centred by a transform its offsets do not show.
    // At home he stands at the door of his house.
    if (stood === home.current) y += stood.offsetHeight * 0.92;
    setSpot({ x, y });
    // A dragged tile moves by its left and top, which the offsets above already hold.
  }, [scene, moved]);

  useLayoutEffect(measure, [measure]);
  useEffect(() => {
    if (!map.current || typeof ResizeObserver === 'undefined') return;
    const watcher = new ResizeObserver(measure);
    watcher.observe(map.current);
    return () => watcher.disconnect();
  }, [measure]);

  const { nameOf, pictureFor } = useShown();
  const held = scene.carries !== null ? scene.places[scene.carries] : undefined;
  // A group is on the map from the first thing of its kind, to the left and right of home by turns, in the order they came.
  // A word that only names a line and is tied to nothing has no tile: the line already says it.
  const onLines = new Set(scene.links.flatMap((link) => link.label.split(' ')));
  const tied = new Set<number>([...scene.links.flatMap((link) => [link.from, link.to]), ...scene.places.flatMap((place) => (place.inside === null ? [] : [place.inside]))]);
  const unshown = (at: number): boolean => onLines.has(scene.places[at].name) && !tied.has(at) && scene.at !== at && scene.carries !== at && scene.points !== at && scene.places[at].tags.length === 0;
  const atTop = (place: { inside: number | null; rooted?: boolean }): boolean => place.inside === null || place.rooted === true;
  const present = scene.places.filter((place, at) => atTop(place) && !unshown(at)).map((place) => place.region).filter((region, at, all) => all.indexOf(region) === at);
  const left = present.filter((_, at) => at % 2 === 0);
  const right = present.filter((_, at) => at % 2 === 1);
  return (
    <Box onPointerDown={onPanStart} onPointerMove={onPanned} onPointerUp={onPanEnd} onPointerCancel={onPanEnd} onWheel={onWheel} sx={{
      position: 'relative', minHeight: { xs: 380, md: 0 }, height: '100%', borderRadius: { xs: 0, md: 2 }, overflow: 'hidden', border: `1px solid ${palette.divider}`,
      backgroundColor: 'rgba(0, 255, 255, 0.03)', display: 'grid', cursor: 'move', touchAction: 'none', userSelect: 'none',
      // The ground's grid is drawn on the frame and follows the view, so it has no edge however far the world is moved or zoomed out.
      backgroundImage: 'linear-gradient(rgba(255,255,255,0.05) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.05) 1px, transparent 1px)',
      backgroundSize: `${gridStep * view.zoom}px ${gridStep * view.zoom}px`, backgroundPosition: `calc(50% + ${view.x}px) calc(50% + ${view.y}px)`,
    }}>
    <Box sx={{ position: 'absolute', top: 10, right: 10, zIndex: 4, display: 'flex', gap: 0.25, borderRadius: 999, background: 'rgba(10, 14, 28, 0.85)', border: `1px solid ${palette.divider}` }}>
      <IconButton size="small" onClick={() => zoomBy(1 / zoomStep)} aria-label="zoom out"><RemoveIcon fontSize="small" /></IconButton>
      <IconButton size="small" onClick={() => setView(plainView)} aria-label="reset the view"><CenterFocusStrongIcon fontSize="small" /></IconButton>
      <IconButton size="small" onClick={() => zoomBy(zoomStep)} aria-label="zoom in"><AddIcon fontSize="small" /></IconButton>
      <IconButton size="small" onClick={toggleFull} aria-label="full screen" title="Full screen"><FullscreenIcon fontSize="small" /></IconButton>
    </Box>
    <Box ref={map} sx={{
      position: 'relative', minHeight: 'inherit', transform: `translate(${view.x}px, ${view.y}px) scale(${view.zoom})`, transformOrigin: '50% 50%',
      display: 'grid', gridTemplateColumns: '1fr auto 1fr', alignItems: 'center', columnGap: 14,
      // Room around the groups, so what he says over a tile of the top row and the controls at the bottom stay inside the map.
      pt: 11, pb: 9, px: 2,
    }}>
      {[left, null, right].map((side, column) => (side === null ? (
        <Box key="home" ref={home} aria-label={words.home} sx={{ position: 'relative', width: 132, height: 118, my: 14, filter: 'drop-shadow(0 6px 5px rgba(0,0,0,0.45))' }}>
          <House />
          {(scene.curious.length > 0 || output || question) && (
            <Box sx={{ position: 'absolute', top: '100%', left: '50%', transform: 'translateX(-50%)', mt: 1, display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 0.25, width: 'max-content', maxWidth: 320 }}>
              {scene.curious.length > 0 && <Typography sx={{ color: 'text.secondary', fontSize: '0.8125rem', letterSpacing: '0.04em', textTransform: 'uppercase' }}>{words.curious}</Typography>}
              <Box sx={{ display: 'flex', gap: 0.5, flexWrap: 'wrap', justifyContent: 'center' }}>
                {scene.curious.map((name) => (
                  <Box key={name} sx={{ borderRadius: 999, border: `1px dashed ${colorOf(name)}`, color: colorOf(name), fontFamily: fonts.mono, fontSize: '1.125rem', lineHeight: 1.5, px: 1.5, py: 0.5, background: 'rgba(10, 14, 28, 0.85)' }}>
                    {`${pictureFor(name) ?? ''} ${nameOf(name)}`.trim()}
                  </Box>
                ))}
              </Box>
              {question && (
                <Box sx={{ mt: 1, px: 1.5, py: 0.5, borderRadius: 2, border: `2px dashed ${worldInk}`, background: 'rgba(8, 16, 28, 0.92)', textAlign: 'center' }}>
                  <Typography sx={{ color: 'text.secondary', fontSize: '0.6875rem', letterSpacing: '0.04em', textTransform: 'uppercase' }}>{words.question}</Typography>
                  <Typography sx={{ fontSize: '1.125rem', color: worldInk, overflowWrap: 'anywhere' }}>{question}</Typography>
                </Box>
              )}
              {output && (
                <Box sx={{ mt: 1, px: 2, py: 0.75, borderRadius: 2, border: `2px solid ${palette.linkInk}`, background: 'rgba(8, 16, 28, 0.92)', textAlign: 'center', '@keyframes said': { from: { opacity: 0, transform: 'translateY(6px)' }, to: { opacity: 1, transform: 'none' } }, animation: 'said 260ms ease-out' }}>
                  <Typography sx={{ color: 'text.secondary', fontSize: '0.6875rem', letterSpacing: '0.04em', textTransform: 'uppercase' }}>{words.output}</Typography>
                  <Typography sx={{ fontFamily: fonts.serif, fontSize: '1.75rem', lineHeight: 1.2, overflowWrap: 'anywhere', color: '#fff' }}>{output}</Typography>
                </Box>
              )}
            </Box>
          )}
        </Box>
      ) : (
        <Box key={column} sx={{ display: 'flex', flexDirection: 'column', gap: 3, alignItems: column === 0 ? 'flex-end' : 'flex-start', minWidth: 0 }}>
          {side.map((region) => (
            <Box key={region} className="group" onPointerDown={(event) => onGroupDown(event, region)} onPointerMove={onGroupDragged} onPointerUp={onGroupUp} onPointerCancel={onGroupUp} sx={{
              left: moved[groupKey(region)]?.x ?? 0, top: moved[groupKey(region)]?.y ?? 0, cursor: 'grab',
              width: moved[sizeKey(region)]?.x, height: moved[sizeKey(region)]?.y, minWidth: moved[sizeKey(region)] ? 0 : 120, minHeight: moved[sizeKey(region)] ? 0 : 80, maxWidth: moved[sizeKey(region)] ? 'none' : '100%', flexShrink: 0, boxSizing: 'border-box', touchAction: 'none', userSelect: 'none', '&:active': { cursor: 'grabbing' },
              position: 'relative', border: `2px dashed ${regionInks[region]}`, background: regionTints[region], borderRadius: 3, p: 1.5, pt: 3.5,
              // Room between the tiles of a group, so a relation's line and its name between two of them can be seen.
              display: 'flex', flexWrap: 'wrap', columnGap: 7, rowGap: 4, alignItems: 'center', justifyContent: 'center',
              '@keyframes open': { from: { transform: 'scale(0.6)', opacity: 0 }, to: { transform: 'scale(1)', opacity: 1 } }, animation: 'open 300ms ease-out',
            }}>
              <Box onPointerDown={(event) => onSizeDown(event, region)} onPointerMove={onSizeDragged} onPointerUp={onGroupUp} onPointerCancel={onGroupUp} aria-label="resize the group" sx={{
                position: 'absolute', right: 2, bottom: 2, width: { xs: 24, md: 16 }, height: { xs: 24, md: 16 }, touchAction: 'none', cursor: 'nwse-resize', borderRight: `3px solid ${regionInks[region]}`, borderBottom: `3px solid ${regionInks[region]}`, borderBottomRightRadius: 8, opacity: { xs: 0.85, md: 0 }, transition: 'opacity 150ms', '.group:hover > &': { opacity: 0.85 },
              }} />
              <Typography sx={{ position: 'absolute', top: 6, left: 12, color: regionInks[region], fontSize: '0.75rem', letterSpacing: '0.04em', textTransform: 'uppercase' }}>
                {regionNames[region]}
              </Typography>
              {scene.places.map((place, at) => (atTop(place) && place.region === region && !unshown(at)
                ? <Tile key={at} places={scene.places} nth={at} born={scene.born} carried={scene.carries} lit={shown === null ? scene.lit : [...scene.lit, shown]} stood={scene.at} register={register} moved={moved} onMove={onMove} zoom={view.zoom} />
                : null))}
            </Box>
          ))}
        </Box>
      )))}
      <Lines lines={lines} moved={moved} setMoved={setMoved} onMove={onMove} zoom={view.zoom} />
      {spot && aim && (
        <Box component="svg" sx={{ position: 'absolute', inset: 0, width: '100%', height: '100%', zIndex: 1, pointerEvents: 'none', '@keyframes aim': { to: { strokeDashoffset: -24 } } }}>
          <Box component="line" x1={spot.x} y1={spot.y - 24} x2={aim.x} y2={aim.y} stroke="#ffd166" strokeWidth={3} strokeDasharray="8 4" strokeLinecap="round" sx={{ animation: 'aim 600ms linear infinite' }} />
          <circle cx={aim.x} cy={aim.y} r={7} fill="none" stroke="#ffd166" strokeWidth={3} />
          <circle cx={aim.x} cy={aim.y} r={2.5} fill="#ffd166" />
        </Box>
      )}
      {spot && (
        <Box sx={{
          position: 'absolute', left: spot.x, top: spot.y + 6, transform: 'translate(-50%, -100%)', zIndex: 2,
          transition: 'left 460ms ease-in-out, top 460ms ease-in-out', display: 'flex', flexDirection: 'column', alignItems: 'center', pointerEvents: 'none',
          '@keyframes hop': { '0%, 100%': { transform: 'translateY(0)' }, '50%': { transform: 'translateY(-5px)' } },
          '@keyframes rise': { from: { opacity: 0, transform: 'translateY(8px)' }, to: { opacity: 1, transform: 'none' } },
        }}>
          {says.map((text, nth) => {
            const newest = nth === says.length - 1;
            return (
              <Box key={`${nth}-${text}`} sx={{
                mb: newest && scene.at === null ? 9 : 0.4, px: newest && answers ? 2 : 1.25, py: newest && answers ? 0.5 : 0.25, borderRadius: 1.5, width: 'max-content',
                border: newest && answers ? '2px solid #08101c' : 'none', boxShadow: newest && answers ? '0 4px 10px rgba(0,0,0,0.5)' : 'none', maxWidth: 360, whiteSpace: 'normal', textAlign: 'center', overflowWrap: 'anywhere', fontFamily: fonts.mono,
                background: newest ? (answers ? palette.linkInk : '#fff') : 'rgba(255,255,255,0.75)', color: '#111', fontWeight: newest && answers ? 700 : 400,
                fontSize: newest ? (answers ? '1.25rem' : '0.9375rem') : '0.75rem', opacity: newest ? 1 : 0.35 + (0.45 * nth) / says.length, animation: newest ? 'rise 220ms ease-out' : 'none',
              }}>{text}</Box>
            );
          })}
          <Box key={nods ? beat : 'walks'} sx={{ position: 'relative', display: 'flex', transformOrigin: '50% 100%', animation: nods ? 'nod 420ms ease-out' : 'hop 460ms ease-in-out infinite',
            '@keyframes nod': { '0%': { transform: 'scale(1, 1)' }, '30%': { transform: 'scale(1.18, 0.78)' }, '60%': { transform: 'scale(0.92, 1.12) translateY(-8px)' }, '100%': { transform: 'scale(1, 1)' } }, filter: 'drop-shadow(0 4px 3px rgba(0,0,0,0.5))' }}>
            {doing && !nods && !cool && (
              <Box key={doing} component="span" sx={{ position: 'absolute', right: -14, bottom: 2, zIndex: 1, fontSize: '1.625rem', lineHeight: 1, '@keyframes show': { from: { transform: 'scale(0.3)', opacity: 0 }, to: { transform: 'scale(1)', opacity: 1 } }, animation: 'show 200ms ease-out' }}>{doing}</Box>
            )}
            <Mascot size={56} spark={false} shades={cool} />
            {scene.notes.length > 0 && (
              <Box sx={{ position: 'absolute', left: '96%', bottom: 6, display: 'flex', flexDirection: 'column', gap: 0.25, alignItems: 'flex-start' }}>
                {scene.notes.map((note, nth) => (
                  <Box key={`${note}-${nth}`} sx={{
                    px: 0.75, borderRadius: 0.5, background: '#ffe98a', color: '#2b2300', fontFamily: fonts.mono, fontSize: '0.75rem', whiteSpace: 'nowrap', boxShadow: '0 1px 2px rgba(0,0,0,0.4)',
                    transform: `rotate(${nth % 2 === 0 ? 3 : -3}deg)`, '@keyframes stick': { from: { transform: 'scale(0.3)', opacity: 0 }, to: { opacity: 1 } }, animation: 'stick 200ms ease-out',
                  }}>{note}</Box>
                ))}
              </Box>
            )}
            {held && (
              <Box sx={{
                position: 'absolute', right: '88%', bottom: 4, px: 0.75, whiteSpace: 'nowrap', borderRadius: 1, border: `2px solid ${colorOf(held.name)}`, color: colorOf(held.name), background: 'rgba(10, 14, 28, 0.9)',
                fontFamily: fonts.mono, fontSize: '0.8125rem', display: 'flex', flexDirection: 'column', alignItems: 'center', transform: 'rotate(8deg)',
              }}>
                {pictureFor(held.name) && <Box component="span" sx={{ fontSize: '1.5rem', lineHeight: 1.1 }}>{pictureFor(held.name)}</Box>}
                <span>{nameOf(held.name)}</span>
              </Box>
            )}
          </Box>
        </Box>
      )}
    </Box>
    </Box>
  );
});
