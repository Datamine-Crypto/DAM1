// His house, drawn at the middle of the map.
import { memo } from 'react';
import Box from '@mui/material/Box';

// His little house in the middle of the map, where he goes after each sentence: drawn, not written.
export const House = memo(function House() {
  return (
    <Box component="svg" viewBox="0 0 132 118" sx={{ width: '100%', height: '100%', display: 'block', overflow: 'visible', '@keyframes smoke': { from: { transform: 'translateY(0)', opacity: 0.7 }, to: { transform: 'translateY(-14px)', opacity: 0 } } }}>
      <ellipse cx="66" cy="110" rx="58" ry="7" fill="rgba(0,0,0,0.35)" />
      <rect x="88" y="16" width="12" height="26" rx="2" fill="#7a4b3a" />
      <Box component="circle" cx="94" cy="10" r="4" fill="#dfe6f5" sx={{ animation: 'smoke 2.4s ease-out infinite' }} />
      <Box component="circle" cx="99" cy="4" r="3" fill="#dfe6f5" sx={{ animation: 'smoke 2.4s ease-out 1.2s infinite' }} />
      <rect x="24" y="52" width="84" height="56" rx="4" fill="#f4e3c1" />
      <path d="M12 58 L66 12 L120 58 Z" fill="#e2574c" stroke="#b63d35" strokeWidth="3" strokeLinejoin="round" />
      <rect x="54" y="70" width="24" height="38" rx="3" fill="#8a5a3c" />
      <circle cx="73" cy="90" r="2" fill="#ffd166" />
      <rect x="32" y="66" width="16" height="16" rx="2" fill="#9fe8ff" stroke="#fff" strokeWidth="2" />
      <path d="M40 66 V82 M32 74 H48" stroke="#fff" strokeWidth="1.5" />
      <rect x="84" y="66" width="16" height="16" rx="2" fill="#9fe8ff" stroke="#fff" strokeWidth="2" />
      <path d="M92 66 V82 M84 74 H100" stroke="#fff" strokeWidth="1.5" />
      <circle cx="66" cy="38" r="6" fill="#f4e3c1" stroke="#b63d35" strokeWidth="2" />
    </Box>
  );
});
