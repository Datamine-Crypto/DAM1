import { createTheme } from '@mui/material/styles';

// The surfaces of analytics.datamine.network: the ground, the panel and the rail. The one
// accent is the same cyan it gives its links.
export const palette = {
  pageBackground: '#202336',
  paper: '#272936',
  nestedPaper: '#22242e',
  sunk: '#1b1e2e',
  ink: '#ffffff',
  mutedInk: '#8b93a7',
  linkInk: '#0ff',
  divider: 'rgba(255, 255, 255, 0.12)',
  hover: 'rgba(255, 255, 255, 0.06)',
  selected: 'rgba(255, 255, 255, 0.1)',
  outlinedButtonBorder: 'rgba(255, 255, 255, 0.23)',
  userBubble: '#373c4e',
  campaignBanner: '#1b1e2e',
  discordButton: '#40486c',
  discordButtonHover: '#333851',
  surfaceShadow: 'rgba(0, 0, 0, 0.35)',
} as const;

export const spacing = {
  unitPixels: 8,
  drawerWidthUnits: 31,
  topBarHeightUnits: 7,
  conversationMaxWidthUnits: 92,
} as const;

export const fonts = {
  serif: '"Georgia", "Times New Roman", "Iowan Old Style", serif',
  mono: 'ui-monospace, "Cascadia Mono", Consolas, monospace',
} as const;

export const muiTheme = createTheme({
  cssVariables: true,
  spacing: spacing.unitPixels,
  palette: {
    mode: 'dark',
    primary: { main: '#fff', contrastText: palette.linkInk },
    secondary: { main: '#00FFFF', contrastText: '#fff' },
    background: { default: palette.pageBackground, paper: palette.paper },
    text: { primary: palette.ink, secondary: palette.mutedInk },
    divider: palette.divider,
    contrastThreshold: 3,
    tonalOffset: 0.2,
  },
  shape: { borderRadius: 8 },
  typography: {
    button: { textTransform: 'none', fontWeight: 400 },
  },
  components: {
    MuiTextField: { defaultProps: { size: 'small' } },
    MuiButton: { styleOverrides: { root: { fontWeight: 400, textTransform: 'none' } } },
    MuiPaper: { styleOverrides: { root: { backgroundColor: palette.paper, backgroundImage: 'none', '--Paper-overlay': 'none !important' } } },
    MuiDrawer: { styleOverrides: { paper: { backgroundColor: palette.nestedPaper, borderRight: `1px solid ${palette.divider}` } } },
    MuiListItemButton: {
      styleOverrides: {
        root: {
          borderRadius: 8,
          '&.Mui-selected': { backgroundColor: palette.selected },
          '&.Mui-selected:hover': { backgroundColor: palette.selected },
          '&:hover': { backgroundColor: palette.hover },
        },
      },
    },
    MuiTooltip: { styleOverrides: { tooltip: { backgroundColor: palette.sunk, border: `1px solid ${palette.divider}`, fontSize: '0.75rem' } } },
  },
});
