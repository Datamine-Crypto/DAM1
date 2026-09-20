// The explorer's game. A conversation is a game: every turn of the chat keeps the moves the network took,
// and the game plays the whole chat back on a map seen from above. The walker is the cursor. Every thing a
// move names is a tile in the region of the map it belongs to, a thing put into another stands inside it,
// and what he grabbed is in his hands. A small picture beside him shows what he does, a thumbs up when he continues, and he says only the word he hears and his answer. On the right are his stats: where
// he stands, what he holds, the stack the network sees, and everything he memorized so far.
import { memo, useCallback, useEffect, useMemo, useRef, useState } from 'react';
import useMediaQuery from '@mui/material/useMediaQuery';
import { useTheme } from '@mui/material/styles';
import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Chip from '@mui/material/Chip';
import IconButton from '@mui/material/IconButton';
import Dialog from '@mui/material/Dialog';
import DialogActions from '@mui/material/DialogActions';
import DialogContent from '@mui/material/DialogContent';
import DialogTitle from '@mui/material/DialogTitle';
import InputBase from '@mui/material/InputBase';
import MenuItem from '@mui/material/MenuItem';
import List from '@mui/material/List';
import ListItemButton from '@mui/material/ListItemButton';
import ListItemText from '@mui/material/ListItemText';
import HistoryIcon from '@mui/icons-material/History';
import TuneIcon from '@mui/icons-material/Tune';
import Select from '@mui/material/Select';
import Tabs from '@mui/material/Tabs';
import Tab from '@mui/material/Tab';
import TextField from '@mui/material/TextField';
import Typography from '@mui/material/Typography';
import PauseIcon from '@mui/icons-material/Pause';
import PlayArrowIcon from '@mui/icons-material/PlayArrow';
import ReplayIcon from '@mui/icons-material/Replay';
import ChevronLeftIcon from '@mui/icons-material/ChevronLeft';
import ChevronRightIcon from '@mui/icons-material/ChevronRight';
import FastForwardIcon from '@mui/icons-material/FastForward';
import CloseIcon from '@mui/icons-material/Close';
import SkipPreviousIcon from '@mui/icons-material/SkipPrevious';
import { useShallow } from 'zustand/react/shallow';
import { Conversation } from './Conversation';
import { pictureOfKinds, regionOfKinds, type Region } from '../emoji';
import { mindKinds } from '../agent';
import { showChatEvent } from './ChatsMenu';
import { byRecent, useChatStore } from '../store';
import { palette } from '../theme';
import { TopBanner } from './RailBanner';
import { words } from '../mind/words';
import { singular, plainName, worldInk, modes, frameTime, type Frame, turnsOf, lacksMoves, framesOf } from '../mind/replay';
import { type You, plainYou, YouContext, youKey, tokenPictures, youPictures, keptYou } from '../mind/you';
import { World2D } from '../mind/World2D';
import { ago, speeds, instantKey, Stats } from '../mind/Stats';
import { BuildStamp } from '../mind/BuildStamp';

export const MindGame = memo(function MindGame({ footnote }: { footnote: React.ReactNode }) {
  const { chats, game, thinking, recalling, ready } = useChatStore(useShallow((state) => ({
    chats: state.chats, game: state.game, thinking: state.thinking, recalling: state.recalling, ready: state.agent.ready,
  })));
  const pickGame = useChatStore((state) => state.pickGame);
  const onMind = useChatStore((state) => state.route.kind === 'mind');
  const [chatShown, setChatShown] = useState(false);
  const [historyShown, setHistoryShown] = useState(false);
  const [speedShown, setSpeedShown] = useState(false);
  // The play-back is folded away until a person asks for it, so the map is clear.
  const [barShown, setBarShown] = useState(false);
  useEffect(() => {
    const onShow = (): void => setChatShown(true);
    window.addEventListener(showChatEvent, onShow);
    return () => window.removeEventListener(showChatEvent, onShow);
  }, []);
  const [undoing, setUndoing] = useState<string | null>(null);
  const undoTurn = useChatStore((state) => state.undoTurn);
  const recallGame = useChatStore((state) => state.recallGame);
  const send = useChatStore((state) => state.send);

  const recent = useMemo(() => byRecent(Object.values(chats)), [chats]);
  const chat = game && Object.hasOwn(chats, game) ? chats[game] : null;
  const turns = useMemo(() => turnsOf(chat), [chat]);
  // The groups come from the network: the engine is asked what each named thing is, and the page's own tables only fill in what the facts leave out.
  const [known, setKnown] = useState<Map<string, Region>>(() => new Map());
  const [kindPictures, setKindPictures] = useState<Map<string, string>>(() => new Map());
  const names = useMemo(() => [...new Set(framesOf(turns, new Map()).flatMap((frame) => frame.scene.places.map((place) => place.name)))].sort().join(' '), [turns]);
  useEffect(() => {
    if (!ready || names === '' || thinking || recalling) return;
    let live = true;
    const asked = names.split(' ').flatMap((name) => [name, singular(name)]).filter((name, at, all) => plainName.test(name) && all.indexOf(name) === at);
    mindKinds(asked).then((told) => {
      if (!live) return;
      const next = new Map<string, Region>();
      const drawn = new Map<string, string>();
      for (const name of names.split(' ')) {
        const kinds = [...(told[name] ?? []), ...(told[singular(name)] ?? [])];
        const region = regionOfKinds(kinds);
        if (region) next.set(name, region);
        const picture = pictureOfKinds(kinds);
        if (picture) drawn.set(name, picture);
      }
      setKnown(next);
      setKindPictures(drawn);
    }).catch(() => undefined);
    return () => { live = false; };
  }, [names, ready, thinking, recalling]);
  const frames = useMemo(() => framesOf(turns, known), [turns, known]);
  const last = frames.length - 1;

  const [text, setText] = useState('');
  // False while what is typed is said to him, true while it changes the world behind his back.
  const [worldMode, setWorldMode] = useState(false);
  const [at, setAt] = useState(0);
  const [playing, setPlaying] = useState(true);
  // A phone has no room for large buttons under the map, so they are small there and medium on a screen.
  const phone = useMediaQuery(useTheme().breakpoints.down('sm'));
  const buttonSize = phone ? 'small' : 'medium';
  // On a phone one of the map, the overview, the stack and the analytics is shown at a time.
  const [phoneTab, setPhoneTab] = useState(0);
  const [speed, setSpeed] = useState(1);
  const [instant, setInstant] = useState(() => { try { return window.localStorage.getItem(instantKey) === 'yes'; } catch { return false; } });
  const pickSpeed = useCallback((one: number | null) => {
    setInstant(one === null);
    if (one !== null) setSpeed(one);
    try { window.localStorage.setItem(instantKey, one === null ? 'yes' : 'no'); } catch { /* A browser that keeps nothing still plays as chosen. */ }
  }, []);
  const done = at >= last;

  // The explorer opens on the chat talked in last, so the game shown is the conversation a person just had.
  const opened = useRef(false);
  useEffect(() => {
    if (opened.current) return;
    opened.current = true;
    if (!game && onMind && recent.length > 0) pickGame(recent[0].id);
  }, [game, onMind, recent, pickGame]);

  // A chat read before its moves were kept is read again once, on the explorer's own mind.
  const recalled = useRef(new Set<string>());
  useEffect(() => {
    if (!chat || !ready || thinking || recalling || !lacksMoves(chat) || recalled.current.has(chat.id)) return;
    recalled.current.add(chat.id);
    void recallGame(chat.id);
  }, [chat, ready, thinking, recalling, recallGame]);

  // Another game opens at its end; a new turn in the same game plays from where that turn starts.
  const justSaid = useRef(false);
  const shown = useRef<{ game: string | null; turns: number }>({ game: null, turns: 0 });
  useEffect(() => {
    const before = shown.current;
    if (before.game !== game) {
      // A game opened shows its end, the world as the chat left it; the controls play it back from any turn.
      setAt(Number.MAX_SAFE_INTEGER);
      setPlaying(false);
    } else if (turns.length > before.turns && justSaid.current && instant) {
      justSaid.current = false;
      setAt(Number.MAX_SAFE_INTEGER);
      setPlaying(false);
    } else if (turns.length > before.turns && justSaid.current) {
      // The turn just said plays from its start; turns that only arrived, a chat loading or read again, show the end.
      justSaid.current = false;
      const first = frames.findIndex((frame) => frame.turn === before.turns);
      setAt(Math.max(first, 0));
      setPlaying(true);
    } else if (turns.length > before.turns) {
      setAt(Number.MAX_SAFE_INTEGER);
      setPlaying(false);
    }
    shown.current = { game, turns: turns.length };
  }, [game, turns.length, frames, instant]);

  useEffect(() => {
    if (!playing) return;
    const timer = setInterval(() => setAt((frame) => frame + 1), frameTime / speed);
    return () => clearInterval(timer);
  }, [playing, speed]);

  useEffect(() => { if (playing && at >= last && last >= 0) setPlaying(false); }, [playing, at, last]);

  // The left and right arrow keys step him back and on through time, unless the keys are moving through words being typed.
  useEffect(() => {
    const onKey = (event: KeyboardEvent): void => {
      if (event.key !== 'ArrowLeft' && event.key !== 'ArrowRight') return;
      const typed = event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement ? event.target.value : '';
      if (typed !== '' || last < 0) return;
      event.preventDefault();
      setPlaying(false);
      setAt((now) => (event.key === 'ArrowLeft' ? Math.max(Math.min(now, last) - 1, 0) : Math.min(now + 1, last)));
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [last]);

  const say = useCallback((prompt: string, world = false) => {
    const asked = prompt.trim();
    if (!asked || thinking) return;
    setText('');
    justSaid.current = true;
    void send(asked, world);
  }, [thinking, send]);

  // The person's own name and picture on the page, and the dialog that picks them.
  const [you, setYou] = useState(keptYou);
  const [picking, setPicking] = useState<{ name: string; picture: string } | null>(null);
  // The person's own labels, kept for each game in this browser.
  const labelsKey = 'mind-pictures';
  const [labels, setLabels] = useState<Record<string, string>>({});
  useEffect(() => {
    try {
      const kept: unknown = JSON.parse(window.localStorage.getItem(labelsKey) ?? '{}');
      const safe: Record<string, string> = {};
      if (typeof kept === 'object' && kept !== null) for (const [name, text] of Object.entries(kept)) if (typeof text === 'string') safe[name] = text.slice(0, 80);
      setLabels(safe);
    } catch {
      setLabels({});
    }
  }, [labelsKey]);
  const [labelling, setLabelling] = useState<{ name: string; text: string } | null>(null);
  const keepLabel = useCallback(() => {
    if (!labelling) return;
    const next = { ...labels };
    if (labelling.text.trim() === '') delete next[labelling.name]; else next[labelling.name] = labelling.text.trim().slice(0, 80);
    setLabels(next);
    try { window.localStorage.setItem(labelsKey, JSON.stringify(next)); } catch { /* A browser that keeps nothing still shows the label. */ }
    setLabelling(null);
  }, [labelling, labels, labelsKey]);
  const shownYou = useMemo<You>(() => ({ ...you, edit: () => setPicking(you), labels, label: (name) => setLabelling({ name, text: labels[name] ?? '' }), kindPictures }), [you, labels, kindPictures]);
  const keepYou = useCallback(() => {
    if (!picking) return;
    const chosen = { name: picking.name.trim() || plainYou.name, picture: picking.picture.trim() || plainYou.picture };
    setYou(chosen);
    try { window.localStorage.setItem(youKey, JSON.stringify(chosen)); } catch { /* A browser that keeps nothing still shows the choice. */ }
    setPicking(null);
  }, [picking]);

  // What the person asked to do with the game shown: rename it, save it as another, or delete it.
  // The time now, looked at twice a minute, so the history's times ago stay true.
  const [now, setNow] = useState(() => Date.now());
  useEffect(() => {
    const clock = setInterval(() => setNow(Date.now()), 30_000);
    return () => clearInterval(clock);
  }, []);
  const frame = frames.length > 0 ? frames[Math.min(at, last)] : null;
  // The history of what was said keeps the turn being played in sight.
  const history = useRef<HTMLDivElement | null>(null);
  const playedTurn = frame?.turn ?? -1;
  useEffect(() => {
    history.current?.querySelector('[data-now="yes"]')?.scrollIntoView({ block: 'nearest' });
  }, [playedTurn]);
  // A turn picked in the history shows its end, what the world was once it was said.
  const toTurnEnd = useCallback((nth: number) => {
    const lastOf = frames.reduce((found, one, at) => (one.turn === nth ? at : found), -1);
    if (lastOf >= 0) { setAt(lastOf); setPlaying(false); }
  }, [frames]);
  const toTurn = useCallback((nth: number) => {
    const first = frames.findIndex((one) => one.turn === nth);
    if (first >= 0) { setAt(first); setPlaying(false); }
  }, [frames]);
  const idle: Frame = { turn: 0, word: 0, scene: { places: [], at: null, carries: null, owns: false, born: null, points: null, links: [], trail: [], notes: [], lit: [], curious: [] }, stack: [{ text: 'cursor', kind: 'cursor' }], note: '', says: [], doing: null, answers: false, heardSoFar: 0, movedSoFar: 0, kept: 0 };

  return (
    <YouContext.Provider value={shownYou}>
    <Box sx={{ flex: 1, minHeight: 0, display: 'grid', gridTemplateColumns: { xs: '1fr', md: 'minmax(0, 1fr) 320px' }, gridTemplateRows: 'minmax(0, 1fr)', gap: { xs: 0, md: 1.5 }, px: { xs: 0, md: 2 }, pt: { xs: 0, md: 1.5 }, pb: 0 }}>
      <Box sx={{ minWidth: 0, minHeight: 0, display: 'flex', flexDirection: 'column' }}>
      <Box sx={{ flex: 1, minHeight: 0, overflowY: { xs: 'auto', md: 'hidden' }, px: 0, pt: { xs: 0.75, md: 0 }, pb: 0, display: 'flex', flexDirection: 'column', gap: { xs: 0.75, md: 1.5 } }}>
      <Box sx={{ px: { xs: 1.5, md: 0 } }}><TopBanner inset /></Box>

      <Tabs value={phoneTab} onChange={(_, picked: number) => setPhoneTab(picked)} variant="fullWidth" sx={{
        display: { xs: 'flex', md: 'none' }, minHeight: 36, '& .MuiTab-root': { minHeight: 36, minWidth: 0, px: 0.5, textTransform: 'none', fontSize: '0.8125rem' },
      }}>
        <Tab label={words.map} />
        <Tab label={words.memorized} />
        <Tab label={words.analytics} />
        <Tab label={words.counts} />
      </Tabs>
      <Box sx={{ flex: 1, minHeight: 0, display: 'grid' }}>
        <Box sx={{ position: 'relative', minWidth: 0, minHeight: 0, display: { xs: phoneTab === 0 ? 'grid' : 'none', md: 'grid' } }}>
          <BuildStamp />
          <World2D scene={(frame ?? idle).scene} says={frame ? frame.says : [recalling ? words.recalling : thinking ? words.reading : words.empty]} doing={frame?.doing ?? null} beat={at} layoutKey={game ?? 'new'} output={frame?.answers ? turns[frame.turn]?.said || (turns[frame.turn]?.asks ? words.noAnswer : null) : null} question={frame && turns[frame.turn]?.asks ? turns[frame.turn].asked : null} answers={frame?.answers ?? false} />
          {barShown && (
          <Box sx={{ position: 'absolute', left: 10, bottom: 10, zIndex: 3, display: 'flex', alignItems: 'center', maxWidth: 'calc(100% - 20px)', gap: { xs: 0.25, md: 0.5 }, px: 1, py: 0.25, borderRadius: 999, background: 'rgba(10, 14, 28, 0.85)', border: `1px solid ${palette.divider}` }}>
        <IconButton size={buttonSize} onClick={() => { if (frame) toTurn(Math.max(frame.turn - (frames[at - 1]?.turn === frame.turn ? 0 : 1), 0)); }} disabled={!frame} aria-label="start of the turn"><SkipPreviousIcon fontSize={buttonSize} /></IconButton>
        <IconButton size={buttonSize} onClick={() => { setPlaying(false); setAt((now) => Math.max(Math.min(now, last) - 1, 0)); }} disabled={!frame || at <= 0} aria-label="step back"><ChevronLeftIcon fontSize={buttonSize} /></IconButton>
        <IconButton size={buttonSize} onClick={() => { if (done) setAt(0); setPlaying((was) => !was || done); }} disabled={!frame} aria-label="play or pause">
          {done && frame ? <ReplayIcon fontSize={buttonSize} /> : playing ? <PauseIcon fontSize={buttonSize} /> : <PlayArrowIcon fontSize={buttonSize} />}
        </IconButton>
        <IconButton size={buttonSize} onClick={() => { setPlaying(false); setAt((now) => Math.min(now + 1, last)); }} disabled={!frame || done} aria-label="next step"><ChevronRightIcon fontSize={buttonSize} /></IconButton>
        <IconButton size={buttonSize} onClick={() => { setPlaying(false); setAt(last); }} disabled={!frame || done} aria-label={words.toEnd}><FastForwardIcon fontSize={buttonSize} /></IconButton>
        <IconButton size={buttonSize} onClick={() => setHistoryShown(true)} disabled={turns.length === 0} aria-label={words.history} title={words.history}><HistoryIcon fontSize={buttonSize} /></IconButton>
        <Box sx={{ width: '1px', alignSelf: 'stretch', background: palette.divider, mx: 0.5 }} />
        <Chip label={instant ? words.instant : `${speed}x`} size="small" variant="outlined" onClick={() => setSpeedShown(true)} title={words.speed} sx={{ fontSize: '0.75rem' }} />
        <IconButton size={buttonSize} onClick={() => setBarShown(false)} aria-label={words.foldControls} title={words.foldControls}><CloseIcon fontSize={buttonSize} /></IconButton>
          </Box>
          )}
          {!barShown && (
            <IconButton size={buttonSize} onClick={() => setBarShown(true)} aria-label={words.controls} title={words.controls} sx={{
              position: 'absolute', left: 10, bottom: 10, zIndex: 3, background: 'rgba(10, 14, 28, 0.85)', border: `1px solid ${palette.divider}`,
            }}><TuneIcon fontSize={buttonSize} /></IconButton>
          )}
        </Box>
      </Box>

      </Box>
      <Dialog open={chatShown && chat !== undefined} onClose={() => setChatShown(false)} fullWidth maxWidth="md">
        <DialogTitle>{chat?.title}</DialogTitle>
        <DialogContent dividers sx={{ height: '70vh', display: 'flex', flexDirection: 'column', p: 0 }}>
          {chat && <Conversation messages={chat.messages} thinking={false} live={null} placeholder="" disabled onSend={() => undefined} footnote={null} readOnly />}
        </DialogContent>
        <DialogActions><Button onClick={() => setChatShown(false)}>{words.close}</Button></DialogActions>
      </Dialog>
      <Dialog open={speedShown} onClose={() => setSpeedShown(false)} maxWidth="xs" slotProps={{ paper: { sx: { borderRadius: 3 } } }}>
        <DialogTitle sx={{ fontSize: '1.125rem' }}>{words.speed}</DialogTitle>
        <DialogContent dividers sx={{ p: 0 }}>
          <List dense disablePadding>
            <ListItemButton selected={instant} onClick={() => { pickSpeed(null); setSpeedShown(false); }}>
              <ListItemText primary={words.instant} secondary={words.instantHint} />
            </ListItemButton>
            {speeds.map((one) => (
              <ListItemButton key={one} selected={!instant && one === speed} onClick={() => { pickSpeed(one); setSpeedShown(false); }}>
                <ListItemText primary={`${one}x`} />
              </ListItemButton>
            ))}
          </List>
        </DialogContent>
      </Dialog>
      <Dialog open={historyShown} onClose={() => setHistoryShown(false)} fullWidth maxWidth="sm" slotProps={{ paper: { sx: { borderRadius: 3 } } }}>
        <DialogTitle sx={{ fontSize: '1.125rem' }}>{words.history}</DialogTitle>
        <DialogContent dividers sx={{ p: 1 }}>
            <Box ref={history} sx={{
              overflowY: 'auto', p: 1,
              display: 'flex', flexDirection: 'column', gap: 0.5,
            }}>
              {turns.map((one, nth) => (
                <Box key={nth} data-now={frame?.turn === nth ? 'yes' : undefined} onClick={() => toTurnEnd(nth)} sx={{
                  cursor: 'pointer', borderRadius: 1, px: 1, py: 0.5, borderLeft: `3px solid ${frame?.turn === nth ? palette.linkInk : 'transparent'}`,
                  background: frame?.turn === nth ? 'rgba(0, 255, 255, 0.08)' : 'transparent', '&:hover': { background: palette.hover },
                }}>
                  <IconButton size="small" aria-label={words.undo} disabled={thinking || recalling} onClick={(event) => { event.stopPropagation(); setUndoing(one.id); }} sx={{ float: 'right', ml: 0.5, p: 0.25, color: 'text.secondary', opacity: 0.6, '&:hover': { opacity: 1, color: '#ff8a80' } }}><CloseIcon sx={{ fontSize: 16 }} /></IconButton>
                  <Typography sx={{ fontSize: { xs: '0.9375rem', md: '1.0625rem' }, overflowWrap: 'anywhere', fontStyle: one.world ? 'italic' : 'normal', color: one.world ? 'text.secondary' : 'text.primary' }}>{one.asked}{one.world && <Box component="span" sx={{ float: 'right', ml: 1, fontStyle: 'normal' }}>{words.worldMark}</Box>}</Typography>
                  <Typography sx={{ fontSize: '0.6875rem', color: 'text.secondary' }}>{ago(one.at, now)}</Typography>
                  {one.said && <Typography sx={{ fontSize: { xs: '0.9375rem', md: '1.0625rem' }, fontWeight: 600, color: palette.linkInk, overflowWrap: 'anywhere' }}>{one.said}</Typography>}
                </Box>
              ))}
            </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setHistoryShown(false)} sx={{ textTransform: 'none', color: 'text.secondary' }}>{words.close}</Button>
        </DialogActions>
      </Dialog>

      <Dialog open={undoing !== null} onClose={() => setUndoing(null)} fullWidth maxWidth="xs">
        <DialogTitle>{words.undoTitle}</DialogTitle>
        <DialogContent><Typography>{words.undoHint}</Typography><Typography sx={{ mt: 1, fontStyle: 'italic' }}>{turns.find((one) => one.id === undoing)?.asked}</Typography></DialogContent>
        <DialogActions>
          <Button onClick={() => setUndoing(null)}>{words.cancel}</Button>
          <Button color="error" onClick={() => { if (chat && undoing) void undoTurn(chat.id, undoing); setUndoing(null); }}>{words.undo}</Button>
        </DialogActions>
      </Dialog>
      <Box component="form" onSubmit={(event) => { event.preventDefault(); say(text, worldMode); }} sx={{ display: 'flex', gap: { xs: 1, md: 1.5 }, flexWrap: 'wrap', alignItems: 'stretch', px: { xs: 1.5, md: 0 }, pt: 1.5, pb: 0.25 }}>
        <Box sx={{ display: { xs: 'none', md: 'flex' }, flexDirection: 'column', gap: 0.75, width: 290 }}>
          {modes.map((mode) => {
            const picked = mode.world === worldMode;
            const ink = mode.world ? worldInk : palette.linkInk;
            return (
              <Box key={mode.title} component="button" type="button" onClick={() => setWorldMode(mode.world)} aria-pressed={picked} sx={{
                flex: 1, textAlign: 'left', cursor: 'pointer', borderRadius: 2, px: 1.5, py: 0.75, fontFamily: 'inherit', color: picked ? '#fff' : 'text.secondary',
                border: `2px solid ${picked ? ink : palette.divider}`, background: picked ? `color-mix(in srgb, ${ink} 14%, transparent)` : 'transparent',
              }}>
                <Typography sx={{ fontSize: '1rem', fontWeight: 600, color: picked ? ink : 'inherit' }}>{mode.title}</Typography>
                <Typography sx={{ fontSize: '0.8125rem' }}>{mode.tells}</Typography>
              </Box>
            );
          })}
        </Box>
        <InputBase
          value={text}
          onChange={(event) => setText(event.target.value)}
          placeholder={worldMode ? words.worldPlaceholder : words.placeholder}
          autoFocus
          multiline
          rows={phone ? 2 : 3}
          onKeyDown={(event) => {
            // Enter sends it; with shift it starts a new line.
            if (event.key === 'Enter' && !event.shiftKey) { event.preventDefault(); say(text, worldMode); }
          }}
          sx={{
            flex: { xs: 'none', md: 1 }, width: { xs: '100%', md: 'auto' }, minWidth: { xs: 0, md: 260 }, px: { xs: 1.5, md: 2.5 }, py: { xs: 1, md: 1.5 },
            borderRadius: 3, border: `2px solid ${palette.divider}`, fontSize: { xs: '1.0625rem', md: '1.375rem' },
            transition: 'border-color 200ms, background 200ms',
            '&:focus-within': { borderColor: worldMode ? worldInk : palette.linkInk, background: worldMode ? 'rgba(255, 209, 102, 0.06)' : 'rgba(0, 255, 255, 0.05)' },
          }}
        />
        <Box sx={{ display: { xs: 'flex', md: 'contents' }, width: { xs: '100%', md: 'auto' }, gap: 1, alignItems: 'center' }}>
          <Select value={worldMode ? 'world' : 'talk'} onChange={(event) => setWorldMode(event.target.value === 'world')} size="small" aria-label={words.mode}
            sx={{ display: { xs: 'flex', md: 'none' }, flex: 1, borderRadius: 2, fontSize: '0.9375rem', color: worldMode ? worldInk : palette.linkInk,
              '& .MuiOutlinedInput-notchedOutline': { borderColor: palette.divider } }}>
            {modes.map((mode) => <MenuItem key={mode.title} value={mode.world ? 'world' : 'talk'} sx={{ fontSize: '0.9375rem' }}>{mode.title}</MenuItem>)}
          </Select>
          <Button type="submit" variant="outlined" disabled={thinking || recalling || !text.trim()} sx={{
            fontSize: { xs: '1rem', md: '1.25rem' }, px: { xs: 3, md: 4 }, borderRadius: 3, textTransform: 'none', fontWeight: 700, borderWidth: 2,
            color: worldMode ? worldInk : palette.linkInk, borderColor: worldMode ? worldInk : palette.linkInk,
            '&:hover': { borderWidth: 2, borderColor: worldMode ? worldInk : palette.linkInk, background: worldMode ? 'rgba(255, 213, 128, 0.12)' : 'rgba(0, 255, 255, 0.1)' },
            '&.Mui-disabled': { borderWidth: 2, borderColor: 'rgba(255, 255, 255, 0.2)' },
          }}>{words.say}</Button>
        </Box>
      </Box>
      <Typography component="div" sx={{ color: 'text.secondary', fontSize: '0.75rem', textAlign: 'center', px: 2, pt: 0.75, pb: 'calc(6px + env(safe-area-inset-bottom))', lineHeight: 1.35 }}>{footnote}</Typography>
      </Box>
          <Box sx={{ display: { xs: phoneTab === 0 ? 'none' : 'grid', md: 'grid' }, minWidth: 0, minHeight: 0, px: { xs: 1.5, md: 0 } }}>
            <Stats frame={frame ?? idle} turns={turns} pick={phone ? Math.max(phoneTab - 1, 0) : undefined} onPick={phone ? (picked: number) => setPhoneTab(picked + 1) : undefined} />
          </Box>
      <Dialog open={labelling !== null} onClose={() => setLabelling(null)} fullWidth maxWidth="xs">
        <DialogTitle>{`${words.labelTitle} ${labelling?.name ?? ''}`}</DialogTitle>
        <DialogContent>
          <Typography sx={{ color: 'text.secondary', fontSize: '0.875rem', mb: 1 }}>{words.labelHint}</Typography>
          <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5 }}>
            {tokenPictures.map((picture) => (
              <Box key={picture} component="button" type="button" onClick={() => setLabelling((was) => (was ? { ...was, text: picture } : was))} sx={{
                fontSize: '1.5rem', lineHeight: 1, p: 0.75, borderRadius: 1.5, cursor: 'pointer', background: labelling?.text === picture ? 'rgba(0, 255, 255, 0.18)' : 'transparent',
                border: `2px solid ${labelling?.text === picture ? palette.linkInk : 'transparent'}`,
              }}>{picture}</Box>
            ))}
          </Box>
          <TextField fullWidth margin="dense" label={words.labelText} value={labelling?.text ?? ''} onChange={(event) => setLabelling((was) => (was ? { ...was, text: event.target.value.slice(0, 16) } : was))} onKeyDown={(event) => { if (event.key === 'Enter') keepLabel(); }} sx={{ mt: 1.5 }} />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setLabelling(null)} sx={{ color: 'text.secondary' }}>{words.cancel}</Button>
          <Button onClick={keepLabel}>{words.keep}</Button>
        </DialogActions>
      </Dialog>
      <Dialog open={picking !== null} onClose={() => setPicking(null)} fullWidth maxWidth="xs">
        <DialogTitle>{words.youTitle}</DialogTitle>
        <DialogContent>
          <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 0.5, mt: 1.5 }}>
            {youPictures.map((picture) => (
              <Box key={picture} component="button" type="button" onClick={() => setPicking((was) => (was ? { ...was, picture } : was))} sx={{
                fontSize: '1.5rem', lineHeight: 1, p: 0.75, borderRadius: 1.5, cursor: 'pointer', background: picking?.picture === picture ? 'rgba(0, 255, 255, 0.18)' : 'transparent',
                border: `2px solid ${picking?.picture === picture ? palette.linkInk : 'transparent'}`,
              }}>{picture}</Box>
            ))}
          </Box>
          <TextField fullWidth margin="dense" label={words.youPicture} value={picking?.picture ?? ''} onChange={(event) => setPicking((was) => (was ? { ...was, picture: event.target.value } : was))} sx={{ mt: 1.5 }} />
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setPicking(null)} sx={{ color: 'text.secondary' }}>{words.cancel}</Button>
          <Button onClick={keepYou}>{words.keep}</Button>
        </DialogActions>
      </Dialog>
    </Box>
    </YouContext.Provider>
  );
});
