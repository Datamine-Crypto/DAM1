import { memo } from 'react';
import Box from '@mui/material/Box';
import Typography from '@mui/material/Typography';
import { fonts } from '../theme';
import { Campaign } from './Campaign';
import { Composer } from './Composer';
import { Mascot } from './Mark';

export interface LandingProps {
  greeting: string;
  placeholder: string;
  disabled: boolean;
  onSend: (text: string) => void;
}

export const Landing = memo(function Landing({ greeting, placeholder, disabled, onSend }: LandingProps) {
  return (
    <Box sx={{ flex: 1, overflowY: 'auto', display: 'flex', flexDirection: 'column', alignItems: 'center', px: 2, py: { xs: 3, sm: 4 }, pb: 'calc(24px + env(safe-area-inset-bottom))' }}>
      <Box sx={{ width: '100%', maxWidth: 720, display: 'flex', flexDirection: 'column', gap: 3, my: 'auto' }}>
        <Campaign />
        <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: 1.5, flexWrap: 'wrap', pt: 4 }}>
          <Mascot size={30} />
          <Typography component="h1" sx={{ fontFamily: fonts.serif, fontSize: { xs: '1.75rem', sm: '2.25rem' }, fontWeight: 400, letterSpacing: '-0.01em', textAlign: 'center' }}>
            {greeting}
          </Typography>
        </Box>
        <Composer placeholder={placeholder} disabled={disabled} onSend={onSend} autoFocus tall />
      </Box>
    </Box>
  );
});
