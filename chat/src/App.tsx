import { lazy, memo, Suspense, useCallback, useEffect, useState, type PointerEvent } from 'react';
import AppBar from '@mui/material/AppBar';
import Box from '@mui/material/Box';
import Drawer from '@mui/material/Drawer';
import IconButton from '@mui/material/IconButton';
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft';
import ChevronRightIcon from '@mui/icons-material/ChevronRight';
import Link from '@mui/material/Link';
import Toolbar from '@mui/material/Toolbar';
import Typography from '@mui/material/Typography';
import MenuIcon from '@mui/icons-material/Menu';
import LockOutlinedIcon from '@mui/icons-material/LockOutlined';
import type { Theme } from '@mui/material/styles';
import { useShallow } from 'zustand/react/shallow';
import { TopBanner } from './components/RailBanner';
import { Conversation } from './components/Conversation';
import { Landing } from './components/Landing';
import { ChatsMenu } from './components/ChatsMenu';
import { Sidebar } from './components/Sidebar';
import { pathFor, routeFromPath } from './routes';
import { selectActiveChat, useChatStore, type Route } from './store';
import { palette, spacing } from './theme';


const words = {
  showRail: 'Show the side bar',
  hideRail: 'Hide the side bar',
  site: 'Datamine Network',
  origin: 'https://datamine.network',
  homeTitle: 'World’s Most Efficient LLM: DAM1',
  indexed: 'index, follow, max-image-preview:large',
  unindexed: 'noindex, nofollow',
  chat: 'Chat',
  newChat: 'New chat',
  privacy: 'Privacy Policy',
  greeting: (records: number) => (records > 0 ? `Powered by ${records.toLocaleString('en-US')} Points of Data` : 'Powered by Points of Data'),
  mind: 'DAM1 Mind Explorer',
  landingPlaceholder: 'Ask Datamine Network',
  chatPlaceholder: 'Write a message...',
  chatsStay: 'Your Datamine Network chats aren’t used to improve our models.',
  mistakes: 'DAM1 is AI and can make mistakes.',
  privacyLink: 'Privacy Policy.',
  source: 'Source',
  sourceLink: 'Source.',
  loading: 'Loading the model...',
  menu: 'Open the chat list',
  resize: 'Resize the chat list',
  separator: '/',
} as const;

// Where the page's source is, at the commit this build came from: the licence asks that a served
// page offer its source, and the page offers it itself.
const repository = 'https://github.com/Datamine-Crypto/DAM1';

// The pages most visits never open load when they are first opened, so the first load carries only
// the chat.
const PrivacyPage = lazy(() => import('./components/PrivacyPage').then((m) => ({ default: m.PrivacyPage })));
const MindPage = lazy(() => import('./components/MindPage').then((m) => ({ default: m.MindPage })));
const sourceHref = __COMMIT__ ? `${repository}/tree/${__COMMIT__}` : repository;

function useRouteSync(): void {
  const route = useChatStore((state) => state.route);
  const { openChat, openPrivacy, openMind, newChat } = useChatStore(useShallow((state) => ({
    openChat: state.openChat, openPrivacy: state.openPrivacy, openMind: state.openMind, newChat: state.newChat,
  })));

  useEffect(() => {
    const go = (wanted: Route): void => {
      switch (wanted.kind) {
        case 'chat': openChat(wanted.id); break;
        case 'privacy': openPrivacy(); break;
        case 'mind': openMind(wanted.id); break;
        default: newChat();
      }
    };
    // The chats load from IndexedDB after the first render, so the address is checked
    // against them only once they are in: checked earlier, every chat address reads as unknown and a
    // refresh lands on the home page.
    const start = (): void => go(useChatStore.getState().route);
    const stopWaiting = useChatStore.persist.hasHydrated() ? (start(), undefined) : useChatStore.persist.onFinishHydration(start);
    const onPop = (): void => go(routeFromPath(window.location.pathname));
    window.addEventListener('popstate', onPop);
    return () => {
      stopWaiting?.();
      window.removeEventListener('popstate', onPop);
    };
  }, [openChat, openPrivacy, openMind, newChat]);

  // On the explorer the address names the game being played, so a game can be linked to and a reload opens it again.
  const game = useChatStore((state) => state.game);
  useEffect(() => {
    const path = (route.kind === 'mind' || route.kind === 'home') && game ? pathFor({ kind: 'mind', id: game }) : pathFor(route.kind === 'mind' ? { kind: 'mind' } : route);
    if (window.location.pathname !== path) window.history.pushState(null, '', path);
  }, [route, game]);

  // What a search engine reads at each address: a title per page, and a canonical link only on the
  // pages anyone can open, since the chats live in one browser and are kept out of search.
  useEffect(() => {
    const shared = route.kind === 'home' || route.kind === 'privacy';
    document.title = titleFor(route);
    let canonical = document.querySelector<HTMLLinkElement>('link[rel="canonical"]');
    if (shared) {
      if (!canonical) {
        canonical = document.createElement('link');
        canonical.rel = 'canonical';
        document.head.append(canonical);
      }
      canonical.href = `${words.origin}${pathFor(route)}`;
    } else {
      canonical?.remove();
    }
    document.querySelector('meta[name="robots"]')?.setAttribute('content', shared ? words.indexed : words.unindexed);
  }, [route]);
}

function titleFor(route: Route): string {
  switch (route.kind) {
    case 'home': return words.homeTitle;
    case 'privacy': return `${words.privacy} | ${words.site}`;
    case 'mind': return `${words.mind} | ${words.site}`;
    case 'chat': return `${words.chat} | ${words.site}`;
    default: return words.homeTitle;
  }
}

const Rail = memo(function Rail({ onNavigate }: { onNavigate: () => void }) {
  return <Sidebar onNavigate={onNavigate} />;
});

// The rail's right edge: drag it to resize, between the limits the store keeps, and the width is
// remembered with the chats.
const RailHandle = memo(function RailHandle() {
  const setRailWidth = useChatStore((state) => state.setRailWidth);
  const [dragging, setDragging] = useState(false);

  const onPointerDown = useCallback((event: PointerEvent<HTMLDivElement>) => {
    event.preventDefault();
    const startX = event.clientX;
    const startWidth = useChatStore.getState().railWidth;
    const target = event.currentTarget;
    target.setPointerCapture(event.pointerId);
    setDragging(true);
    const onMove = (move: globalThis.PointerEvent): void => setRailWidth(startWidth + move.clientX - startX);
    const onUp = (): void => {
      target.removeEventListener('pointermove', onMove);
      target.removeEventListener('pointerup', onUp);
      target.removeEventListener('pointercancel', onUp);
      setDragging(false);
    };
    target.addEventListener('pointermove', onMove);
    target.addEventListener('pointerup', onUp);
    target.addEventListener('pointercancel', onUp);
  }, [setRailWidth]);

  return (
    <Box
      role="separator"
      aria-orientation="vertical"
      aria-label={words.resize}
      onPointerDown={onPointerDown}
      sx={{
        position: 'absolute',
        top: 0,
        bottom: 0,
        right: -3,
        width: 6,
        cursor: 'col-resize',
        zIndex: 1,
        '&::after': {
          content: '""',
          position: 'absolute',
          top: 0,
          bottom: 0,
          left: 2,
          width: 2,
          background: dragging ? palette.outlinedButtonBorder : 'transparent',
          transition: 'background-color 120ms',
        },
        '&:hover::after': { background: palette.outlinedButtonBorder },
      }}
    />
  );
});

// The line under the composer: where the chats live, what the reader is, and the way to the policy.
const Footnote = memo(function Footnote() {
  const openPrivacy = useChatStore((state) => state.openPrivacy);
  return (
    <>
      <LockOutlinedIcon sx={{ fontSize: '1.05em', verticalAlign: 'text-bottom', mr: 0.5, opacity: 0.8 }} />
      {`${words.chatsStay} ${words.mistakes} `}
      <Link component="button" onClick={openPrivacy} underline="always"
        sx={{ color: 'inherit', fontSize: 'inherit', fontFamily: 'inherit', verticalAlign: 'baseline' }}>
        {words.privacyLink}
      </Link>
      {' '}
      <Link href={sourceHref} target="_blank" rel="noopener noreferrer" underline="always"
        sx={{ color: 'inherit', fontSize: 'inherit', fontFamily: 'inherit', verticalAlign: 'baseline' }}>
        {words.sourceLink}
      </Link>
    </>
  );
});

const Main = memo(function Main() {
  const route = useChatStore((state) => state.route);
  const chat = useChatStore(selectActiveChat);
  const thinking = useChatStore((state) => state.thinking);
  const live = useChatStore((state) => state.live);
  const booting = useChatStore((state) => state.booting);
  const send = useChatStore((state) => state.send);
  const openMind = useChatStore((state) => state.openMind);
  const records = useChatStore((state) => state.agent.nodes);
  const disabled = thinking;

  if (route.kind === 'privacy') return <Suspense fallback={null}><PrivacyPage /></Suspense>;
  // The mind explorer is the default view: home is the explorer on a new game.
  if (route.kind === 'mind' || route.kind === 'home') return <Suspense fallback={null}><MindPage footnote={<Footnote />} /></Suspense>;
  if (!chat) {
    return <Landing greeting={words.greeting(records)} placeholder={booting ? words.loading : words.landingPlaceholder} disabled={disabled} onSend={send} />;
  }
  return (
    <Conversation messages={chat.messages} thinking={thinking} live={live} placeholder={booting ? words.loading : words.chatPlaceholder}
      disabled={disabled} onSend={send} footnote={<Footnote />} onMind={() => openMind(chat.id)} />
  );
});

// The path above the page: the chat, when one is open.
const Crumbs = memo(function Crumbs() {
  const route = useChatStore((state) => state.route);
  const chatTitle = useChatStore((state) => selectActiveChat(state)?.title ?? null);
  const crumbs: { label: string; go?: () => void }[] = [];
  if (chatTitle) crumbs.push({ label: chatTitle });
  if (route.kind === 'privacy') crumbs.push({ label: words.privacy });
  if (crumbs.length === 0) crumbs.push({ label: words.newChat });
  return (
    <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, minWidth: 0, flex: 1, fontSize: '0.9375rem' }}>
      {crumbs.map((crumb, at) => (
        <Box key={`${at}-${crumb.label}`} sx={{ display: 'flex', alignItems: 'center', gap: 1, minWidth: 0 }}>
          {at > 0 && <Typography sx={{ color: 'text.secondary' }}>{words.separator}</Typography>}
          {crumb.go
            ? <Link component="button" onClick={crumb.go} underline="hover" sx={{ color: 'text.secondary', fontSize: 'inherit', fontFamily: 'inherit' }}>{crumb.label}</Link>
            : <Typography noWrap sx={{ color: 'text.primary', fontSize: 'inherit' }}>{crumb.label}</Typography>}
        </Box>
      ))}
    </Box>
  );
});

// The policy link sits at the bottom right of the page on a desktop; on a phone it stays in the
// drawer, where the page's bottom edge belongs to the composer.
// Hidden on a chat, where the composer's footnote already links to the policy, and on the policy
// itself.
const PrivacyLink = memo(function PrivacyLink() {
  const current = useChatStore((state) => state.route.kind === 'privacy');
  const onChat = useChatStore((state) => state.route.kind === 'chat');
  const openPrivacy = useChatStore((state) => state.openPrivacy);
  const onMind = useChatStore((state) => state.route.kind === 'mind' || state.route.kind === 'home');
  if (onChat || current || onMind) return null;
  const look = { background: 'none', border: 0, cursor: 'pointer', color: 'text.secondary', fontSize: '0.75rem', fontFamily: 'inherit', textDecoration: 'none', '&:hover': { color: 'text.primary' } } as const;
  return (
    <Box sx={{ position: 'absolute', right: 16, bottom: 10, display: { xs: 'none', md: 'flex' }, gap: 2 }}>
      <Typography component="button" onClick={openPrivacy} sx={look}>
        {words.privacy}
      </Typography>
      <Typography component="a" href={sourceHref} target="_blank" rel="noopener noreferrer" sx={look}>
        {words.source}
      </Typography>
    </Box>
  );
});

export function App() {
  const [mobileOpen, setMobileOpen] = useState(false);
  const routeKind = useChatStore((state) => state.route.kind);
  const title = useChatStore((state) => selectActiveChat(state)?.title ?? words.site);
  useRouteSync();

  useEffect(() => { document.title = `${title} | ${words.site}`; }, [title]);

  const closeMobile = useCallback(() => setMobileOpen(false), []);
  const toggleMobile = useCallback(() => setMobileOpen((open) => !open), []);

  const railWidth = useChatStore((state) => state.railWidth);
  const railHidden = useChatStore((state) => state.railHidden);
  const toggleRail = useChatStore((state) => state.toggleRail);
  const mobileSx = {
    width: (theme: Theme) => theme.spacing(spacing.drawerWidthUnits),
    '& .MuiDrawer-paper': { width: (theme: Theme) => theme.spacing(spacing.drawerWidthUnits), boxSizing: 'border-box' },
  } as const;
  const railSx = {
    width: railWidth,
    flexShrink: 0,
    '& .MuiDrawer-paper': { width: railWidth, boxSizing: 'border-box', overflow: 'visible' },
  } as const;

  return (
    <Box sx={{ display: 'flex', height: '100dvh', overflow: 'hidden' }}>
      <Drawer variant="temporary" open={mobileOpen} onClose={closeMobile} ModalProps={{ keepMounted: true }}
        sx={{ display: { xs: 'block', md: 'none' }, ...mobileSx }}>
        <Rail onNavigate={closeMobile} />
      </Drawer>
      <Drawer variant="permanent" sx={{ display: { xs: 'none', md: railHidden ? 'none' : 'block' }, ...railSx }}>
        <Rail onNavigate={closeMobile} />
        <RailHandle />
      </Drawer>
      <Box component="main" sx={{ flex: 1, minWidth: 0, display: 'flex', flexDirection: 'column', minHeight: 0, position: 'relative' }}>
        <IconButton onClick={toggleRail} size="small" aria-label={railHidden ? words.showRail : words.hideRail} title={railHidden ? words.showRail : words.hideRail} sx={{
          display: { xs: 'none', md: 'inline-flex' }, position: 'absolute', left: 4, top: '50%', transform: 'translateY(-50%)', zIndex: 6, width: 22, height: 48, borderRadius: 1.5,
          background: palette.nestedPaper, border: `1px solid ${palette.divider}`, color: 'text.secondary', '&:hover': { background: palette.hover, color: '#fff' },
        }}>
          {railHidden ? <ChevronRightIcon fontSize="small" /> : <ChevronLeftIcon fontSize="small" />}
        </IconButton>
        {routeKind !== 'mind' && routeKind !== 'home' && <TopBanner />}
        <AppBar position="static" elevation={0}
          sx={{ background: palette.pageBackground, borderBottom: `1px solid ${palette.divider}`, display: routeKind === 'home' || routeKind === 'mind' ? { xs: 'block', md: 'none' } : 'block' }}>
          <Toolbar variant="dense" sx={{ minHeight: (theme) => theme.spacing(spacing.topBarHeightUnits), gap: 1 }}>
            <IconButton color="inherit" aria-label={words.menu} edge="start" onClick={toggleMobile} sx={{ display: { xs: 'inline-flex', md: 'none' } }}>
              <MenuIcon />
            </IconButton>
            {routeKind !== 'mind' && <Crumbs />}
            {routeKind === 'mind' && <ChatsMenu />}
          </Toolbar>
        </AppBar>
        <Main />
        <PrivacyLink />
      </Box>
    </Box>
  );
}
