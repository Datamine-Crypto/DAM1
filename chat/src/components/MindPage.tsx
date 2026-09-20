// The DAM1 Mind Explorer: the game, a conversation played back on a map, and nothing else for now.
import { memo, type ReactNode } from 'react';
import Box from '@mui/material/Box';
import { MindGame } from './MindGame';

export const MindPage = memo(function MindPage({ footnote }: { footnote: ReactNode }) {
  return (
    <Box sx={{ flex: 1, minHeight: 0, display: 'flex', flexDirection: 'column' }}>
      <Box sx={{ flex: 1, minHeight: 0, width: '100%', display: 'flex', flexDirection: 'column' }}>
        <MindGame footnote={footnote} />
      </Box>
    </Box>
  );
});
