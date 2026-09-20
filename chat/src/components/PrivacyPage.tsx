import { memo } from 'react';
import Box from '@mui/material/Box';
import Link from '@mui/material/Link';
import Typography from '@mui/material/Typography';
import { styled } from '@mui/material/styles';
import { fonts, palette } from '../theme';

// The policy is a text a person reads, so it lives here in full. Every statement in it describes
// what this page does today; a change to the page that moves data anywhere is a change to it.
interface Section {
  heading: string;
  paragraphs: string[];
  items?: string[];
  links?: { label: string; href: string }[];
}

const policy = {
  title: 'Privacy Policy',
  company: 'Datamine Network Inc.',
  place: 'Ontario, Canada',
  effective: 'September 17, 2026',
  officer: {
    role: 'Privacy Officer',
    name: 'Michael Kloubkov',
    lines: ['Datamine Network Inc.', '146 Wandering Glider Trail', 'Bradford, ON L3Z 4K8', 'Canada'],
  },
  discordHref: 'https://discord.gg/2dQ7XAB22u',
  sections: [
    {
      heading: 'Who we are and what this covers',
      paragraphs: [
        'This service is operated by Datamine Network Inc. ("we", "us"), a corporation registered in Ontario, Canada. This policy covers the chat application at this address: its pages, the model that runs inside them, and the files the page loads. It does not cover other Datamine Network sites, which have their own policies, or the third-party services this page links to.',
        'We follow the Personal Information Protection and Electronic Documents Act (PIPEDA), Canada’s federal private-sector privacy law, and its ten fair information principles. "Personal information" means what PIPEDA means by it: information about an identifiable individual. An IP address can be personal information. Your IP address and the other details that every web request carries, listed under "Hosting on Cloudflare", are the only personal information that reaches any server when you use this page. Where this policy and the law differ, the law applies.',
      ],
    },
    {
      heading: 'The short version',
      paragraphs: [
        'The model runs inside your browser. Your prompts, the replies, your chats and what they hold never leave your device. We have no accounts, we set no cookies, we run no analytics or advertising trackers, and we build no profile of you. The only things any server sees are the ordinary web requests that load the page and one request for a public figure. Both go to the company that hosts this page for us, Cloudflare.',
      ],
    },
    {
      heading: 'Consent',
      paragraphs: [
        'By using the page you consent to the handling described here, which is limited to what is needed to deliver the page and keep it running. You withdraw that consent by stopping use of the page. Because we hold nothing of yours on any server, there is nothing further to withdraw from. Clearing the site’s data in your browser removes what the page kept on your device.',
      ],
    },
    {
      heading: 'What runs where',
      paragraphs: [
        'When you open the page, your browser downloads the application, the model (a WebAssembly build of the reader and its trained weights) and its settings. From then on, everything you type is read and answered on your device. No prompt, reply, project text or chat title is sent to us or to anyone else, at any time. The page loads no fonts, scripts or images from other companies.',
        'The reader makes no decisions about you and builds no profile of you: it answers the text you give it and nothing else. If you type personal information about yourself or about other people into a chat or a project, it stays on your device in the same way.',
        'The copy button beside a reply puts that reply on your device’s clipboard, only when you press it. As with anything you copy, other programs on your device can then read it. The page never reads your clipboard.',
      ],
    },
    {
      heading: 'What is stored in your browser',
      paragraphs: [
        'The page keeps its state in your browser’s own storage, an IndexedDB database named datamine-chats. It holds:',
      ],
      items: [
        'Your chats: each title, each prompt and reply with its time, the statements the reader listed under a reply, whether the chat is pinned, and which project it belongs to.',
        'The width you set for the side rail.',
      ],
    },
    {
      heading: 'How long it is kept, and how to delete it',
      paragraphs: [
        'Nothing else is stored: no identifier for you or your device, no record of how you use the page, and nothing from another site. That storage stays on that device and in that browser until you remove it. It is not synchronised anywhere, and we cannot see it, back it up, recover it or delete it for you. It is not encrypted by the page: anyone who can use your browser profile can read it, so on a shared device treat it like any other file on that device. An earlier version of the page kept the same record in local storage; the page moves such a record into the database on its first visit and removes the old copy.',
        'You can delete a single chat from its menu. Clearing this site’s data in your browser’s settings removes everything at once, and so does closing a private or incognito window. The page also registers a service worker that stores the application, the reader and the model files in your browser’s cache storage, so the page opens and answers with no connection, and so it can be added to your home screen as an app. These files are the same for everyone and hold nothing of yours. They are removed with the site’s data, or when a new version of the page replaces them.',
      ],
    },
    {
      heading: 'Hosting on Cloudflare',
      paragraphs: [
        'The page is served by Cloudflare Pages, a hosting and content delivery service of Cloudflare, Inc. To deliver the page, Cloudflare receives what every web request carries: your IP address, the address you asked for, your browser’s user agent, the referring page if any, and the time. Cloudflare uses this to serve the files, to protect the site against abuse, and to show us aggregate request counts. Cloudflare keeps such request logs for a limited time under its own policy, and may serve you from a data centre outside Canada, so this information can be processed outside the country and be subject to the laws there.',
        'We do not enable Cloudflare’s analytics beacon on this page. Cloudflare acts as our service provider for hosting only, and we receive no individual visitor records from it. Cloudflare is bound by its own privacy policy and by our agreement with it, and may use the information only to provide the service.',
      ],
      links: [{ label: 'Cloudflare’s privacy policy', href: 'https://www.cloudflare.com/privacypolicy/' }],
    },
    {
      heading: 'The one outside request',
      paragraphs: [
        'The page shows the available liquidity of the Datamine ecosystem. To do that it asks its own host for one public figure. The host relays that request to analytics.datamine.network, a site that Datamine Network Inc. owns and operates, so the figure never comes from a third party. The relay forwards nothing from your request: not your IP address, not your browser details, not anything you typed. Your request reaches this page’s host alone, and it carries the same details as any web request there. The figure returned is the same for everyone and is cached for a minute.',
      ],
      links: [{ label: 'analytics.datamine.network', href: 'https://analytics.datamine.network/' }],
    },
    {
      heading: 'Cookies and tracking signals',
      paragraphs: [
        'We set no cookies. Cloudflare may set a cookie of its own when its protection against automated traffic checks a request; such a cookie is used only for that check and is described in Cloudflare’s policy. The page reads no cookies and does not depend on any.',
        'Because the page tracks nothing, browser signals such as Do Not Track and Global Privacy Control are honoured as a matter of course: there is nothing to switch off.',
      ],
    },
    {
      heading: 'Disclosure',
      paragraphs: [
        'We do not sell, rent, trade or otherwise disclose personal information to anyone; we hold none to disclose. If a court order or other lawful demand ever requires us to produce information about a user, we can produce only what this policy describes, which is nothing beyond what Cloudflare holds in its own logs, and we would disclose only what the law requires.',
      ],
    },
    {
      heading: 'Open source and the model',
      paragraphs: [
        'The page, the DAM1 reader that runs inside it, and the model’s weights, facts and training quiz are open source. Datamine Network Inc. releases them under the GNU Affero General Public License, version 3 or later. The source is on GitHub, and the model is published on Hugging Face.',
        'The licence changes nothing about your privacy on this service: the reader still runs in your browser and your chats still never leave it. Downloading the source or the model from GitHub or Hugging Face is a matter between you and that service, under its own privacy policy. Anyone may run their own copy of DAM1 under the licence. This policy covers only the service Datamine Network Inc. runs at this address, not a copy someone else runs, which answers to its own operator.',
      ],
      links: [
        { label: 'DAM1 source on GitHub', href: 'https://github.com/Datamine-Crypto/DAM1' },
        { label: 'DAM1 model on Hugging Face', href: 'https://huggingface.co/DatamineNetwork/DAM1' },
        { label: 'GNU Affero General Public License', href: 'https://www.gnu.org/licenses/agpl-3.0.html' },
      ],
    },
    {
      heading: 'Links to other services',
      paragraphs: [
        'The page links to YouTube (the launch video), Discord, GitHub (the source code, the token audits, and the Datamine dashboard, which GitHub hosts), Hugging Face, Uniswap and DefiLlama. This policy also links to Cloudflare, the GNU licence and the Office of the Privacy Commissioner of Canada. Each link opens in a new tab, and the page tells the other site nothing about where you came from. When you follow a link you leave this service, and the privacy policy of that service applies. Nothing about your chats travels with the link, and we share nothing with those services.',
        'The trade links open trading interfaces for public blockchains. Anything you do there happens on a public chain: transactions, balances and wallet addresses on such a chain are public, permanent and outside our control. This page does not connect to a wallet and never sees one.',
      ],
    },
    {
      heading: 'Security and breaches',
      paragraphs: [
        'The page is served only over HTTPS, which protects the files you load and the one figure the page fetches against being read or changed in transit. It is also served with a Content Security Policy that forbids the browser from loading any script, style, image or connection from another origin, and forbids other sites from framing it. Because we hold no user data on any server, there is no server-side store of yours to breach. The remaining risk is the device itself, which is why the section on local storage matters.',
        'If a breach of security safeguards ever involves personal information under our control and creates a real risk of significant harm, we will notify the individuals affected and report to the Privacy Commissioner of Canada, as PIPEDA requires, and keep the record of it that the law requires.',
      ],
    },
    {
      heading: 'Children',
      paragraphs: [
        'The service is not directed at children. We ask no one for personal information, children included, and the only information that reaches a server is the hosting data every visit carries. A parent or guardian with a concern can reach us as described below.',
      ],
    },
    {
      heading: 'Your rights',
      paragraphs: [
        'Under PIPEDA you may ask what personal information we hold about you, ask for it to be corrected, withdraw consent, and challenge our compliance with this policy. We answer such requests within thirty days and at no cost. Because this service keeps your information only in your own browser, the answer to the first question will be: nothing beyond the request logs described above, which Cloudflare holds under its own policy and which we do not tie to a person. Everything else you hold yourself and can delete at any time.',
        'If you are not satisfied with our answer, you may complain to the Office of the Privacy Commissioner of Canada.',
      ],
      links: [{ label: 'Office of the Privacy Commissioner of Canada', href: 'https://www.priv.gc.ca/' }],
    },
    {
      heading: 'Visitors outside Canada',
      paragraphs: [
        'If you use the service from elsewhere, the same facts apply: no account, no tracking, and nothing collected beyond the hosting data described above.',
      ],
      items: [
        'European Union, European Economic Area and United Kingdom: Datamine Network Inc. is the controller of the hosting data, and Cloudflare processes it on our behalf. Our legal basis is our legitimate interest in delivering the page and keeping it secure. You may ask for access, correction, erasure, restriction or portability, object to the processing, and complain to your data protection authority. Cloudflare may process the data outside your country under the transfer safeguards described in its policy.',
        'California and other US states: we do not sell or share personal information as those laws define the terms, and we do not use it for targeted advertising or profiling.',
        'Anywhere else: where your local law gives you rights over your information, contact us and we will answer under that law as far as it applies to us. In nearly every case the answer will be that we hold nothing.',
      ],
    },
    {
      heading: 'Changes to this policy',
      paragraphs: [
        'If the service ever starts to collect, store or send more than described here, this policy will be updated before that change takes effect, and the effective date at the top will change. The date at the top always shows the version in force. Earlier versions are available from us on request.',
      ],
    },
    {
      heading: 'Governing law',
      paragraphs: [
        'This policy is governed by the laws of the Province of Ontario and the federal laws of Canada that apply there, without taking away any protection that the law where you live gives you.',
      ],
    },
    {
      heading: 'Accountability and contact',
      paragraphs: [
        'Datamine Network Inc. has designated a Privacy Officer who is accountable for this policy and answers questions, requests and complaints about it. Write to the address below. The team can also be reached on Discord for a faster answer; Discord is a third-party service with its own privacy policy, so a message there is seen by Discord as well.',
      ],
    },
  ] as Section[],
} as const;

const Page = styled('div')(({ theme }) => ({
  flex: 1,
  overflowY: 'auto',
  minHeight: 0,
  padding: theme.spacing(4, 3, 6),
}));

const Column = styled('article')(({ theme }) => ({
  maxWidth: theme.spacing(90),
  margin: '0 auto',
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(3),
}));

const Paragraph = styled(Typography)(({ theme }) => ({
  color: theme.palette.text.secondary,
  fontSize: '0.9375rem',
  lineHeight: 1.6,
}));

const Items = styled('ul')(({ theme }) => ({
  margin: 0,
  paddingLeft: theme.spacing(2.5),
  color: theme.palette.text.secondary,
  fontSize: '0.9375rem',
  lineHeight: 1.6,
  display: 'flex',
  flexDirection: 'column',
  gap: theme.spacing(0.5),
}));

const Address = styled('address')(({ theme }) => ({
  fontStyle: 'normal',
  color: theme.palette.text.primary,
  fontSize: '0.9375rem',
  lineHeight: 1.6,
  padding: theme.spacing(2, 2.5),
  borderRadius: 12,
  background: palette.paper,
  border: `1px solid ${palette.divider}`,
}));

export const PrivacyPage = memo(function PrivacyPage() {
  return (
    <Page>
      <Column>
        <Box>
          <Typography component="h1" sx={{ fontFamily: fonts.serif, fontSize: '2rem', fontWeight: 400, lineHeight: 1.2 }}>{policy.title}</Typography>
          <Typography sx={{ color: 'text.secondary', fontSize: '0.875rem', mt: 1 }}>
            {policy.company}, {policy.place}. Effective {policy.effective}.
          </Typography>
        </Box>
        {policy.sections.map((section) => (
          <Box key={section.heading} component="section" sx={{ display: 'flex', flexDirection: 'column', gap: 1 }}>
            <Typography component="h2" sx={{ fontSize: '1.0625rem', fontWeight: 600 }}>{section.heading}</Typography>
            {section.paragraphs.map((paragraph) => <Paragraph key={paragraph.slice(0, 48)}>{paragraph}</Paragraph>)}
            {section.items && (
              <Items>
                {section.items.map((item) => <li key={item.slice(0, 48)}>{item}</li>)}
              </Items>
            )}
            {section.links && section.links.map((link) => (
              <Link key={link.href} href={link.href} target="_blank" rel="noopener noreferrer" sx={{ color: palette.linkInk, fontSize: '0.9375rem', alignSelf: 'flex-start' }}>
                {link.label}
              </Link>
            ))}
          </Box>
        ))}
        <Address>
          <Box sx={{ fontWeight: 600 }}>{policy.officer.name}, {policy.officer.role}</Box>
          {policy.officer.lines.map((line) => <Box key={line}>{line}</Box>)}
          <Link href={policy.discordHref} target="_blank" rel="noopener noreferrer" sx={{ color: palette.linkInk, display: 'inline-block', mt: 1 }}>
            {policy.discordHref}
          </Link>
        </Address>
      </Column>
    </Page>
  );
});
