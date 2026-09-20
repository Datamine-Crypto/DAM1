import { memo } from 'react';
import Box from '@mui/material/Box';
import { palette } from '../theme';

// The mark from the reference site: a ring around a D, drawn rather than fetched so the page has
// no outside asset to wait on.
export const Mark = memo(function Mark({ size = 28 }: { size?: number }) {
  return (
    <Box component="svg" viewBox="0 0 32 32" sx={{ width: size, height: size, flexShrink: 0 }} aria-hidden="true">
      <circle cx="16" cy="16" r="15" fill="none" stroke={palette.linkInk} strokeWidth="1.5" />
      <path d="M10 23V9h5.5c4.1 0 6.5 2.7 6.5 7s-2.4 7-6.5 7H10zm3.2-2.6h2.1c2.4 0 3.6-1.6 3.6-4.4s-1.2-4.4-3.6-4.4h-2.1z" fill={palette.ink} />
    </Box>
  );
});

const cheekInk = '#ff8fb1';

// DAM1's mascot, which heads a reply, the greeting and the launch note: a small blob in the accent
// with a face, a shine, and the old spark beside it. While the reader works it squishes and blinks.
export const Mascot = memo(function Mascot({ size = 24, busy = false, spark = true, shades = false }: { size?: number; busy?: boolean; spark?: boolean; shades?: boolean }) {
  return (
    <Box
      component="svg"
      viewBox="0 0 32 32"
      aria-hidden="true"
      sx={{
        width: size,
        height: size,
        flexShrink: 0,
        overflow: 'visible',
        color: palette.linkInk,
        '@keyframes mascot-squish': { '0%, 100%': { transform: 'scale(1, 1)' }, '50%': { transform: 'scale(1.07, 0.92)' } },
        '@keyframes mascot-blink': { '0%, 44%, 52%, 100%': { transform: 'scaleY(1)' }, '48%': { transform: 'scaleY(0.1)' } },
        '@keyframes mascot-hop': {
          '0%, 100%': { transform: 'translateY(0) scale(1, 1) rotate(0deg)' },
          '15%': { transform: 'translateY(0) scale(1.1, 0.88) rotate(0deg)' },
          '40%': { transform: 'translateY(-22%) scale(0.94, 1.08) rotate(-6deg)' },
          '65%': { transform: 'translateY(0) scale(1.08, 0.9) rotate(4deg)' },
          '80%': { transform: 'translateY(0) scale(0.98, 1.02) rotate(0deg)' },
        },
        '@keyframes mascot-twinkle': { '0%, 100%': { transform: 'scale(1) rotate(0deg)' }, '50%': { transform: 'scale(1.6) rotate(90deg)' } },
        '& .body': { transformBox: 'fill-box', transformOrigin: '50% 100%', animation: busy ? 'mascot-squish 1.1s ease-in-out infinite' : 'none' },
        '& .eyes': { transformBox: 'fill-box', transformOrigin: 'center', animation: busy ? 'mascot-blink 2.4s ease-in-out infinite' : 'none' },
        '& .spark': { transformBox: 'fill-box', transformOrigin: 'center' },
        '@keyframes mascot-shades': { from: { transform: 'translateY(-9px)', opacity: 0 }, to: { transform: 'translateY(0)', opacity: 1 } },
        '& .shades': { animation: 'mascot-shades 360ms ease-out' },
        // A pointer over the mascot makes it hop and wiggle, blink, and its spark twinkle.
        '&:hover .body': { animation: 'mascot-hop 0.8s ease-in-out infinite' },
        '&:hover .eyes': { animation: 'mascot-blink 1.6s ease-in-out infinite' },
        '&:hover .spark': { animation: 'mascot-twinkle 0.8s ease-in-out infinite' },
        '@media (prefers-reduced-motion: reduce)': { '& .body, & .eyes, & .spark, &:hover .body, &:hover .eyes, &:hover .spark': { animation: 'none' } },
      }}
    >
      {spark && <path className="spark" d="M27 2.2 L27.7 4.1 L29.6 4.8 L27.7 5.5 L27 7.4 L26.3 5.5 L24.4 4.8 L26.3 4.1 Z" fill="currentColor" />}
      <g className="body">
        <path d="M16 5 C21.4 5 24.9 9.2 26.3 14.8 C27.8 20.8 28.4 27.6 21.8 27.9 Q16 28.5 10.2 27.9 C3.6 27.6 4.2 20.8 5.7 14.8 C7.1 9.2 10.6 5 16 5 Z" fill="currentColor" />
        <ellipse cx="11.2" cy="10.4" rx="2.4" ry="1.3" fill={palette.ink} opacity="0.55" transform="rotate(-35 11.2 10.4)" />
        <g className="eyes">
          <ellipse cx="12.4" cy="17" rx="1.75" ry="2.25" fill={palette.pageBackground} />
          <ellipse cx="19.6" cy="17" rx="1.75" ry="2.25" fill={palette.pageBackground} />
          <circle cx="13" cy="16.2" r="0.6" fill={palette.ink} />
          <circle cx="20.2" cy="16.2" r="0.6" fill={palette.ink} />
        </g>
        <circle cx="9.6" cy="20.6" r="1.35" fill={cheekInk} opacity="0.8" />
        <circle cx="22.4" cy="20.6" r="1.35" fill={cheekInk} opacity="0.8" />
        <path d="M14.3 20.7 Q16 22.3 17.7 20.7" fill="none" stroke={palette.pageBackground} strokeWidth="1.2" strokeLinecap="round" />
      </g>
      {shades && (
        <g className="shades">
          <rect x="8.2" y="14.7" width="6.6" height="4.6" rx="1.6" fill="#10141f" />
          <rect x="17.2" y="14.7" width="6.6" height="4.6" rx="1.6" fill="#10141f" />
          <path d="M14.8 16.1 H17.2 M8.2 15.9 L6.2 14.9 M23.8 15.9 L25.8 14.9" stroke="#10141f" strokeWidth="1.1" fill="none" strokeLinecap="round" />
          <path d="M9.4 15.9 L11.2 15.9 M18.4 15.9 L20.2 15.9" stroke="#fff" strokeOpacity="0.6" strokeWidth="0.7" strokeLinecap="round" />
        </g>
      )}
    </Box>
  );
});
