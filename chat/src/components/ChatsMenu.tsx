// The chats as one button in the bar at the top: it says which chat is open and opens a dialog that holds them
// all with everything a person does to a chat, so the game below it carries no row of buttons.
import { memo, useCallback, useMemo, useState } from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogTitle from '@mui/material/DialogTitle';
import List from '@mui/material/List';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemText from '@mui/material/ListItemText';
import TextField from '@mui/material/TextField';
import Typography from '@mui/material/Typography';
import AddIcon from '@mui/icons-material/Add';
import ChatBubbleOutlineIcon from '@mui/icons-material/ChatBubbleOutlineOutlined';
import ContentCopyIcon from '@mui/icons-material/ContentCopy';
import DeleteOutlineIcon from '@mui/icons-material/DeleteOutlineOutlined';
import EditOutlinedIcon from '@mui/icons-material/EditOutlined';
import ExpandMoreIcon from '@mui/icons-material/ExpandMore';
import { byRecent, useChatStore } from '../store';
import ListItemIcon from '@mui/material/ListItemIcon';
import { styled } from '@mui/material/styles';
import { palette } from '../theme';
import { words } from '../mind/words';

// What the button asks the game to do, since the chat he read stands in the game and not here.
export const showChatEvent = 'mind-show-chat';

// A dot before a discussion, filled for the one open, as the rail marks it.
const Dot = styled('span')<{ open: boolean }>(({ open }) => ({
  width: 6,
  height: 6,
  borderRadius: '50%',
  border: `1px solid ${open ? palette.linkInk : palette.mutedInk}`,
  background: open ? palette.linkInk : 'transparent',
}));

export const ChatsMenu = memo(function ChatsMenu() {
  const chats = useChatStore((state) => state.chats);
  const game = useChatStore((state) => state.game);
  const pickGame = useChatStore((state) => state.pickGame);
  const renameChat = useChatStore((state) => state.renameChat);
  const copyChat = useChatStore((state) => state.copyChat);
  const deleteChat = useChatStore((state) => state.deleteChat);
  const recent = useMemo(() => byRecent(Object.values(chats)), [chats]);
  const chat = game === null ? null : chats[game] ?? null;
  const [shown, setShown] = useState(false);
  const [asked, setAsked] = useState<{ kind: 'rename' | 'copy' | 'delete'; text: string } | null>(null);
  const confirmAsked = useCallback(() => {
    if (!asked || !chat) return;
    if (asked.kind === 'rename') renameChat(chat.id, asked.text);
    if (asked.kind === 'copy') copyChat(chat.id, asked.text);
    if (asked.kind === 'delete') deleteChat(chat.id);
    setAsked(null);
  }, [asked, chat, renameChat, copyChat, deleteChat]);
  const ask = useCallback((kind: 'rename' | 'copy' | 'delete', text: string) => { setAsked({ kind, text }); setShown(false); }, []);
  return (
    <>
      <Button variant="outlined" onClick={() => setShown(true)} endIcon={<ExpandMoreIcon />} sx={{
        flex: 1, minWidth: 0, justifyContent: 'space-between', fontSize: '0.9375rem', borderRadius: 2,
        textTransform: 'none', color: '#fff', borderColor: palette.divider, py: 0.25,
        '& .MuiButton-endIcon': { ml: 0.5 },
      }} aria-haspopup="dialog" aria-label={words.games}>
        <Box component="span" sx={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{chat ? chat.title : words.newGame}</Box>
      </Button>
      <Dialog open={shown} onClose={() => setShown(false)} fullWidth maxWidth="xs" slotProps={{ paper: { sx: { borderRadius: 3 } } }}>
        <DialogTitle sx={{ display: 'flex', alignItems: 'center', gap: 1, fontSize: '1rem', fontWeight: 600, px: 2, py: 1 }}>
          <Box component="span" sx={{ flex: 1 }}>{words.games}</Box>
          <Button variant="outlined" startIcon={<AddIcon />} onClick={() => { pickGame(null); setShown(false); }} sx={{ textTransform: 'none', borderRadius: 2, px: 2, py: 0.75, fontSize: '1rem', fontWeight: 600, color: palette.linkInk, borderColor: palette.linkInk }}>{words.newGame}</Button>
        </DialogTitle>
        <DialogContent dividers sx={{ p: 0 }}>
          <List dense disablePadding>
            {recent.map((one) => (
              <ListItemButton key={one.id} selected={one.id === game} onClick={() => { pickGame(one.id); setShown(false); }}>
                <ListItemIcon sx={{ minWidth: (theme) => theme.spacing(3) }}><Dot open={one.id === game} /></ListItemIcon>
                <ListItemText primary={one.title} slotProps={{ primary: { sx: { fontSize: '1rem' }, noWrap: true } }} />
              </ListItemButton>
            ))}
            {recent.length === 0 && <ListItemText sx={{ px: 2, py: 1.5 }} primary={words.noGames} slotProps={{ primary: { sx: { color: 'text.secondary', fontSize: '0.9375rem' } } }} />}
          </List>
        </DialogContent>
        <DialogActions sx={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 0.5, px: 1.5, py: 1.25, '& > :not(style) ~ :not(style)': { ml: 0 } }}>
          <Button startIcon={<ChatBubbleOutlineIcon />} disabled={!chat} onClick={() => { window.dispatchEvent(new CustomEvent(showChatEvent)); setShown(false); }} sx={{ justifyContent: 'flex-start', textTransform: 'none', color: 'text.secondary' }}>{words.showChat}</Button>
          <Button startIcon={<EditOutlinedIcon />} disabled={!chat} onClick={() => chat && ask('rename', chat.title)} sx={{ justifyContent: 'flex-start', textTransform: 'none', color: 'text.secondary' }}>{words.rename}</Button>
          <Button startIcon={<ContentCopyIcon />} disabled={!chat} onClick={() => chat && ask('copy', words.copyOf(chat.title))} sx={{ justifyContent: 'flex-start', textTransform: 'none', color: 'text.secondary' }}>{words.saveAs}</Button>
          <Button startIcon={<DeleteOutlineIcon />} disabled={!chat} onClick={() => ask('delete', '')} sx={{ justifyContent: 'flex-start', textTransform: 'none', color: 'text.secondary' }}>{words.remove}</Button>
        </DialogActions>
      </Dialog>
      <Dialog open={asked !== null} onClose={() => setAsked(null)} fullWidth maxWidth="xs">
        <DialogTitle>{asked?.kind === 'rename' ? words.rename : asked?.kind === 'copy' ? words.saveAs : words.remove}</DialogTitle>
        <DialogContent>
          {asked?.kind === 'delete'
            ? <Typography>{words.removeAsk}</Typography>
            : <TextField autoFocus fullWidth margin="dense" label={words.name} value={asked?.text ?? ''} onChange={(event) => setAsked((was) => (was ? { ...was, text: event.target.value } : was))} />}
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setAsked(null)} sx={{ color: 'text.secondary' }}>{words.cancel}</Button>
          <Button onClick={confirmAsked} color={asked?.kind === 'delete' ? 'error' : 'primary'} disabled={asked !== null && asked.kind !== 'delete' && asked.text.trim() === ''}>
            {asked?.kind === 'rename' ? words.rename : asked?.kind === 'copy' ? words.saveAs : words.remove}
          </Button>
        </DialogActions>
      </Dialog>
    </>
  );
});
