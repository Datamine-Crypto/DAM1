import { memo, useCallback, useMemo, useRef, useState, type KeyboardEvent, type MouseEvent } from 'react';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogTitle from '@mui/material/DialogTitle';
import Divider from '@mui/material/Divider';
import IconButton from '@mui/material/IconButton';
import List from '@mui/material/List';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemIcon from '@mui/material/ListItemIcon';
import ListItemText from '@mui/material/ListItemText';
import Menu from '@mui/material/Menu';
import MenuItem from '@mui/material/MenuItem';
import TextField from '@mui/material/TextField';
import Tooltip from '@mui/material/Tooltip';
import Typography from '@mui/material/Typography';
import AddIcon from '@mui/icons-material/Add';
import DeleteOutlinedIcon from '@mui/icons-material/DeleteOutlined';
import EditOutlinedIcon from '@mui/icons-material/EditOutlined';
import MoreVertIcon from '@mui/icons-material/MoreVert';
import PushPinOutlinedIcon from '@mui/icons-material/PushPinOutlined';
import { styled } from '@mui/material/styles';
import { useShallow } from 'zustand/react/shallow';
import { byRecent, useChatStore, type Chat } from '../store';
import { palette } from '../theme';
import { ConfirmDialog } from './ConfirmDialog';
import { RailBanner } from './RailBanner';

const words = {
  brand: 'Datamine Network',
  newChat: 'New',
  pinned: 'Pinned',
  chats: 'Chats',
  none: 'No chats yet.',
  more: 'More',
  pin: 'Pin',
  unpin: 'Unpin',
  rename: 'Rename',
  edit: 'Edit details',
  archive: 'Archive',
  unarchive: 'Unarchive',
  delete: 'Delete',
  renameTitle: 'Rename chat',
  renameLabel: 'Title',
  cancel: 'Cancel',
  privacy: 'Privacy Policy',
  save: 'Save',
  deleteChatTitle: 'Delete chat?',
  deleteChatBody: (title: string) => `“${title}” will be permanently deleted. This cannot be undone.`,
} as const;

// The same invitation the analytics site's button carries.
const discord = {
  label: 'Chat With Us On Discord!',
  note: 'Talk with our developers. Come and say hello!',
  href: 'https://discord.gg/2dQ7XAB22u',
} as const;

// Where the model is published, so anyone can download and run it outside the page.
const huggingFace = {
  label: 'DAM1 On Hugging Face',
  note: 'Download the model and run it yourself.',
  href: 'https://huggingface.co/DatamineNetwork/DAM1',
} as const;

// The links at the foot of the rail: stacked in one column, each as wide as the widest.
const linkButtonSx = { background: palette.discordButton, color: palette.ink, fontSize: '0.65rem', whiteSpace: 'nowrap', px: 1.5, textTransform: 'uppercase', '&:hover': { background: palette.discordButtonHover } } as const;

const deleteInk = '#ff6b6b';
const menuPaperSx = { minWidth: 180, border: `1px solid ${palette.divider}`, borderRadius: 2.5, mt: 0.5 } as const;

const Brand = styled('a')(({ theme }) => ({
  display: 'flex',
  alignItems: 'center',
  gap: theme.spacing(1.5),
  padding: theme.spacing(2, 2, 1.5),
  color: theme.palette.text.primary,
  textDecoration: 'none',
  fontSize: '1.25rem',
  fontWeight: 500,
}));

const SectionRow = styled('div')(({ theme }) => ({
  display: 'flex',
  alignItems: 'center',
  justifyContent: 'space-between',
  padding: theme.spacing(2, 1, 0.5, 2),
  color: theme.palette.text.secondary,
  fontSize: '0.75rem',
}));

const Dot = styled('span')<{ active: boolean }>(({ active }) => ({
  width: 6,
  height: 6,
  borderRadius: '50%',
  border: `1px solid ${active ? palette.linkInk : palette.mutedInk}`,
  background: active ? palette.linkInk : 'transparent',
}));

// A touch screen has no hover, so the menu button stays visible there.
const rowSx = (active: boolean) => ({ pr: 0.5, '& .more': { opacity: active ? 1 : 0 }, '&:hover .more, &:focus-within .more': { opacity: 1 }, '@media (hover: none)': { '& .more': { opacity: 1 } } });

const MoreButton = memo(function MoreButton({ onOpen }: { onOpen: (anchor: HTMLElement) => void }) {
  return (
    <IconButton className="more" size="small" aria-label={words.more} aria-haspopup="menu"
      onClick={(event: MouseEvent<HTMLElement>) => { event.stopPropagation(); onOpen(event.currentTarget); }} sx={{ color: 'text.secondary' }}>
      <MoreVertIcon fontSize="inherit" />
    </IconButton>
  );
});

const RenameDialog = memo(function RenameDialog({ chat, onClose, onRename }: { chat: Chat | null; onClose: () => void; onRename: (id: string, title: string) => void }) {
  const [title, setTitle] = useState('');
  const titleRef = useRef<HTMLInputElement>(null);
  const save = (): void => {
    if (chat && title.trim()) onRename(chat.id, title);
    onClose();
  };
  const onKeyDown = (event: KeyboardEvent<HTMLDivElement>): void => {
    if (event.key === 'Enter') {
      event.preventDefault();
      save();
    }
  };
  return (
    <Dialog open={chat !== null} onClose={onClose} maxWidth="xs" fullWidth
      slotProps={{ paper: { sx: { border: `1px solid ${palette.divider}`, borderRadius: 3, backgroundImage: 'none' } }, transition: { onEnter: () => setTitle(chat?.title ?? ''), onEntered: () => titleRef.current?.focus() } }}>
      <DialogTitle sx={{ fontSize: '1rem' }}>{words.renameTitle}</DialogTitle>
      <DialogContent>
        <TextField inputRef={titleRef} fullWidth label={words.renameLabel} value={title} onChange={(event) => setTitle(event.target.value)}
          onKeyDown={onKeyDown} slotProps={{ htmlInput: { spellCheck: false } }} sx={{ mt: 1 }} />
      </DialogContent>
      <DialogActions sx={{ px: 3, pb: 2 }}>
        <Button onClick={onClose} sx={{ color: 'text.secondary' }}>{words.cancel}</Button>
        <Button onClick={save} variant="outlined" disabled={!title.trim()} sx={{ color: palette.linkInk, borderColor: palette.linkInk }}>{words.save}</Button>
      </DialogActions>
    </Dialog>
  );
});

const ChatRow = memo(function ChatRow({ chat, active, onOpen, onMenu }: {
  chat: Chat; active: boolean; onOpen: (id: string) => void; onMenu: (chat: Chat, anchor: HTMLElement) => void;
}) {
  const open = useCallback((anchor: HTMLElement) => onMenu(chat, anchor), [chat, onMenu]);
  return (
    <ListItemButton dense selected={active} onClick={() => onOpen(chat.id)} sx={rowSx(active)}>
      <ListItemIcon sx={{ minWidth: (theme) => theme.spacing(3) }}><Dot active={active} /></ListItemIcon>
      <ListItemText primary={chat.title} slotProps={{ primary: { noWrap: true, sx: { fontSize: '0.875rem' } } }} />
      <MoreButton onOpen={open} />
    </ListItemButton>
  );
});


export const Sidebar = memo(function Sidebar({ onNavigate }: { onNavigate: () => void }) {
  const chats = useChatStore(useShallow((state) => Object.values(state.chats)));
  const route = useChatStore((state) => state.route);
  const game = useChatStore((state) => state.game);
  const actions = useChatStore(useShallow((state) => ({
    newChat: state.newChat, openMind: state.openMind, openPrivacy: state.openPrivacy,
    deleteChat: state.deleteChat, togglePin: state.togglePin, renameChat: state.renameChat,
  })));

  const [chatMenu, setChatMenu] = useState<{ chat: Chat; anchor: HTMLElement } | null>(null);
  const [renaming, setRenaming] = useState<Chat | null>(null);
  const [deletingChat, setDeletingChat] = useState<Chat | null>(null);
  const closeDeleteChat = useCallback(() => setDeletingChat(null), []);

  const onChatMenu = useCallback((chat: Chat, anchor: HTMLElement) => setChatMenu({ chat, anchor }), []);
  const closeChatMenu = useCallback(() => setChatMenu(null), []);
  const closeRename = useCallback(() => setRenaming(null), []);

  const onNew = useCallback(() => { actions.newChat(); onNavigate(); }, [actions, onNavigate]);
  const onOpenChat = useCallback((id: string) => { actions.openMind(id); onNavigate(); }, [actions, onNavigate]);
  const onOpenPrivacy = useCallback(() => { actions.openPrivacy(); onNavigate(); }, [actions, onNavigate]);

  const recentChats = useMemo(() => byRecent(chats), [chats]);
  const pinnedChats = useMemo(() => recentChats.filter((chat) => chat.pinned), [recentChats]);
  const otherChats = useMemo(() => recentChats.filter((chat) => !chat.pinned), [recentChats]);

  const activeChatId = route.kind === 'chat' ? route.id : route.kind === 'mind' ? game : null;

  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0 }}>
      <Brand href="/" onClick={(event) => { event.preventDefault(); onNew(); }}>
        <img src="/images/logo.svg" alt="" width={36} height={36} />
        <Box component="span" sx={{ whiteSpace: "nowrap", overflow: "hidden", textOverflow: "ellipsis" }}>{words.brand}</Box>
      </Brand>
      <List disablePadding sx={{ px: 1, pt: 1.5 }}>
        <ListItemButton onClick={onNew} dense selected={route.kind === 'home'}>
          <ListItemIcon sx={{ minWidth: (theme) => theme.spacing(4) }}><AddIcon fontSize="small" /></ListItemIcon>
          <ListItemText primary={words.newChat} />
        </ListItemButton>
      </List>
      <Box sx={{ flex: 1, overflowY: 'auto', minHeight: 0 }}>
        {pinnedChats.length > 0 && (
          <>
            <SectionRow><span>{words.pinned}</span></SectionRow>
            <List disablePadding sx={{ px: 1 }}>
              {pinnedChats.map((chat) => <ChatRow key={chat.id} chat={chat} active={chat.id === activeChatId} onOpen={onOpenChat} onMenu={onChatMenu} />)}
            </List>
          </>
        )}
        <SectionRow><span>{words.chats}</span></SectionRow>
        {otherChats.length === 0 && <Typography sx={{ color: 'text.secondary', fontSize: '0.8125rem', px: 2, py: 1 }}>{words.none}</Typography>}
        <List disablePadding sx={{ px: 1 }}>
          {otherChats.map((chat) => <ChatRow key={chat.id} chat={chat} active={chat.id === activeChatId} onOpen={onOpenChat} onMenu={onChatMenu} />)}
        </List>
      </Box>

      <Menu anchorEl={chatMenu?.anchor ?? null} open={chatMenu !== null} onClose={closeChatMenu}
        anchorOrigin={{ vertical: 'bottom', horizontal: 'right' }} transformOrigin={{ vertical: 'top', horizontal: 'right' }}
        slotProps={{ paper: { sx: menuPaperSx }, list: { dense: true } }}>
        <MenuItem onClick={() => { if (chatMenu) actions.togglePin(chatMenu.chat.id); closeChatMenu(); }}>
          <ListItemIcon><PushPinOutlinedIcon fontSize="small" /></ListItemIcon>
          <ListItemText>{chatMenu?.chat.pinned ? words.unpin : words.pin}</ListItemText>
        </MenuItem>
        <MenuItem onClick={() => { if (chatMenu) setRenaming(chatMenu.chat); closeChatMenu(); }}>
          <ListItemIcon><EditOutlinedIcon fontSize="small" /></ListItemIcon>
          <ListItemText>{words.rename}</ListItemText>
        </MenuItem>
        <Divider />
        <MenuItem onClick={() => { if (chatMenu) setDeletingChat(chatMenu.chat); closeChatMenu(); }} sx={{ color: deleteInk }}>
          <ListItemIcon><DeleteOutlinedIcon fontSize="small" sx={{ color: deleteInk }} /></ListItemIcon>
          <ListItemText>{words.delete}</ListItemText>
        </MenuItem>
      </Menu>



      <RenameDialog chat={renaming} onClose={closeRename} onRename={actions.renameChat} />
      <ConfirmDialog open={deletingChat !== null} title={words.deleteChatTitle} body={words.deleteChatBody(deletingChat?.title ?? '')} confirm={words.delete}
        onClose={closeDeleteChat} onConfirm={() => { if (deletingChat) actions.deleteChat(deletingChat.id); }} />

      <RailBanner />
      <Box sx={{ borderTop: `1px solid ${palette.divider}`, display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 1.5, pt: 3, pb: 2, px: 1 }}>
        <Box sx={{ display: 'flex', flexDirection: 'column', gap: 1.5 }}>
          <Tooltip title={huggingFace.note}>
            <Button variant="contained" href={huggingFace.href} target="_blank" rel="noopener noreferrer"
              startIcon={<img src="/images/huggingface.svg" alt="" width={18} height={18} />}
              sx={linkButtonSx}>
              {huggingFace.label}
            </Button>
          </Tooltip>
          <Tooltip title={discord.note}>
            <Button variant="contained" href={discord.href} target="_blank" rel="noopener noreferrer"
              startIcon={<img src="/images/discordWhite.svg" alt="" width={18} height={18} />}
              sx={linkButtonSx}>
              {discord.label}
            </Button>
          </Tooltip>
        </Box>
        <Typography component="button" onClick={onOpenPrivacy}
          sx={{ display: { xs: 'inline-flex', md: 'none' }, background: 'none', border: 0, cursor: 'pointer', color: route.kind === 'privacy' ? 'text.primary' : 'text.secondary', fontSize: '0.75rem', fontFamily: 'inherit', '&:hover': { color: 'text.primary' } }}>
          {words.privacy}
        </Typography>
      </Box>
    </Box>
  );
});
