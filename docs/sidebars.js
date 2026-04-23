// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  guideSidebar: [
    'intro',
    'why-boincrs',
    'getting-started',
    'installation',
    'configuration',
    'usage',
    'keyboard',
    'accessibility',
    'compatibility',
    'testing',
    {
      type: 'category',
      label: 'Architecture',
      collapsed: false,
      items: [
        'architecture/app-controller',
        'architecture/beta-primegrid-asteroids',
        'architecture/project-templates-and-profiles',
        'architecture/smoke-checklist',
      ],
    },
    {
      type: 'category',
      label: 'Decisions',
      collapsed: true,
      items: ['decisions/0001-error-handling'],
    },
    'release-checklist',
    'roadmap',
    'changelog',
    'contributing',
    'support',
  ],
};

export default sidebars;
