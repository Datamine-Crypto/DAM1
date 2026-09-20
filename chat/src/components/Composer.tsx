import { memo, useState, type KeyboardEvent } from 'react';
import Box from '@mui/material/Box';
import CircularProgress from '@mui/material/CircularProgress';
import InputBase from '@mui/material/InputBase';
import IconButton from '@mui/material/IconButton';
import Tooltip from '@mui/material/Tooltip';
import ArrowUpwardIcon from '@mui/icons-material/ArrowUpward';
import LockOutlinedIcon from '@mui/icons-material/LockOutlined';
import { styled } from '@mui/material/styles';
import { useShallow } from 'zustand/react/shallow';
import { useChatStore } from '../store';
import { palette } from '../theme';
import { ModelMenu } from './ModelMenu';

const Frame = styled('form')(({ theme }) => ({
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(0.5),
  padding: theme.spacing(1.5, 1.5, 1, 2),
  borderRadius: 14,
  background: palette.paper,
  border: `1px solid ${palette.divider}`,
  boxShadow: `0 8px 24px ${palette.surfaceShadow}`,
  '&:focus-within': { borderColor: palette.outlinedButtonBorder },
}));

const Controls = styled('div')(({ theme }) => ({
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'flex-end',
  gap: theme.spacing(1),
}));

// The lock says where the chat stays: in this browser, with nothing sent anywhere.
const privateNote = 'Private: DAM1 LLM runs inside your browser and uses local storage.';

export interface ComposerProps {
  placeholder: string;
  disabled: boolean;
  onSend: (text: string) => void;
  autoFocus?: boolean;
  tall?: boolean;
}

export const Composer = memo(function Composer({ placeholder, disabled, onSend, autoFocus = false, tall = false }: ComposerProps) {
  const [text, setText] = useState('');
  const busy = useChatStore(useShallow((state) => state.booting || state.thinking));
  const ready = text.trim().length > 0 && !disabled;

  const submit = (): void => {
    if (!ready) return;
    onSend(text.trim());
    setText('');
  };

  const onKeyDown = (event: KeyboardEvent<HTMLTextAreaElement | HTMLInputElement>): void => {
    if (event.key === 'Enter' && !event.shiftKey) {
      event.preventDefault();
      submit();
    }
  };

  return (
    <Frame onSubmit={(event) => { event.preventDefault(); submit(); }}>
      <InputBase
        multiline
        minRows={tall ? 2 : 1}
        maxRows={8}
        fullWidth
        autoFocus={autoFocus}
        placeholder={placeholder}
        value={text}
        onChange={(event) => setText(event.target.value)}
        onKeyDown={onKeyDown}
        inputProps={{ 'aria-label': 'Message', spellCheck: false }}
        startAdornment={(
          <Tooltip title={privateNote} placement="bottom-start" slotProps={{ tooltip: { sx: { maxWidth: 'none', whiteSpace: 'nowrap', ml: -1 } } }}>
            <LockOutlinedIcon aria-label={privateNote} sx={{ fontSize: 16, color: 'text.secondary', mr: 1, alignSelf: 'flex-start', mt: '0.25em', cursor: 'default' }} />
          </Tooltip>
        )}
        sx={{ fontSize: '1rem', py: 0.5, alignItems: 'flex-start' }}
      />
      <Controls>
        <ModelMenu />
        <Tooltip title="Send (Enter)">
          <Box component="span">
            <IconButton
              type="submit"
              aria-label="Send"
              disabled={!ready}
              size="small"
              sx={{
                background: ready ? palette.linkInk : palette.userBubble,
                color: ready ? palette.pageBackground : palette.mutedInk,
                '&:hover': { background: ready ? palette.linkInk : palette.userBubble },
                '&.Mui-disabled': { background: palette.userBubble, color: palette.mutedInk },
              }}
            >
              {busy ? <CircularProgress size={16} thickness={5} sx={{ color: palette.mutedInk }} /> : <ArrowUpwardIcon fontSize="small" />}
            </IconButton>
          </Box>
        </Tooltip>
      </Controls>
    </Frame>
  );
});
