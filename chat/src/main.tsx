import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { App } from './App';
import { registerOffline } from './install';
import { useChatStore } from './store';
import { muiTheme } from './theme';

registerOffline();
// The engine starts loading the moment the page is entered, in its worker thread, so the network is
// on the device and in memory by the time the first prompt is sent; the page never waits on it.
void useChatStore.getState().ensureAgent();

const rootElement = document.getElementById('root');
if (rootElement) {
  createRoot(rootElement).render(
    <StrictMode>
      <ThemeProvider theme={muiTheme}>
        <CssBaseline />
        <App />
      </ThemeProvider>
    </StrictMode>,
  );
}
