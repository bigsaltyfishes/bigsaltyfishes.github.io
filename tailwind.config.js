/** @type {import('tailwindcss').Config} */
const { withMaterialColors } = require('tailwind-material-colors');

const config = {
  // Configure files to scan for Tailwind classes
  content: [
    "./src/**/*.rs",
    "./index.html",
  ],
  
  // Enable dark mode using a 'class' strategy
  darkMode: 'selector',

  theme: {
    extend: {
      // Define custom max-width for content area
      maxWidth: {
        'content': '1200px',
      },
      // Define your font families
      fontFamily: {
        sans: ['"JetBrains Mono"', '"Noto Sans SC"', '"Microsoft YaHei"', '"PingFang SC"', '"Hiragino Sans GB"', '"Source Han Sans CN"', '"WenQuanYi Micro Hei"', 'sans-serif'],
        mono: ['"JetBrains Mono"', '"Noto Sans SC"', '"Cascadia Code"', 'Consolas', '"Microsoft YaHei"', '"Courier New"', 'monospace'],
      },
      // Define custom font sizes
      fontSize: {
        'page-title': '2.0rem',
        'article-list-item-title': '1.15rem',
        'markdown-h1': '1.8rem',
        'normal': '1rem',
        'small': '0.9rem',
        'site-title': '1.25rem',
      },        // Configure typography styles provided by the @tailwindcss/typography plugin
      typography: ({ theme }) => ({
        DEFAULT: {
          css: {
            '--tw-prose-body': theme('colors.on-background'),
            '--tw-prose-headings': theme('colors.on-background'),
            '--tw-prose-links': theme('colors.primary'),
            '--tw-prose-bold': theme('colors.on-background'),
            '--tw-prose-counters': theme('colors.on-surface-variant'),
            '--tw-prose-bullets': theme('colors.on-surface-variant'),
            '--tw-prose-hr': theme('colors.outline'),
            '--tw-prose-quotes': theme('colors.on-background'),
            '--tw-prose-quote-borders': theme('colors.primary'),
            '--tw-prose-captions': theme('colors.on-surface-variant'),
            '--tw-prose-code': theme('colors.on-surface-variant'),
            '--tw-prose-pre-code': theme('colors.on-surface'),
            '--tw-prose-pre-bg': theme('colors.surface-variant'),
            '--tw-prose-invert-body': theme('colors.on-background'),
            '--tw-prose-invert-headings': theme('colors.on-background'),
            '--tw-prose-invert-links': theme('colors.primary'),
            '--tw-prose-invert-bold': theme('colors.on-background'),
            '--tw-prose-invert-counters': theme('colors.on-surface-variant'),
            '--tw-prose-invert-bullets': theme('colors.on-surface-variant'),
            '--tw-prose-invert-hr': theme('colors.outline'),
            '--tw-prose-invert-quotes': theme('colors.on-background'),
            '--tw-prose-invert-quote-borders': theme('colors.primary'),
            '--tw-prose-invert-captions': theme('colors.on-surface-variant'),
            '--tw-prose-invert-code': theme('colors.on-surface-variant'),
            '--tw-prose-invert-pre-code': theme('colors.on-surface'),
            '--tw-prose-invert-pre-bg': theme('colors.surface-variant'),
          },
        },
      }),
      // Define custom keyframes for animations
      keyframes: {
        'fade-in-up': {
          'from': { opacity: '0', transform: 'translateY(20px)' },
          'to': { opacity: '1', transform: 'translateY(0)' },
        },
        'overlay-fade-in': {
            'from': { opacity: '0' },
            'to': { opacity: '1' },
        },
        'overlay-fade-out': {
            'from': { opacity: '1' },
            'to': { opacity: '0' },
        },
        'slide-in-from-right': {
            'from': { transform: 'translateX(100%)' },
            'to': { transform: 'translateX(0)' },
        },
        'slide-out-to-right': {
            'from': { transform: 'translateX(0)' },
            'to': { transform: 'translateX(100%)' },
        },
        spin: {
            'from': { transform: 'rotate(0deg)' },
            'to': { transform: 'rotate(360deg)' },
        },
      },
      // Define custom animations
      animation: {
        'fade-in-up': 'fade-in-up 0.5s ease-out forwards',
        'overlay-fade-in': 'overlay-fade-in 0.1s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'overlay-fade-out': 'overlay-fade-out 0.1s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'slide-in-from-right': 'slide-in-from-right 0.1s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'slide-out-to-right': 'slide-out-to-right 0.1s cubic-bezier(0.16, 1, 0.3, 1) forwards',
        'spin': 'spin 1s linear infinite',
      },
    },
    // Define your breakpoints
    screens: {
      'sm': '480px',
      'md': '768px',
    }
  },
  // Add the typography plugin
  plugins: [
    require('@tailwindcss/typography'),
  ],
}

module.exports = withMaterialColors(config, {
  // Your base colors as HEX values. 'primary' is required.
  primary: '#ff0000',
  secondary: '#ffff00',
  tertiary: '#0000ff',
},
{
  /* one of 'content', 'expressive', 'fidelity', 'monochrome', 'neutral', 'tonalSpot' or 'vibrant' */
  scheme: 'neutral',
  // contrast is optional and ranges from -1 (less contrast) to 1 (more contrast).
  contrast: 0,
});
