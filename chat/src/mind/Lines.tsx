// The lines of the relations between tiles: drawn over the tiles, bent by a press and a drag, their names dragged,
// and all of it kept with the layout.
import { memo, useRef } from 'react';
import Box from '@mui/material/Box';
import { fonts, palette } from '../theme';
import { words } from './words';
import { type Line, type Moved, type OnMove, bendSnap, labelKey, withBends } from './layout';

export const Lines = memo(function Lines({ lines, moved, setMoved, onMove, zoom }: { lines: Line[]; moved: Moved; setMoved: React.Dispatch<React.SetStateAction<Moved>>; onMove: OnMove; zoom: number }) {
  // A press on a line puts a bend there and drags it; a press on a bend drags it; two presses on a bend take it out.
  const bendDrag = useRef<{ line: Line; nth: number } | null>(null);
  const worldSpot = (event: React.PointerEvent<SVGElement>): { x: number; y: number } => {
    const box = event.currentTarget.ownerSVGElement?.getBoundingClientRect() ?? { left: 0, top: 0 };
    return { x: (event.clientX - box.left) / zoom, y: (event.clientY - box.top) / zoom };
  };
  const onLineDown = (event: React.PointerEvent<SVGElement>, line: Line): void => {
    event.stopPropagation();
    event.currentTarget.setPointerCapture(event.pointerId);
    const at = worldSpot(event);
    // The new bend goes into the stretch of the line the press is nearest to.
    const way = [{ x: line.x1, y: line.y1 }, ...line.bends, { x: line.x2, y: line.y2 }];
    let nth = 0;
    let least = Infinity;
    for (let leg = 0; leg + 1 < way.length; leg += 1) {
      const dx = way[leg + 1].x - way[leg].x;
      const dy = way[leg + 1].y - way[leg].y;
      const along = Math.min(Math.max(((at.x - way[leg].x) * dx + (at.y - way[leg].y) * dy) / (dx * dx + dy * dy || 1), 0), 1);
      const far = Math.hypot(at.x - (way[leg].x + dx * along), at.y - (way[leg].y + dy * along));
      if (far < least) { least = far; nth = leg; }
    }
    const bends = [...line.bends.slice(0, nth), at, ...line.bends.slice(nth)];
    bendDrag.current = { line: { ...line, bends }, nth };
    setMoved((before) => withBends(before, line.key, bends));
  };
  const onBendDown = (event: React.PointerEvent<SVGElement>, line: Line, nth: number): void => {
    event.stopPropagation();
    event.currentTarget.setPointerCapture(event.pointerId);
    bendDrag.current = { line, nth };
  };
  const onBendDragged = (event: React.PointerEvent<SVGElement>): void => {
    const drag = bendDrag.current;
    if (!drag) return;
    event.stopPropagation();
    const at = worldSpot(event);
    const { line, nth } = drag;
    // Pulled onto the line up or across through the bend before and the bend after, or the middle of the tile at that end.
    const beside = [nth === 0 ? { x: line.ax, y: line.ay } : line.bends[nth - 1], nth === line.bends.length - 1 ? { x: line.bx, y: line.by } : line.bends[nth + 1]];
    for (const near of beside) {
      if (Math.abs(at.x - near.x) < bendSnap / zoom) at.x = near.x;
      if (Math.abs(at.y - near.y) < bendSnap / zoom) at.y = near.y;
    }
    const bends = line.bends.map((bend, other) => (other === nth ? at : bend));
    bendDrag.current = { line: { ...line, bends }, nth };
    setMoved((before) => withBends(before, line.key, bends));
  };
  const onBendUp = (): void => { bendDrag.current = null; };
  const labelDrag = useRef<{ key: string; x: number; y: number; fromX: number; fromY: number } | null>(null);
  const onLabelDown = (event: React.PointerEvent<SVGElement>, line: Line): void => {
    event.stopPropagation();
    event.currentTarget.setPointerCapture(event.pointerId);
    const kept = moved[labelKey(line.key)] ?? { x: 0, y: 0 };
    labelDrag.current = { key: line.key, x: event.clientX, y: event.clientY, fromX: kept.x, fromY: kept.y };
  };
  const onLabelDragged = (event: React.PointerEvent<SVGElement>): void => {
    const from = labelDrag.current;
    if (!from) return;
    event.stopPropagation();
    onMove(labelKey(from.key), from.fromX + (event.clientX - from.x) / zoom, from.fromY + (event.clientY - from.y) / zoom);
  };
  const onLabelUp = (): void => { labelDrag.current = null; };
  const dropBend = (line: Line, nth: number): void => setMoved((before) => withBends(before, line.key, line.bends.filter((_, other) => other !== nth)));
  return (
  <Box component="svg" sx={{ position: 'absolute', inset: 0, width: '100%', height: '100%', zIndex: 2, pointerEvents: 'none', overflow: 'visible' }}>
    <defs>
      <marker id="link-end" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse">
        <path d="M 0 0 L 10 5 L 0 10 z" fill={palette.linkInk} />
      </marker>
    </defs>
    {lines.map((line, nth) => {
      const { cx, cy } = line;
      const t = 0.66;
      const bent = line.bends.length > 0;
      const way = [{ x: line.x1, y: line.y1 }, ...line.bends, { x: line.x2, y: line.y2 }];
      const d = bent ? `M ${way.map((at) => `${at.x} ${at.y}`).join(' L ')}` : `M ${line.x1} ${line.y1} Q ${cx} ${cy} ${line.x2} ${line.y2}`;
      // The name of a bent line stands on its longest stretch.
      const longest = way.slice(1).reduce((best, at, leg) => (Math.hypot(at.x - way[leg].x, at.y - way[leg].y) > Math.hypot(way[best + 1].x - way[best].x, way[best + 1].y - way[best].y) ? leg : best), 0);
      const lx = bent ? (way[longest].x + way[longest + 1].x) / 2 : (1 - t) * (1 - t) * line.x1 + 2 * (1 - t) * t * cx + t * t * line.x2;
      const ly = bent ? (way[longest].y + way[longest + 1].y) / 2 : (1 - t) * (1 - t) * line.y1 + 2 * (1 - t) * t * cy + t * t * line.y2;
      return (
        <Box component="g" key={nth} sx={{ '& circle': { opacity: 0.35, transition: 'opacity 150ms' }, '&:hover circle': { opacity: 1 } }}>
          <path d={d} fill="none" stroke={palette.linkInk} strokeOpacity={line.past ? 0.4 : 0.7} strokeDasharray={line.past ? '6 5' : undefined} strokeWidth={2} strokeLinejoin="round" markerEnd="url(#link-end)" />
          <path d={d} fill="none" stroke="transparent" strokeWidth={16} style={{ pointerEvents: 'stroke', cursor: 'crosshair' }} onPointerDown={(event) => onLineDown(event, line)} onPointerMove={onBendDragged} onPointerUp={onBendUp} onPointerCancel={onBendUp}><title>{words.bendHint}</title></path>
          <text x={lx + (moved[labelKey(line.key)]?.x ?? 0)} y={ly - 6 + (moved[labelKey(line.key)]?.y ?? 0)} textAnchor="middle" fill="#fff" stroke="#10141f" strokeWidth={4} paintOrder="stroke" fontFamily={fonts.mono} fontSize={14} style={{ pointerEvents: 'all', cursor: 'move', userSelect: 'none' }} onPointerDown={(event) => onLabelDown(event, line)} onPointerMove={onLabelDragged} onPointerUp={onLabelUp} onPointerCancel={onLabelUp}>{line.label}</text>
          {line.bends.map((bend, at) => (
            <circle key={at} cx={bend.x} cy={bend.y} r={6} fill={palette.linkInk} stroke="#10141f" strokeWidth={2} style={{ pointerEvents: 'all', cursor: 'move' }} onPointerDown={(event) => onBendDown(event, line, at)} onPointerMove={onBendDragged} onPointerUp={onBendUp} onPointerCancel={onBendUp} onDoubleClick={() => dropBend(line, at)} />
          ))}
        </Box>
      );
    })}
  </Box>
  );
});
