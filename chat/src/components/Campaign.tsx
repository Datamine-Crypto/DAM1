import { memo, useCallback, useEffect, useState, type KeyboardEvent } from 'react';
import Box from '@mui/material/Box';
import LinearProgress from '@mui/material/LinearProgress';
import Typography from '@mui/material/Typography';
import { styled } from '@mui/material/styles';
import { useShallow } from 'zustand/react/shallow';
import { currentMilestone, dollars, progress, useLiquidityStore } from '../liquidity';
import { fonts, palette } from '../theme';
import { PoolsDialog } from './PoolsDialog';

const words = {
  title: 'We’re fundraising liquidity for DAM1 Improvements and DAM2 LLM',
  next: 'Next milestone',
  done: 'Every milestone reached',
  learn: 'Learn More',
} as const;

const Card = styled('section')(({ theme }) => ({
  width: '100%',
  maxWidth: 560,
  margin: '0 auto',
  padding: theme.spacing(2, 2.5, 2.5),
  [theme.breakpoints.down('sm')]: { padding: theme.spacing(2, 2, 2.5) },
  borderRadius: 16,
  background: palette.campaignBanner,
  border: `1px solid ${palette.divider}`,
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(1.5),
  cursor: 'pointer',
  transition: 'border-color 120ms',
  '&:hover, &:focus-visible': { borderColor: palette.outlinedButtonBorder, outline: 'none' },
}));

const Bar = styled(LinearProgress)({
  height: 6,
  borderRadius: 3,
  backgroundColor: palette.sunk,
  '& .MuiLinearProgress-bar': { borderRadius: 4, backgroundColor: palette.linkInk, opacity: 0.6 },
});

const IconBadge = styled('span')({
  display: 'inline-flex',
  alignItems: 'center',
  justifyContent: 'center',
  width: 34,
  height: 34,
  borderRadius: '50%',
  background: palette.paper,
  flexShrink: 0,
});

// Opens the pools dialog on click, Enter or Space, so the whole card is one control.
export function useOpensPools(): { open: boolean; show: () => void; hide: () => void; onKeyDown: (event: KeyboardEvent<HTMLElement>) => void } {
  const [open, setOpen] = useState(false);
  const show = useCallback(() => setOpen(true), []);
  const hide = useCallback(() => setOpen(false), []);
  const onKeyDown = useCallback((event: KeyboardEvent<HTMLElement>) => {
    if (event.key === 'Enter' || event.key === ' ') {
      event.preventDefault();
      setOpen(true);
    }
  }, []);
  return { open, show, hide, onKeyDown };
}

export const Campaign = memo(function Campaign() {
  const { total, loadedAt, load } = useLiquidityStore(useShallow((state) => ({
    total: state.total, loadedAt: state.loadedAt, load: state.load,
  })));
  const { open, show, hide, onKeyDown } = useOpensPools();

  useEffect(() => { load(); }, [load]);

  const loaded = loadedAt > 0;
  const milestone = currentMilestone(total);
  const share = progress(total);

  return (
    <>
      <Card role="button" tabIndex={0} aria-label={words.title} aria-haspopup="dialog" onClick={show} onKeyDown={onKeyDown}>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', gap: 2, flexWrap: 'wrap' }}>
          <Box sx={{ display: 'flex', alignItems: 'center', gap: 1.5, minWidth: 0 }}>
            <IconBadge><img src="/images/ethereum.svg" alt="" width={20} height={20} /></IconBadge>
            <Typography component="h2" sx={{ fontFamily: fonts.serif, fontSize: '1.125rem', fontWeight: 400, lineHeight: 1.3, color: '#d7dbe8' }}>{words.title}</Typography>
          </Box>
          <Typography component="span" sx={{ color: 'text.secondary', fontSize: '0.8125rem', textDecoration: 'underline', textUnderlineOffset: 3 }}>{words.learn}</Typography>
        </Box>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'flex-end', gap: 2, flexWrap: 'wrap' }}>
          <Typography sx={{ fontSize: '1.75rem', fontWeight: 400, lineHeight: 1, letterSpacing: '-0.01em', color: '#d7dbe8' }}>
            {loaded ? dollars.format(total) : ' '}
          </Typography>
          <Box sx={{ textAlign: 'right' }}>
            <Typography sx={{ color: 'text.secondary', fontSize: '0.75rem', letterSpacing: '0.04em', textTransform: 'uppercase' }}>
              {milestone.reached ? words.done : words.next}
            </Typography>
            {!milestone.reached && (
              <Typography sx={{ fontSize: '1rem', fontWeight: 500, lineHeight: 1.2 }}>{dollars.format(milestone.target)}</Typography>
            )}
          </Box>
        </Box>
        <Bar variant="determinate" value={loaded ? share * 100 : 0} aria-label={words.next} />
      </Card>
      <PoolsDialog open={open} onClose={hide} />
    </>
  );
});
