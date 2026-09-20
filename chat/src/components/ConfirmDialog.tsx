import { memo, useRef } from 'react';
import Button from '@mui/material/Button';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogTitle from '@mui/material/DialogTitle';
import Typography from '@mui/material/Typography';
import { palette } from '../theme';

const words = {
  cancel: 'Cancel',
} as const;

const paperSx = { border: `1px solid ${palette.divider}`, borderRadius: 3, backgroundImage: 'none' } as const;
const dangerInk = '#ff6b6b';
const dangerSx = { color: dangerInk, borderColor: dangerInk, '&:hover': { borderColor: dangerInk, background: 'rgba(255, 107, 107, 0.08)' } } as const;

export interface ConfirmDialogProps {
  open: boolean;
  title: string;
  body: string;
  confirm: string;
  onClose: () => void;
  onConfirm: () => void;
}

// A question before something that cannot be undone. The confirming button is the danger colour.
export const ConfirmDialog = memo(function ConfirmDialog({ open, title, body, confirm, onClose, onConfirm }: ConfirmDialogProps) {
  const confirmRef = useRef<HTMLButtonElement>(null);
  return (
    <Dialog open={open} onClose={onClose} maxWidth="xs" fullWidth
      slotProps={{ paper: { sx: paperSx }, transition: { onEntered: () => confirmRef.current?.focus() } }}>
      <DialogTitle sx={{ fontSize: '1rem', pb: 1 }}>{title}</DialogTitle>
      <DialogContent>
        <Typography sx={{ color: 'text.secondary', fontSize: '0.9375rem', lineHeight: 1.5 }}>{body}</Typography>
      </DialogContent>
      <DialogActions sx={{ px: 3, pb: 2 }}>
        <Button onClick={onClose} sx={{ color: 'text.secondary' }}>{words.cancel}</Button>
        <Button ref={confirmRef} onClick={() => { onConfirm(); onClose(); }} variant="outlined" sx={dangerSx}>{confirm}</Button>
      </DialogActions>
    </Dialog>
  );
});
