// One thing on the map, with the things inside it.
import { memo, useContext, useRef } from 'react';
import Box from '@mui/material/Box';
import { colorOf, type Region } from '../emoji';
import { fonts } from '../theme';
import { Mascot } from '../components/Mark';
import { regionNames, regionInks } from './words';
import { baseName, selfName, networkNames, pictures, type Place } from './replay';
import { YouContext, useShown } from './you';
import { type Register, type Moved, type OnMove, sizeLeast, tileSizeKey, tileEdge, groupHead, copyKey } from './layout';

export const Tile = memo(function Tile({ places, nth, born, carried, lit, stood, register, moved, onMove, zoom, depth = 0, within, ghost = false }: { places: Place[]; nth: number; born: number | null; carried: number | null; lit: number[]; stood: number | null; register: Register; moved: Moved; onMove: OnMove; zoom: number; depth?: number; within?: Region; ghost?: boolean }) {
  const inside = places.map((place, at) => ({ place, at })).filter(({ place }) => place.inside === nth);
  const name = places[nth].name;
  const { nameOf, pictureFor } = useShown();
  const you = useContext(YouContext);
  const picture = pictureFor(name);
  const ink = colorOf(baseName(name));
  // The copy inside a holder is dragged and sized on its own, apart from the one at the top of the group.
  const key = ghost ? copyKey(name) : name;
  const shift = moved[key] ?? { x: 0, y: 0 };
  const drag = useRef<{ x: number; y: number; fromX: number; fromY: number } | null>(null);
  const onDown = (event: React.PointerEvent<HTMLElement>): void => {
    event.stopPropagation();
    event.currentTarget.setPointerCapture(event.pointerId);
    drag.current = { x: event.clientX, y: event.clientY, fromX: shift.x, fromY: shift.y };
    pressed.current = { x: event.clientX, y: event.clientY };
  };
  // A press on the person's own tile that did not drag it opens the choice of their name and picture.
  const pressed = useRef<{ x: number; y: number } | null>(null);
  const onDragged = (event: React.PointerEvent<HTMLElement>): void => {
    if (!drag.current) return;
    const tile = event.currentTarget;
    const holder = tile.offsetParent instanceof HTMLElement ? tile.offsetParent : null;
    const wantX = drag.current.fromX + (event.clientX - drag.current.x) / zoom;
    const wantY = drag.current.fromY + (event.clientY - drag.current.y) / zoom;
    if (!holder) { onMove(key, wantX, wantY); return; }
    // Where the tile lies with no shift at all, and how far it may go each way before it leaves its holder.
    const restX = tile.offsetLeft - shift.x;
    const restY = tile.offsetTop - shift.y;
    const within = (want: number, rest: number, size: number, room: number, lead: number): number => Math.min(Math.max(want, lead - rest), Math.max(room - size - tileEdge - rest, lead - rest));
    onMove(key, within(wantX, restX, tile.offsetWidth, holder.clientWidth, tileEdge), within(wantY, restY, tile.offsetHeight, holder.clientHeight, depth === 0 ? groupHead : tileEdge));
  };
  const sizing = useRef<{ x: number; y: number; fromX: number; fromY: number } | null>(null);
  const onSizeStart = (event: React.PointerEvent<HTMLElement>): void => {
    event.stopPropagation();
    event.currentTarget.setPointerCapture(event.pointerId);
    const box = event.currentTarget.parentElement;
    sizing.current = { x: event.clientX, y: event.clientY, fromX: box?.offsetWidth ?? 0, fromY: box?.offsetHeight ?? 0 };
  };
  const onSized = (event: React.PointerEvent<HTMLElement>): void => {
    const from = sizing.current;
    if (!from) return;
    event.stopPropagation();
    onMove(tileSizeKey(key), Math.max(from.fromX + (event.clientX - from.x) / zoom, sizeLeast), Math.max(from.fromY + (event.clientY - from.y) / zoom, sizeLeast));
  };
  const onSizeEnd = (event: React.PointerEvent<HTMLElement>): void => { event.stopPropagation(); sizing.current = null; };
  const size = moved[tileSizeKey(key)];
  const onUp = (event: React.PointerEvent<HTMLElement>): void => {
    const from = pressed.current;
    if (drag.current && from && Math.hypot(event.clientX - from.x, event.clientY - from.y) < 4) {
      event.stopPropagation();
      if (name === selfName) you.edit(); else you.label(name);
    }
    drag.current = null;
    pressed.current = null;
  };
  return (
    <Box className="tile" ref={(element: HTMLElement | null) => { if (!ghost) register(nth, element); }} onPointerDown={onDown} onPointerMove={onDragged} onPointerUp={onUp} onPointerCancel={onUp} sx={{
      width: size?.x, height: size?.y, flexShrink: 0, boxSizing: 'border-box', left: shift.x, top: shift.y, cursor: 'grab', touchAction: 'none', userSelect: 'none', '&:active': { cursor: 'grabbing' },
      display: 'inline-flex', flexDirection: 'column', alignItems: 'center', gap: 0.75, px: 1.5, py: 1.25, borderRadius: 1.5,
      border: `2px ${places[nth].past ? 'dashed' : 'solid'} ${ink}`, filter: places[nth].past ? 'sepia(0.85) saturate(0.7) brightness(0.85)' : 'none', background: lit.includes(nth) ? `color-mix(in srgb, ${ink} 30%, #0a0e1c)` : `color-mix(in srgb, #ffffff ${depth * 7}%, #0a0e1c)`, color: ink,
      // A thing inside another stands higher, like a floor of a tower: a thick edge below it in its own color and a longer shadow.
      boxShadow: `0 ${6 + depth * 5}px 0 0 color-mix(in srgb, ${ink} 45%, #000), 0 ${12 + depth * 9}px ${14 + depth * 8}px rgba(0, 0, 0, 0.55)`, mb: `${6 + depth * 5}px`, fontFamily: fonts.mono, fontSize: '0.9375rem', whiteSpace: 'nowrap',
      opacity: carried === nth ? 0.3 : places[nth].unseen ? 0.5 : 1, borderStyle: places[nth].unseen ? 'dotted' : undefined, transition: 'opacity 200ms, background 300ms', position: 'relative', zIndex: 1,
      '@keyframes pop': { from: { transform: 'scale(0.2)', opacity: 0 }, to: { transform: 'scale(1)', opacity: 1 } },
      // Where he stands, the tile's own border goes between its color and white.
      '@keyframes here': { from: { borderColor: ink }, to: { borderColor: '#fff' } },
      animation: [born === nth ? 'pop 320ms ease-out' : '', stood === nth ? 'here 700ms ease-in-out infinite alternate' : ''].filter(Boolean).join(', ') || 'none',
    }}>
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.75 }}>
        {networkNames.includes(name) ? <Mascot size={28} spark={false} /> : picture && <Box component="span" sx={{ fontSize: '1.5rem', lineHeight: 1 }}>{picture}</Box>}
        <span>{`${places[nth].article ? `${places[nth].article} ` : ''}${nameOf(name)}${places[nth].count ? ` (${places[nth].count})` : ''}`}</span>
      </Box>
      {within && places[nth].region !== within && <Box component="span" sx={{ fontSize: '0.625rem', letterSpacing: '0.04em', textTransform: 'uppercase', color: regionInks[places[nth].region], opacity: 0.9 }}>{regionNames[places[nth].region]}</Box>}
      {places[nth].unseen && <Box component="span" sx={{ fontSize: '0.6875rem', color: '#fff', opacity: 0.9 }}>{pictures.unseen}</Box>}
      {places[nth].tags.length > 0 && (
        <Box sx={{ display: 'flex', gap: 0.5, flexWrap: 'wrap', justifyContent: 'center' }}>
          {places[nth].tags.map((text) => (
            <Box key={text} component="span" sx={{ px: 0.75, borderRadius: 999, background: 'rgba(255, 255, 255, 0.14)', color: '#fff', fontSize: '0.75rem' }}>{text}</Box>
          ))}
        </Box>
      )}
      {(
        <Box onPointerDown={onSizeStart} onPointerMove={onSized} onPointerUp={onSizeEnd} onPointerCancel={onSizeEnd} aria-label="resize" sx={{
          position: 'absolute', right: 2, bottom: 2, width: { xs: 22, md: 14 }, height: { xs: 22, md: 14 }, touchAction: 'none', cursor: 'nwse-resize', borderRight: `3px solid ${ink}`, borderBottom: `3px solid ${ink}`, borderBottomRightRadius: 6, opacity: { xs: 0.85, md: 0 }, transition: 'opacity 150ms', '.tile:hover > &': { opacity: 0.85 },
        }} />
      )}
      {inside.length > 0 && (
        <Box sx={{ display: 'flex', gap: 0.5, flexWrap: 'wrap', justifyContent: 'center', maxWidth: size ? 'none' : 280 }}>
          {inside.map(({ at }) => <Tile key={at} places={places} nth={at} born={born} carried={carried} lit={lit} stood={stood} register={register} moved={moved} onMove={onMove} zoom={zoom} depth={depth + 1} within={within ?? places[nth].region} ghost={ghost || places[at].rooted === true} />)}
        </Box>
      )}
    </Box>
  );
});
