// @ts-check
// `@type` JSDoc annotations allow editor autocompletion and type-checking
// (when paired with `@ts-check`).
// See: https://docusaurus.io/docs/api/docusaurus-config

import { themes as prismThemes } from 'prism-react-renderer';

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'boincrs',
  tagline: 'A fast, accessible terminal UI for your local BOINC client.',
  favicon: 'img/favicon.svg',

  url: 'https://jakenherman.github.io',
  baseUrl: '/boincrs/',

  organizationName: 'jakenherman',
  projectName: 'boincrs',
  trailingSlash: false,

  onBrokenLinks: 'warn',

  markdown: {
    mermaid: true,
    hooks: {
      onBrokenMarkdownLinks: 'warn',
    },
  },

  themes: ['@docusaurus/theme-mermaid'],

  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          path: 'guide',
          routeBasePath: 'guide',
          sidebarPath: './sidebars.js',
          editUrl:
            'https://github.com/jakenherman/boincrs/edit/main/docs/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      image: 'img/boincrs-social-card.jpg',
      colorMode: {
        defaultMode: 'dark',
        respectPrefersColorScheme: true,
      },
      navbar: {
        title: 'boincrs',
        logo: {
          alt: 'boincrs',
          src: 'img/logo.svg',
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'guideSidebar',
            position: 'left',
            label: 'Guide',
          },
          {
            to: '/guide/getting-started',
            label: 'Getting started',
            position: 'left',
          },
          {
            to: '/guide/architecture/app-controller',
            label: 'Architecture',
            position: 'left',
          },
          {
            href: 'https://github.com/jakenherman/boincrs',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              { label: 'Introduction', to: '/guide/intro' },
              { label: 'Getting started', to: '/guide/getting-started' },
              { label: 'Configuration', to: '/guide/configuration' },
              { label: 'Keyboard reference', to: '/guide/keyboard' },
            ],
          },
          {
            title: 'Project',
            items: [
              { label: 'Changelog', to: '/guide/changelog' },
              { label: 'Roadmap', to: '/guide/roadmap' },
              { label: 'Compatibility', to: '/guide/compatibility' },
              { label: 'Contributing', to: '/guide/contributing' },
            ],
          },
          {
            title: 'Community',
            items: [
              {
                label: 'BOINC',
                href: 'https://boinc.berkeley.edu/',
              },
              {
                label: 'PrimeGrid',
                href: 'https://www.primegrid.com/',
              },
              {
                label: 'Asteroids@home',
                href: 'https://asteroidsathome.net/boinc/',
              },
            ],
          },
          {
            title: 'Source',
            items: [
              {
                label: 'GitHub',
                href: 'https://github.com/jakenherman/boincrs',
              },
              {
                label: 'Sponsor',
                href: 'https://github.com/sponsors/JakenHerman',
              },
              {
                label: 'Issues',
                href: 'https://github.com/jakenherman/boincrs/issues',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Jaken Herman. boincrs is MIT-licensed.`,
      },
      prism: {
        theme: prismThemes.github,
        darkTheme: prismThemes.dracula,
        additionalLanguages: ['bash', 'toml', 'rust', 'diff'],
      },
      mermaid: {
        theme: { light: 'neutral', dark: 'dark' },
      },
      docs: {
        sidebar: {
          hideable: true,
          autoCollapseCategories: false,
        },
      },
    }),
};

export default config;
