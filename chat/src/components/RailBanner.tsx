import { memo, useEffect } from 'react';
import Box from '@mui/material/Box';
import LinearProgress from '@mui/material/LinearProgress';
import Typography from '@mui/material/Typography';
import { styled } from '@mui/material/styles';
import { useShallow } from 'zustand/react/shallow';
import { dollars, progress, useLiquidityStore } from '../liquidity';
import { palette } from '../theme';
import { useOpensPools } from './Campaign';
import { PoolsDialog } from './PoolsDialog';

const fundInk = '#4ade80';

const words = {
  title: 'We’re fundraising liquidity for DAM1 Improvements and DAM2 LLM',
  rail: 'We’re fundraising liquidity for DAM1 & DAM2 LLM',
  short: 'We’re fundraising liquidity',
} as const;

const Banner = styled('div')(({ theme }) => ({
  margin: theme.spacing(1.5, 1.5, 1.5),
  padding: theme.spacing(2, 2, 2.25),
  borderRadius: 10,
  background: palette.campaignBanner,
  border: `1px solid ${palette.divider}`,
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(1.25),
  cursor: 'pointer',
  transition: 'border-color 120ms',
  '&:hover, &:focus-visible': { borderColor: palette.linkInk, outline: 'none' },
}));

const ThinBar = styled(LinearProgress)({
  height: 6,
  borderRadius: 3,
  backgroundColor: palette.sunk,
  '& .MuiLinearProgress-bar': { borderRadius: 3, backgroundColor: palette.linkInk },
});

// The sign of the chain, on a round face of its own so it stands out of the strip behind it.
const Mark = styled('span', { shouldForwardProp: (name) => name !== 'size' })<{ size: number }>(({ size }) => ({
  display: 'inline-flex',
  alignItems: 'center',
  justifyContent: 'center',
  width: size,
  height: size,
  borderRadius: '50%',
  background: 'rgba(10, 22, 16, 0.85)',
  flexShrink: 0,
}));

// The campaign in the rail, for pages where the card is not on screen.
export const RailBanner = memo(function RailBanner() {
  const { total, loadedAt, load } = useLiquidityStore(useShallow((state) => ({
    total: state.total, loadedAt: state.loadedAt, load: state.load,
  })));
  const { open, show, hide, onKeyDown } = useOpensPools();

  useEffect(() => { if (loadedAt === 0) load(); }, [loadedAt, load]);

  const loaded = loadedAt > 0;

  return (
    <>
      <Banner role="button" tabIndex={0} aria-label={words.rail} aria-haspopup="dialog" onClick={show} onKeyDown={onKeyDown}>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
          <Mark size={24}><img src="/images/ethereum.svg" alt="" width={14} height={14} /></Mark>
          <Typography sx={{ fontSize: '0.8125rem', fontWeight: 500 }}>{words.rail}</Typography>
        </Box>
        <Typography sx={{ fontSize: '1.25rem', fontWeight: 500, lineHeight: 1.1 }}>
          {loaded ? dollars.format(total) : ' '}
        </Typography>
        <ThinBar variant="determinate" value={loaded ? progress(total) * 100 : 0} aria-label={words.rail} />
      </Banner>
      <PoolsDialog open={open} onClose={hide} />
    </>
  );
});

// The campaign as a strip across the top of the page: always there, on every page, with no way to close it.
export const TopBanner = memo(function TopBanner({ inset = false }: { inset?: boolean }) {
  const { total, loadedAt, load } = useLiquidityStore(useShallow((state) => ({
    total: state.total, loadedAt: state.loadedAt, load: state.load,
  })));
  const { open, show, hide, onKeyDown } = useOpensPools();
  useEffect(() => { if (loadedAt === 0) load(); }, [loadedAt, load]);
  const loaded = loadedAt > 0;
  const filled = loaded ? Math.round(progress(total) * 100) : 0;
  return (
    <>
      <Box role="alert" tabIndex={0} aria-label={words.title} aria-haspopup="dialog" onClick={show} onKeyDown={onKeyDown} sx={{
        display: 'flex', alignItems: 'center', justifyContent: 'flex-start', gap: { xs: 1, md: 2 }, flexWrap: 'wrap', mx: inset ? 0 : 2, mt: inset ? 0 : 1.5, px: { xs: 1.5, md: 3 }, py: { xs: 0.75, md: 1.25 }, cursor: 'pointer', flexShrink: 0, borderRadius: 2,
        // The strip itself is the progress bar: it is filled from the left as far as the campaign has come.
        background: `linear-gradient(90deg, rgba(74, 222, 128, 0.38) ${filled}%, rgba(14, 30, 22, 0.88) ${filled}%)`,
        border: `2px dashed ${fundInk}`, boxShadow: '0 6px 18px rgba(0, 0, 0, 0.35)', '&:hover, &:focus-visible': { filter: 'brightness(1.12)', outline: 'none' },
      }}>
        <Mark size={28}><img src="/images/ethereum.svg" alt="" width={16} height={16} /></Mark>
        <Typography sx={{ fontSize: { xs: '0.8125rem', md: '1.125rem' }, fontWeight: 600, color: '#fff', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
          <Box component="span" sx={{ display: { xs: 'inline', md: 'none' } }}>{words.short}</Box>
          <Box component="span" sx={{ display: { xs: 'none', md: 'inline' } }}>{words.rail}</Box>
        </Typography>
        <Typography sx={{ fontSize: { xs: '0.9375rem', md: '1.25rem' }, fontWeight: 700, ml: 'auto' }}>{loaded ? dollars.format(total) : ' '}</Typography>
      </Box>
      <PoolsDialog open={open} onClose={hide} />
    </>
  );
});
