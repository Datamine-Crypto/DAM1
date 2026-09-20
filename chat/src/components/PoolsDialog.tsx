import { memo, useMemo, type MouseEvent } from 'react';
import useMediaQuery from '@mui/material/useMediaQuery';
import { useTheme } from '@mui/material/styles';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Chip from '@mui/material/Chip';
import Dialog from '@mui/material/Dialog';
import DialogContent from '@mui/material/DialogContent';
import DialogTitle from '@mui/material/DialogTitle';
import Divider from '@mui/material/Divider';
import IconButton from '@mui/material/IconButton';
import Typography from '@mui/material/Typography';
import CloseIcon from '@mui/icons-material/Close';
import OpenInNewIcon from '@mui/icons-material/OpenInNew';
import VerifiedUserIcon from '@mui/icons-material/VerifiedUser';
import { styled } from '@mui/material/styles';
import { dollars, useLiquidityStore } from '../liquidity';
import { poolChains, type PoolChain, type PoolToken } from '../pools';
import { palette } from '../theme';

const words = {
  title: 'Our Liquidity Pools',
  note: 'Help us train DAM2 and expand into new features like coding, vision & speech. Our community progress is limited by liquidity.',
  liquidity: 'available liquidity',
  liquidityUnknown: 'liquidity not read yet',
  audited: 'All Tokens Audited',
  auditsNote: 'Read the security audits',
  auditsHref: 'https://github.com/Datamine-Crypto/white-paper/tree/master/audits',
  trade: 'Trade on',
  hot: 'HOT',
  close: 'Close',
  learn: 'Learn More',
  dashboardNote: 'Solutions to inflation always required a centralized authority. We have finally solved monetary inflation by decentralizing it on Ethereum.',
  dashboardHref: 'https://datamine-crypto.github.io/dashboard/',
} as const;

const ChainHeading = styled(Typography)(({ theme }) => ({
  display: 'flex',
  alignItems: 'center',
  gap: theme.spacing(1),
  color: theme.palette.text.secondary,
  fontSize: '0.9375rem',
  paddingBottom: theme.spacing(1),
}));

const TokenRow = styled('div')(({ theme }) => ({
  display: 'flex',
  alignItems: 'center',
  flexWrap: 'wrap',
  gap: theme.spacing(2),
  padding: theme.spacing(1, 0),
}));

const Hot = styled('span')(({ theme }) => ({
  fontSize: '0.6875rem',
  fontWeight: 600,
  letterSpacing: '0.04em',
  padding: theme.spacing(0.25, 1),
  borderRadius: 999,
  border: `1px solid ${palette.outlinedButtonBorder}`,
  color: theme.palette.text.secondary,
}));

const tradeSx = {
  color: palette.linkInk,
  borderColor: palette.linkInk,
  width: { xs: '100%', sm: 196 },
  flexShrink: 0,
  whiteSpace: 'nowrap',
  '&:hover': { borderColor: palette.linkInk, background: palette.hover },
} as const;

const TokenLine = memo(function TokenLine({ token, liquidity }: { token: PoolToken; liquidity: number | undefined }) {
  return (
    <TokenRow>
      <img src={token.icon} alt="" width={36} height={36} style={{ borderRadius: '50%' }} />
      <Box sx={{ flex: 1, minWidth: 0 }}>
        <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, fontSize: '1.0625rem' }}>
          <span>{token.name}</span>
          {token.hot && <Hot>{words.hot}</Hot>}
        </Box>
        <Typography sx={{ color: 'text.secondary', fontSize: '0.8125rem' }}>
          {liquidity === undefined ? words.liquidityUnknown : `${dollars.format(liquidity)} ${words.liquidity}`}
        </Typography>
      </Box>
      <Button component="a" variant="outlined" sx={tradeSx} href={token.trade} target="_blank" rel="noopener noreferrer"
        startIcon={<img src={token.venue.icon} alt="" width={18} height={18} style={{ borderRadius: '50%' }} />}
        onClick={(event: MouseEvent<HTMLElement>) => event.stopPropagation()}>
        {words.trade} {token.venue.name}
      </Button>
    </TokenRow>
  );
});

const ChainBlock = memo(function ChainBlock({ chain, liquidity }: { chain: PoolChain; liquidity: Map<string, number> }) {
  return (
    <Box sx={{ py: 1.5 }}>
      <ChainHeading>
        <img src={chain.icon} alt="" width={16} height={16} />
        <span>{chain.name}:</span>
      </ChainHeading>
      {chain.tokens.map((token) => <TokenLine key={token.name} token={token} liquidity={liquidity.get(token.name)} />)}
    </Box>
  );
});

export interface PoolsDialogProps {
  open: boolean;
  onClose: () => void;
}

export const PoolsDialog = memo(function PoolsDialog({ open, onClose }: PoolsDialogProps) {
  const theme = useTheme();
  const phone = useMediaQuery(theme.breakpoints.down('sm'));
  const tokens = useLiquidityStore((state) => state.tokens);
  const liquidity = useMemo(() => new Map(tokens.map((token) => [token.token, token.dollars])), [tokens]);
  return (
    <Dialog open={open} onClose={onClose} maxWidth="sm" fullWidth fullScreen={phone}
      slotProps={{ paper: { sx: { border: `1px solid ${palette.divider}`, borderRadius: phone ? 0 : 3, backgroundImage: 'none' } } }}>
      <DialogTitle sx={{ display: 'flex', flexDirection: 'column', gap: 1, px: { xs: 2, sm: 3 }, pt: 2, pb: 1.5, pr: 6, fontSize: '1.0625rem' }}>
        <Box sx={{ display: 'flex', alignItems: 'center', flexWrap: 'wrap', gap: 1.5 }}>
          <Box component="span" sx={{ fontWeight: 600 }}>{words.title}</Box>
          <Chip
            component="a"
            href={words.auditsHref}
            target="_blank"
            rel="noopener noreferrer"
            clickable
            size="small"
            icon={<VerifiedUserIcon sx={{ color: `${palette.mutedInk} !important` }} />}
            label={words.audited}
            title={words.auditsNote}
            sx={{ border: `1px solid ${palette.outlinedButtonBorder}`, background: 'transparent', color: palette.mutedInk, fontWeight: 400, '&:hover': { color: palette.ink } }}
          />
        </Box>
        <Box component="span" sx={{ color: 'text.secondary', fontSize: '0.875rem', fontWeight: 400, lineHeight: 1.4 }}>{words.note}</Box>
      </DialogTitle>
      <IconButton aria-label={words.close} onClick={onClose} sx={{ position: 'absolute', right: 8, top: 8, color: 'text.secondary' }}>
        <CloseIcon fontSize="small" />
      </IconButton>
      <Divider />
      <DialogContent sx={{ px: { xs: 2, sm: 3 }, pt: 1.5, pb: 2 }}>
        {poolChains.map((chain, at) => (
          <Box key={chain.name}>
            {at > 0 && <Divider />}
            <ChainBlock chain={chain} liquidity={liquidity} />
          </Box>
        ))}
      </DialogContent>
      <Divider />
      <Box sx={{ px: { xs: 2, sm: 3 }, py: 2.5, display: 'flex', flexDirection: 'column', alignItems: 'flex-start', gap: 1 }}>
        <Typography sx={{ color: 'text.secondary', fontSize: '0.875rem' }}>{words.dashboardNote}</Typography>
        <Button component="a" href={words.dashboardHref} target="_blank" rel="noopener noreferrer" variant="text"
          endIcon={<OpenInNewIcon fontSize="small" />} sx={{ color: palette.linkInk, whiteSpace: 'nowrap' }}>
          {words.learn}
        </Button>
      </Box>
    </Dialog>
  );
});
