import { memo, useCallback, useMemo, useState, type MouseEvent } from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Divider from '@mui/material/Divider';
import Menu from '@mui/material/Menu';
import MenuItem from '@mui/material/MenuItem';
import Typography from '@mui/material/Typography';
import CheckIcon from '@mui/icons-material/Check';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import { useShallow } from 'zustand/react/shallow';
import type { AgentStatus } from '../agent';
import { useChatStore } from '../store';
import { palette } from '../theme';

const model = {
  name: 'DAM1',
  loading: 'Loading',
  notLoaded: 'The model loads with your first message.',
    settingsLabel: 'Settings',
  engineLabel: 'Engine',
  effortLabel: 'Effort',
  effort: 'Max',
  effortNote: 'The network reads every word of every turn. There is no cheaper setting.',
} as const;

interface Row {
  title: string;
  value: string;
  note: string;
}

const counted = new Intl.NumberFormat('en-US');

function count(value: number): string {
  return counted.format(value);
}

function size(bytes: number): string {
  const megabytes = bytes / (1024 * 1024);
  return `${megabytes.toFixed(2)} MB`;
}

// What the engine holds and the numbers it runs with, as it reports them once loaded.
function rows(status: AgentStatus): { engine: Row[]; settings: Row[] } {
  const settings = status.settings;
  const engine: Row[] = [
    { title: 'Weights (count, size)', value: `${count(status.weightCount)} (${size(status.weightBytes)})`, note: 'The network over the stack: every number it learned.' },
    { title: 'Model (size)', value: size(status.modelBytes), note: 'The engine itself, compiled to WebAssembly: the cursor and its tree, apart from what the network learned.' },
    { title: 'Permanent state (nodes, size)', value: `${count(status.nodes)} (${size(status.stateBytes)})`, note: 'The tree every chat starts from, built from the facts and loaded whole when the engine starts.' },
  ];
  if (!settings) return { engine, settings: [] };
  return {
    engine: [...engine, { title: 'Steps', value: String(settings.classes.length), note: 'The step classes the network chooses between at every word.' }],
    settings: [
      { title: 'Steps per word', value: String(settings.steps), note: 'The most steps the network takes at one word before it must continue.' },
      { title: 'Stack', value: `${settings.slots} items`, note: 'How many of the newest stack items the network reads.' },
      { title: 'Hidden units', value: String(settings.hidden), note: 'The units between the stack and the steps.' },
    ],
  };
}

const SettingRow = memo(function SettingRow({ row }: { row: Row }) {
  return (
    <Box component="li" sx={{ listStyle: 'none', px: 2, py: 0.75, display: 'flex', justifyContent: 'space-between', gap: 2 }}>
      <Typography sx={{ fontSize: '0.9375rem' }}>{row.title}</Typography>
      <Typography sx={{ fontSize: '0.9375rem', color: 'text.secondary', whiteSpace: 'nowrap' }}>{row.value}</Typography>
    </Box>
  );
});

const GroupLabel = memo(function GroupLabel({ text }: { text: string }) {
  return (
    <Typography sx={{ px: 2, pt: 1.5, pb: 0.5, fontSize: '0.75rem', color: 'text.secondary', letterSpacing: '0.04em', textTransform: 'uppercase' }}>
      {text}
    </Typography>
  );
});

export const ModelMenu = memo(function ModelMenu() {
  const status = useChatStore(useShallow((state) => state.agent));
  const [anchor, setAnchor] = useState<HTMLElement | null>(null);
  const open = useCallback((event: MouseEvent<HTMLElement>) => setAnchor(event.currentTarget), []);
  const close = useCallback(() => setAnchor(null), []);
  const listed = useMemo(() => rows(status), [status]);
  const summary = model.effort;

  return (
    <>
      <Button
        onClick={open}
        size="small"
        endIcon={<ExpandMoreIcon sx={{ color: 'text.secondary' }} />}
        aria-haspopup="menu"
        aria-expanded={anchor ? 'true' : undefined}
        sx={{ color: 'text.primary', px: 1.25, minWidth: 0, background: anchor ? palette.hover : 'transparent', '&:hover': { background: palette.hover } }}
      >
        <Box component="span" sx={{ fontWeight: 500 }}>{model.name}</Box>
        <Box component="span" sx={{ color: 'text.secondary', ml: 1 }}>{summary}</Box>
      </Button>
      <Menu
        anchorEl={anchor}
        open={anchor !== null}
        onClose={close}
        anchorOrigin={{ vertical: 'top', horizontal: 'right' }}
        transformOrigin={{ vertical: 'bottom', horizontal: 'right' }}
        slotProps={{
          paper: { sx: { width: 360, maxWidth: 'calc(100vw - 32px)', maxHeight: 'min(70vh, 720px)', border: `1px solid ${palette.divider}`, borderRadius: 3, mb: 1 } },
          list: { dense: true, sx: { py: 1 } },
        }}
      >
        <MenuItem selected onClick={close} sx={{ alignItems: 'flex-start', py: 1 }}>
          <Box sx={{ flex: 1 }}>
            <Typography sx={{ fontSize: '1.0625rem' }}>{model.name}</Typography>
          </Box>
          <CheckIcon fontSize="small" sx={{ color: palette.linkInk, mt: 0.5 }} />
        </MenuItem>
        <Divider sx={{ my: 0.5 }} />
        <SettingRow row={{ title: model.effortLabel, value: model.effort, note: model.effortNote }} />
        <Divider sx={{ my: 0.5 }} />
        {status.ready && <GroupLabel text={model.engineLabel} />}
        {status.ready && listed.engine.map((row) => <SettingRow key={row.title} row={row} />)}
        {!status.ready && <SettingRow row={{ title: model.notLoaded, value: '', note: '' }} />}
        {listed.settings.length > 0 && <Divider sx={{ my: 0.5 }} />}
        {listed.settings.length > 0 && <GroupLabel text={model.settingsLabel} />}
        {listed.settings.map((row) => <SettingRow key={row.title} row={row} />)}
      </Menu>
    </>
  );
});
