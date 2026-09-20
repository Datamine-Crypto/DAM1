import { memo, useEffect, useRef, useState, type ReactNode } from 'react';
import Box from '@mui/material/Box';
import IconButton from '@mui/material/IconButton';
import LinearProgress from '@mui/material/LinearProgress';
import Tooltip from '@mui/material/Tooltip';
import Typography from '@mui/material/Typography';
import ContentCopyIcon from '@mui/icons-material/ContentCopy';
import CheckIcon from '@mui/icons-material/Check';
import AllInclusiveIcon from '@mui/icons-material/AllInclusive';
import SearchOffIcon from '@mui/icons-material/SearchOff';
import TaskAltIcon from '@mui/icons-material/TaskAlt';
import UnfoldLessIcon from '@mui/icons-material/UnfoldLess';
import UnfoldMoreIcon from '@mui/icons-material/UnfoldMore';
import { styled } from '@mui/material/styles';
import { create } from 'zustand';
import type { Live, Message } from '../store';
import { fonts, palette, spacing } from '../theme';
import { Composer } from './Composer';
import { Mascot } from './Mark';

const Column = styled('div')(({ theme }) => ({
  width: '100%',
  maxWidth: theme.spacing(spacing.conversationMaxWidthUnits),
  margin: '0 auto',
  padding: theme.spacing(3, 2),
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(3),
}));

const UserBubble = styled('div')(({ theme }) => ({
  alignSelf: 'flex-end',
  maxWidth: '80%',
  padding: theme.spacing(1.25, 2),
  borderRadius: 12,
  background: palette.userBubble,
  whiteSpace: 'pre-wrap',
  overflowWrap: 'anywhere',
}));

// The marks a reply can carry: told and kept, asked and found nothing, or a number without end.
const marks = {
  noted: { label: 'Noted', Icon: TaskAltIcon, ink: palette.mutedInk },
  nothing: { label: 'Nothing', Icon: SearchOffIcon, ink: palette.mutedInk },
  infinity: { label: 'Infinity', Icon: AllInclusiveIcon, ink: palette.mutedInk },
} as const;

const Mark = styled('span')<{ ink: string }>(({ theme, ink }) => ({
  display: 'inline-flex',
  alignItems: 'center',
  gap: theme.spacing(0.75),
  padding: theme.spacing(0.25, 1.25, 0.25, 0.75),
  borderRadius: 999,
  border: `1px solid ${ink}`,
  color: ink,
  fontSize: '0.8125rem',
  fontWeight: 500,
  lineHeight: 1.6,
  alignSelf: 'flex-start',
}));

const AgentText = styled('div')({
  fontFamily: fonts.serif,
  fontSize: '1.125rem',
  lineHeight: 1.55,
  whiteSpace: 'pre-wrap',
  overflowWrap: 'anywhere',
});

// What the reader kept from a turn, as badges wrapping as the column allows.
const Detail = styled('div')(({ theme }) => ({
  margin: theme.spacing(0.5, 0, 0),
  display: 'flex',
  flexWrap: 'wrap',
  alignItems: 'flex-start',
  gap: theme.spacing(0.5),
}));

const StatementBadge = styled('span')(({ theme }) => ({
  display: 'inline-flex',
  flexWrap: 'wrap',
  alignItems: 'center',
  gap: 4,
  padding: '0 6px',
  borderRadius: 999,
  border: `1px solid ${palette.divider}`,
  background: palette.hover,
  color: theme.palette.text.secondary,
  fontFamily: fonts.mono,
  fontSize: '0.6875rem',
  lineHeight: 1.5,
  overflowWrap: 'anywhere',
}));

// A thing with what is said of it inside: rounder corners than a pill so it can wrap to more
// lines, and each level inside is a shade lighter as the backgrounds stack.
const ThingBadge = styled('span')(({ theme }) => ({
  display: 'inline-flex',
  flexWrap: 'wrap',
  alignItems: 'center',
  gap: 4,
  padding: '1px 2px 1px 6px',
  borderRadius: 9,
  border: `1px solid ${palette.divider}`,
  background: palette.hover,
  color: theme.palette.text.primary,
  fontFamily: fonts.mono,
  fontSize: '0.6875rem',
  lineHeight: 1.5,
  overflowWrap: 'anywhere',
}));

// The relations the reader writes with fixed words (reader.rs IN_FORM, IS_FORM, OWNS_FORM,
// WORTH_FORM). Any other relation is an open word, found only when the statement is three words,
// since a name of two words cannot be told from a relation in plain text.
const fixedRelations = [' is ', ' in ', ' owns ', ' = '] as const;

function relationOf(line: string): [string, string, string] | null {
  for (const form of fixedRelations) {
    const at = line.indexOf(form);
    if (at > 0) return [line.slice(0, at), form.trim(), line.slice(at + form.length)];
  }
  const words = line.split(' ');
  return words.length === 3 ? [words[0], words[1], words[2]] : null;
}

// Statements gathered into a tree, so the context found reads as badges inside each other: a thing
// holds what is said of it, a thing in a place sits inside that place, and an object with
// statements of its own opens inside the statement that names it. Each thing is shown once; a
// line the page cannot split stays a badge of its own.
interface Thing {
  name: string;
  facts: { relation: string; object: Thing | string }[];
  inside: Thing[];
}

function treeOf(lines: string[]): { things: Thing[]; loose: string[] } {
  const said = new Map<string, { facts: [string, string][]; inside: string[] }>();
  const entry = (name: string) => {
    const known = said.get(name);
    if (known) return known;
    const fresh = { facts: [] as [string, string][], inside: [] as string[] };
    said.set(name, fresh);
    return fresh;
  };
  const placed = new Set<string>();
  const named = new Set<string>();
  const loose: string[] = [];
  for (const line of lines) {
    const parts = relationOf(line);
    if (!parts) {
      loose.push(line);
      continue;
    }
    const [subject, relation, object] = parts;
    if (relation === 'in' && subject !== object && !placed.has(subject)) {
      entry(subject);
      placed.add(subject);
      entry(object).inside.push(subject);
    } else {
      entry(subject).facts.push([relation, object]);
      named.add(object);
    }
  }
  const shown = new Set<string>();
  const holds = (name: string): boolean => {
    const known = said.get(name);
    return known !== undefined && (known.facts.length > 0 || known.inside.length > 0);
  };
  const build = (name: string): Thing => {
    shown.add(name);
    const known = entry(name);
    const facts = known.facts.map(([relation, object]) => ({ relation, object: holds(object) && !shown.has(object) ? build(object) : object }));
    const inside: Thing[] = [];
    for (const child of known.inside) if (!shown.has(child)) inside.push(build(child));
    return { name, facts, inside };
  };
  // Things nothing points to are the roots; the rest open inside them, and any left over by a
  // cycle start a tree of their own.
  const nested = (name: string): number => Number(named.has(name) || placed.has(name));
  const things: Thing[] = [];
  for (const name of [...said.keys()].sort((a, b) => nested(a) - nested(b))) {
    if (!shown.has(name) && holds(name)) things.push(build(name));
  }
  return { things, loose };
}

const ThingView = memo(function ThingView({ thing }: { thing: Thing }) {
  if (thing.facts.length === 0 && thing.inside.length === 0) return <StatementBadge>{thing.name}</StatementBadge>;
  return (
    <ThingBadge>
      <span>{thing.name}</span>
      {thing.facts.map((fact, at) => (
        <StatementBadge key={`${at}-${fact.relation}`}>
          <Box component="span" sx={{ color: palette.linkInk }}>{fact.relation}</Box>
          {typeof fact.object === 'string' ? <span>{fact.object}</span> : <ThingView thing={fact.object} />}
        </StatementBadge>
      ))}
      {thing.inside.map((child) => <ThingView key={child.name} thing={child} />)}
    </ThingBadge>
  );
});

const StatementTree = memo(function StatementTree({ lines }: { lines: string[] }) {
  const { things, loose } = treeOf(lines);
  return (
    <Detail>
      {things.map((thing) => <ThingView key={thing.name} thing={thing} />)}
      {loose.map((line, at) => <StatementBadge key={`${at}-${line}`}>{line}</StatementBadge>)}
    </Detail>
  );
});

const CopyButton = memo(function CopyButton({ text }: { text: string }) {
  const [copied, setCopied] = useState(false);
  const copy = async (): Promise<void> => {
    try {
      await navigator.clipboard.writeText(text);
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    } catch {
      // A blocked clipboard is not an error the reader needs to hear about.
    }
  };
  return (
    <Tooltip title={copied ? 'Copied' : 'Copy'}>
      <IconButton size="small" aria-label="Copy" onClick={copy} sx={{ color: 'text.secondary' }}>
        {copied ? <CheckIcon fontSize="inherit" /> : <ContentCopyIcon fontSize="inherit" />}
      </IconButton>
    </Tooltip>
  );
});

const detailWords = {
  hide: 'Hide all statements',
  show: 'Show all statements',
} as const;

// Whether the statements under replies are shown, one choice for every reply at once. It is held in
// memory only, so a reload hides them again and nothing is written to the browser's storage.
const useStatementsShown = create<{ open: boolean; toggle: () => void }>()((set) => ({
  open: false,
  toggle: () => set((state) => ({ open: !state.open })),
}));

// Beside the copy button: shows or hides the statements under every reply.
const DetailToggle = memo(function DetailToggle() {
  const { open, toggle } = useStatementsShown();
  const label = open ? detailWords.hide : detailWords.show;
  return (
    <Tooltip title={label}>
      <IconButton size="small" aria-label={label} aria-expanded={open} onClick={toggle} sx={{ color: 'text.secondary' }}>
        {open ? <UnfoldLessIcon fontSize="inherit" /> : <UnfoldMoreIcon fontSize="inherit" />}
      </IconButton>
    </Tooltip>
  );
});

// The copy button and the statements toggle stay visible on the last reply; on earlier ones they
// appear on hover or focus.
const AgentMessage = memo(function AgentMessage({ message, last }: { message: Message; last: boolean }) {
  const open = useStatementsShown((state) => state.open);
  const count = message.detail?.length ?? 0;
  return (
    <Box sx={{ display: 'flex', flexDirection: 'column', gap: 0.75, '& .actions': { opacity: last ? 1 : 0, transition: 'opacity 120ms' }, '&:hover .actions, &:focus-within .actions': { opacity: 1 }, '@media (hover: none)': { '& .actions': { opacity: 1 } } }}>
      {message.kind && (() => {
        const mark = marks[message.kind];
        return <Mark ink={mark.ink}><mark.Icon sx={{ fontSize: 16 }} />{mark.label}</Mark>;
      })()}
      {message.text !== '' && <AgentText>{message.text}</AgentText>}
      {count > 0 && open && message.detail && <StatementTree lines={message.detail} />}
      {(message.text !== '' || count > 0) && (
        <Box className="actions" sx={{ display: 'flex', alignItems: 'center', gap: 1, ml: -0.5 }}>
          {message.text !== '' && <CopyButton text={message.text} />}
          {count > 0 && <DetailToggle />}
        </Box>
      )}
    </Box>
  );
});

// What the network is doing, in its own step classes, at the word it took the step at.
function actionLine(step: Live['actions'][number]): string {
  return `${step.action}: ${step.word}`;
}

// While a turn runs, one bar floats above the composer in a fixed place: the phase, the latest
// step, and a thin running line. Only the words in the bar change, never the layout around it.
const ProgressShell = styled('div')(({ theme }) => ({
  position: 'absolute',
  left: 0,
  right: 0,
  bottom: '100%',
  marginBottom: theme.spacing(1),
  height: 36,
  display: 'flex',
  alignItems: 'center',
  gap: theme.spacing(1.25),
  padding: theme.spacing(0, 1.75),
  borderRadius: 12,
  background: palette.paper,
  border: `1px solid ${palette.divider}`,
  boxShadow: `0 6px 18px ${palette.surfaceShadow}`,
  overflow: 'hidden',
  pointerEvents: 'none',
}));

const phases = { reading: 'Reading', thinking: 'Thinking', answering: 'Answering' } as const;

// How far a turn has come, from 0 to 1: reading the prompt fills the first half one token at a time,
// and saying the reply fills the second half one word at a time. While the reader works out the reply
// between the two, it sends nothing, and the bar waits at the half.
const readShare = 0.5;

function turnShare(live: Live | null): number {
  if (!live) return 0;
  if (live.outputOf > 0) return readShare + (1 - readShare) * Math.min(1, live.outputAt / live.outputOf);
  const read = live.inputOf > 0 ? Math.min(1, live.input.length / live.inputOf) : 0;
  return readShare * read;
}

const ProgressBar = memo(function ProgressBar({ live }: { live: Live | null }) {
  const input = live?.input ?? [];
  const actions = live?.actions ?? [];
  const output = live?.output ?? [];
  const phase = output.length > 0 ? phases.answering : actions.length > 0 ? phases.thinking : phases.reading;
  const lines = actions.map(actionLine).filter((line): line is string => line !== null);
  const step = output.length > 0 ? '' : lines.at(-1) ?? input.at(-1) ?? '';
  const percent = Math.round(turnShare(live) * 100);
  return (
    <ProgressShell role="status" aria-live="off">
      <Typography component="span" sx={{ fontSize: '0.8125rem', fontWeight: 500, color: 'text.primary', flexShrink: 0, minWidth: 72 }}>{phase}</Typography>
      <Typography component="span" noWrap sx={{ flex: 1, minWidth: 0, fontSize: '0.8125rem', fontFamily: fonts.mono, color: 'text.secondary' }}>{step}</Typography>
      <Typography component="span" sx={{ flexShrink: 0, fontSize: '0.75rem', fontVariantNumeric: 'tabular-nums', color: 'text.secondary' }}>{`${percent}%`}</Typography>
      <LinearProgress variant="determinate" value={percent} aria-label={phase}
        sx={{ position: 'absolute', left: 0, right: 0, bottom: 0, height: 3, backgroundColor: palette.divider, '& .MuiLinearProgress-bar': { backgroundColor: palette.linkInk, transition: 'transform 120ms linear' } }} />
    </ProgressShell>
  );
});

const mindLabel = 'Mind Explorer';

export interface ConversationProps {
  messages: Message[];
  thinking: boolean;
  live: Live | null;
  placeholder: string;
  disabled: boolean;
  onSend: (text: string) => void;
  footnote: ReactNode;
  // Opens the mind explorer on this chat, where the chat plays back as a game.
  onMind?: () => void;
  // Shown to be read only, as in the explorer's dialog: no box to write in and no mascot under the last message.
  readOnly?: boolean;
}

// A message just sent shows greyed out and turns white one letter at a time, as the reader takes it in.
// A message older than the window, as on opening a chat again, shows whole at once.
const revealWindowMs = 3000;
const revealFrames = 90;
const unrevealedOpacity = 0.4;

function reducedMotion(): boolean {
  return typeof window !== 'undefined' && window.matchMedia?.('(prefers-reduced-motion: reduce)').matches === true;
}

const RevealText = memo(function RevealText({ text, at }: { text: string; at: number }) {
  const [shown, setShown] = useState(() => (reducedMotion() || Date.now() - at > revealWindowMs ? text.length : 0));
  useEffect(() => {
    if (shown >= text.length) return undefined;
    const step = Math.max(1, Math.ceil(text.length / revealFrames));
    const frame = requestAnimationFrame(() => setShown((n) => Math.min(text.length, n + step)));
    return () => cancelAnimationFrame(frame);
  }, [shown, text]);
  return (
    <>
      {text.slice(0, shown)}
      <span style={{ opacity: unrevealedOpacity }}>{text.slice(shown)}</span>
    </>
  );
});

export const Conversation = memo(function Conversation({ messages, thinking, live, placeholder, disabled, onSend, footnote, onMind, readOnly = false }: ConversationProps) {
  const end = useRef<HTMLDivElement>(null);
  const streamed = live ? live.output.length : 0;

  useEffect(() => {
    end.current?.scrollIntoView({ block: 'end' });
  }, [messages.length, thinking, streamed]);

  return (
    <Box sx={{ flex: 1, display: 'flex', flexDirection: 'column', minHeight: 0 }}>
      <Box sx={{ flex: 1, overflowY: 'auto', minHeight: 0 }}>
        <Column>
          {messages.map((message, at) => (
            message.role === 'user'
              ? <UserBubble key={message.id}><RevealText text={message.text} at={message.at} /></UserBubble>
              : <AgentMessage key={message.id} message={message} last={at === messages.length - 1} />
          ))}
          {thinking && live && live.output.length > 0 && <AgentText>{live.output.join(' ')}</AgentText>}
          {!readOnly && <Box sx={{ display: 'flex', alignItems: 'center', gap: 1.5, minHeight: 32 }}>
            <Mascot busy={thinking} />
            {onMind && !thinking && messages.length > 0 && (
              <Box component="button" type="button" onClick={onMind} sx={{
                border: `1px solid ${palette.linkInk}`, color: palette.linkInk, background: 'transparent', borderRadius: 999, px: 1.5, py: 0.25, fontSize: '0.8125rem',
                fontFamily: 'inherit', cursor: 'pointer', '&:hover': { background: 'rgba(0, 255, 255, 0.1)' },
              }}>{mindLabel}</Box>
            )}
          </Box>}
          <div ref={end} />
        </Column>
      </Box>
      {!readOnly && <Box sx={{ px: 2, pt: 1, pb: 'calc(16px + env(safe-area-inset-bottom))' }}>
        <Box sx={{ position: 'relative', width: '100%', maxWidth: (theme) => theme.spacing(spacing.conversationMaxWidthUnits), mx: 'auto' }}>
          {thinking && <ProgressBar live={live} />}
          <Composer placeholder={placeholder} disabled={disabled} onSend={onSend} autoFocus />
          <Typography sx={{ color: 'text.secondary', fontSize: '0.75rem', textAlign: 'center', pt: 1, whiteSpace: { md: 'nowrap' } }}>{footnote}</Typography>
        </Box>
      </Box>}
    </Box>
  );
});
