# Seven Bridge Portal Design System V1
*Browser UI Design Standard - Full Build*

## Overview
The Seven Bridge Portal Design System is inspired by Milwaukee/Grant Park aesthetics combined with LVMH premium typography and Gwern.net academic styling. This system provides a cohesive, professional foundation for all browser UI components.

## Core Philosophy
- **Location-inspired**: Milwaukee/Grant Park natural elements
- **Premium Typography**: LVMH luxury brand standards
- **Academic Clarity**: Gwern.net readability principles
- **Professional Interface**: Clean, accessible, and consistent

---

## Color Palette

### Primary Colors
```css
:root {
  /* Milwaukee/Grant Park Color Palette */
  --lake-michigan-blue: #1e3a8a;      /* Primary brand color */
  --wisconsin-gold: #f59e0b;          /* Accent/highlight color */
  --stone-bridge-gray: #6b7280;       /* Neutral base */
  --ravine-green: #059669;            /* Success/positive actions */
  --limestone-white: #fefefe;         /* Background base */
  --charcoal-text: #1f2937;          /* Primary text */
  --mist-gray: #f3f4f6;              /* Light backgrounds */
  --bridge-shadow: rgba(0, 0, 0, 0.1); /* Subtle shadows */
}
```

### Color Usage Guidelines

#### Primary Actions & Navigation
- **Lake Michigan Blue** (`#1e3a8a`): Primary buttons, active states, headings
- **Wisconsin Gold** (`#f59e0b`): Accent elements, hover states, highlights

#### Content & Text
- **Charcoal Text** (`#1f2937`): Primary text, dark elements
- **Stone Bridge Gray** (`#6b7280`): Secondary text, borders, inactive states

#### Backgrounds & Surfaces
- **Limestone White** (`#fefefe`): Main backgrounds, cards, panels
- **Mist Gray** (`#f3f4f6`): Code blocks, subtle backgrounds, input fields

#### Status & Feedback
- **Ravine Green** (`#059669`): Success messages, positive indicators
- **Bridge Shadow** (`rgba(0, 0, 0, 0.1)`): Subtle depth, card shadows

### Dark Mode Adaptation
```css
@media (prefers-color-scheme: dark) {
  :root {
    --limestone-white: #1a1a1a;
    --charcoal-text: #e5e5e5;
    --mist-gray: #2a2a2a;
    --bridge-shadow: rgba(255, 255, 255, 0.1);
  }
}
```

---

## Typography System

### Font Families
```css
:root {
  /* LVMH Premium Typography Scale */
  --font-luxury-serif: 'Cormorant Garamond', Georgia, serif;
  --font-modern-sans: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
  --font-mono: 'JetBrains Mono', 'Fira Code', monospace;
}
```

### Font Imports
```css
@import url('https://fonts.googleapis.com/css2?family=Cormorant+Garamond:ital,wght@0,300;0,400;0,500;0,600;0,700;1,300;1,400;1,500;1,600;1,700&display=swap');
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&display=swap');
```

### Type Scale
```css
:root {
  /* Gwern-inspired sizing */
  --text-xs: 0.75rem;     /* 12px */
  --text-sm: 0.875rem;    /* 14px */
  --text-base: 1rem;      /* 16px */
  --text-lg: 1.125rem;    /* 18px */
  --text-xl: 1.25rem;     /* 20px */
  --text-2xl: 1.5rem;     /* 24px */
  --text-3xl: 1.875rem;   /* 30px */
  --text-4xl: 2.25rem;    /* 36px */
}
```

### Typography Hierarchy
```css
/* Base styling */
html {
  font-size: 18px; /* Gwern.net standard */
  scroll-behavior: smooth;
}

body {
  font-family: var(--font-luxury-serif);
  line-height: 1.6;
  color: var(--charcoal-text);
  background: var(--limestone-white);
  font-weight: 400;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

/* Headings */
h1, h2, h3, h4, h5, h6 {
  font-family: var(--font-luxury-serif);
  font-weight: 600;
  line-height: 1.2;
  margin-bottom: 1rem;
  color: var(--lake-michigan-blue);
}

h1 {
  font-size: var(--text-4xl);
  font-weight: 700;
  margin-bottom: 2rem;
  border-bottom: 3px solid var(--wisconsin-gold);
  padding-bottom: 0.5rem;
}

h2 {
  font-size: var(--text-3xl);
  margin-top: 2rem;
  margin-bottom: 1.5rem;
}

h3 {
  font-size: var(--text-2xl);
  margin-top: 1.5rem;
}

h4 {
  font-size: var(--text-xl);
  font-weight: 500;
}
```

---

## Component Library

### Layout Components

#### Main Container
```css
.bridge-portal {
  max-width: 80ch;
  margin: 0 auto;
  padding: 2rem;
  position: relative;
}
```

#### Navigation Bridge
```css
.bridge-navigation {
  position: fixed;
  left: 2rem;
  top: 50%;
  transform: translateY(-50%);
  z-index: 100;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(10px);
  padding: 1rem;
  border-radius: 1rem;
  box-shadow: 0 4px 20px var(--bridge-shadow);
}

.bridge-link {
  display: block;
  width: 3rem;
  height: 0.5rem;
  background: var(--stone-bridge-gray);
  margin: 1rem 0;
  border-radius: 0.25rem;
  transition: all 0.3s ease;
  text-decoration: none;
  position: relative;
  cursor: pointer;
}

.bridge-link:hover {
  background: var(--wisconsin-gold);
  transform: scale(1.1);
}

.bridge-link.active {
  background: var(--lake-michigan-blue);
  transform: scale(1.2);
}
```

### Content Components

#### Paragraphs & Text
```css
p {
  margin-bottom: 1.5rem;
  text-align: justify;
  hyphens: auto;
}
```

#### Links
```css
a {
  color: var(--lake-michigan-blue);
  text-decoration: none;
  border-bottom: 1px solid transparent;
  transition: border-color 0.3s;
}

a:hover {
  border-bottom-color: var(--wisconsin-gold);
}
```

#### Blockquotes
```css
blockquote {
  border-left: 4px solid var(--ravine-green);
  padding-left: 2rem;
  margin: 2rem 0;
  font-style: italic;
  color: var(--stone-bridge-gray);
  background: var(--mist-gray);
  padding: 1.5rem 2rem;
  border-radius: 0 0.5rem 0.5rem 0;
}
```

#### Code Elements
```css
pre, code {
  font-family: var(--font-mono);
  font-size: 0.9em;
}

pre {
  background: var(--mist-gray);
  padding: 1.5rem;
  border-radius: 0.5rem;
  overflow-x: auto;
  margin: 2rem 0;
  border: 1px solid var(--stone-bridge-gray);
}

code {
  background: var(--mist-gray);
  padding: 0.2rem 0.4rem;
  border-radius: 0.25rem;
  border: 1px solid rgba(107, 114, 128, 0.3);
}
```

### Interactive Components

#### Tables
```css
table {
  width: 100%;
  border-collapse: collapse;
  margin: 2rem 0;
  font-family: var(--font-modern-sans);
}

th, td {
  border: 1px solid var(--stone-bridge-gray);
  padding: 0.75rem;
  text-align: left;
}

th {
  background: var(--lake-michigan-blue);
  color: white;
  font-weight: 600;
}

tr:nth-child(even) {
  background: var(--mist-gray);
}
```

#### Citation Elements
```css
.citation {
  font-family: var(--font-modern-sans);
  font-size: var(--text-sm);
  color: var(--stone-bridge-gray);
  border: 1px solid var(--stone-bridge-gray);
  padding: 1rem;
  margin: 1rem 0;
  border-radius: 0.5rem;
  background: var(--mist-gray);
}
```

#### Cross-References
```css
.cross-reference {
  background: linear-gradient(135deg, var(--wisconsin-gold), var(--lake-michigan-blue));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s;
}

.cross-reference:hover {
  text-decoration: underline;
}
```

### Verification Components

#### Verification Matrix
```css
.verification-matrix {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin: 2rem 0;
}

.verification-item {
  background: var(--mist-gray);
  padding: 1rem;
  border-radius: 0.5rem;
  border-left: 4px solid var(--ravine-green);
  font-family: var(--font-modern-sans);
}

.verification-item.verified {
  border-left-color: var(--ravine-green);
}

.verification-item.pending {
  border-left-color: var(--wisconsin-gold);
}

.verification-item.failed {
  border-left-color: #dc2626;
}
```

---

## Responsive Design

### Breakpoints
```css
/* Mobile */
@media (max-width: 768px) {
  .bridge-navigation {
    position: static;
    transform: none;
    margin-bottom: 2rem;
    left: auto;
    top: auto;
  }
  
  .bridge-portal {
    padding: 1rem;
  }
  
  html {
    font-size: 16px;
  }
}
```

### Print Styles
```css
@media print {
  .bridge-navigation {
    display: none;
  }
  
  .bridge-portal {
    max-width: none;
    margin: 0;
    padding: 0;
  }
  
  body {
    font-size: 12pt;
    line-height: 1.4;
  }
}
```

---

## Browser-Specific Implementations

### Address Bar Integration
```css
.browser-address-bar {
  background: var(--limestone-white);
  border: 1px solid var(--stone-bridge-gray);
  border-radius: 0.5rem;
  padding: 0.5rem 1rem;
  font-family: var(--font-modern-sans);
  color: var(--charcoal-text);
  transition: border-color 0.3s;
}

.browser-address-bar:focus {
  outline: none;
  border-color: var(--lake-michigan-blue);
  box-shadow: 0 0 0 2px rgba(30, 58, 138, 0.2);
}
```

### Tab Interface
```css
.browser-tab {
  background: var(--mist-gray);
  border: 1px solid var(--stone-bridge-gray);
  border-bottom: none;
  border-radius: 0.5rem 0.5rem 0 0;
  padding: 0.5rem 1rem;
  font-family: var(--font-modern-sans);
  font-size: var(--text-sm);
  cursor: pointer;
  transition: all 0.3s;
}

.browser-tab.active {
  background: var(--limestone-white);
  border-color: var(--lake-michigan-blue);
  color: var(--lake-michigan-blue);
}

.browser-tab:hover:not(.active) {
  background: rgba(245, 158, 11, 0.1);
  border-color: var(--wisconsin-gold);
}
```

### Navigation Buttons
```css
.browser-nav-button {
  background: var(--limestone-white);
  border: 1px solid var(--stone-bridge-gray);
  border-radius: 0.25rem;
  padding: 0.5rem;
  cursor: pointer;
  transition: all 0.3s;
  font-size: var(--text-sm);
}

.browser-nav-button:hover {
  background: var(--wisconsin-gold);
  color: white;
  border-color: var(--wisconsin-gold);
}

.browser-nav-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background: var(--mist-gray);
}
```

---

## Animation & Transitions

### Standard Transitions
```css
:root {
  --transition-fast: 0.15s ease;
  --transition-base: 0.3s ease;
  --transition-slow: 0.5s ease;
}

/* Applied to interactive elements */
.interactive {
  transition: all var(--transition-base);
}
```

### Loading States
```css
@keyframes aiPulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

.loading-indicator {
  animation: aiPulse 2s infinite;
}
```

---

## Accessibility

### Focus States
```css
*:focus {
  outline: 2px solid var(--lake-michigan-blue);
  outline-offset: 2px;
}

.focus-visible {
  outline: 2px solid var(--wisconsin-gold);
  outline-offset: 2px;
}
```

### High Contrast Support
```css
@media (prefers-contrast: high) {
  :root {
    --bridge-shadow: rgba(0, 0, 0, 0.3);
    --stone-bridge-gray: #4a4a4a;
  }
}
```

---

## Implementation Guide

### CSS Variables Setup
Add this to your main CSS file:
```css
@import url('seven-bridge-design-system.css');
```

### HTML Structure Example
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Browser UI - Seven Bridge Design</title>
    <link rel="stylesheet" href="seven-bridge-design-system.css">
</head>
<body>
    <div class="bridge-portal">
        <!-- Browser interface content -->
    </div>
</body>
</html>
```

### JavaScript Integration
```javascript
// Apply theme dynamically
function applySevenBridgeTheme() {
    document.documentElement.classList.add('seven-bridge-theme');
}

// Toggle dark mode
function toggleDarkMode() {
    document.documentElement.classList.toggle('dark-theme');
}
```

---

## Version History

### V1.0 (Current)
- Complete color palette extraction from Seven Bridge Portal
- Typography system with LVMH premium fonts
- Component library with navigation, content, and interactive elements
- Responsive design patterns
- Browser-specific implementations
- Accessibility guidelines

---

## Next Steps

1. **Integration**: Apply this design system to existing browser UI components
2. **Testing**: Validate accessibility and responsive behavior
3. **Refinement**: Gather feedback and iterate on components
4. **Documentation**: Create component usage examples and guidelines
5. **Automation**: Set up design tokens and build processes

---

*Seven Bridge Portal Design System V1 - Created July 1, 2025*
*For MoodBridge Rust Browser UI Implementation*
