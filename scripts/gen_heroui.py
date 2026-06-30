import json

components = []

# 1. HeroUI Button Primary
components.append({
    'name': 'HeroUI Button Primary',
    'description': 'A primary action button with hover/active/focus states, built on React Aria patterns with Tailwind CSS.',
    'category': 'other',
    'framework': 'react',
    'code': '''import { useState } from 'react';

export function Button({ children, variant = 'primary', size = 'md', disabled = false, fullWidth = false, onPress, ...props }) {
  const [isHovered, setIsHovered] = useState(false);
  const [isPressed, setIsPressed] = useState(false);

  const base = 'inline-flex items-center justify-center font-medium rounded-xl transition-all duration-150 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40 disabled:opacity-50 disabled:pointer-events-none';

  const variants = {
    primary: 'bg-primary text-white hover:bg-primary/90 shadow-sm active:scale-[0.97]',
    secondary: 'bg-secondary text-secondary-foreground hover:bg-secondary/80 shadow-sm active:scale-[0.97]',
    outline: 'border-2 border-border bg-transparent hover:bg-muted active:scale-[0.97]',
    ghost: 'bg-transparent hover:bg-muted active:scale-[0.97]',
    danger: 'bg-danger text-white hover:bg-danger/90 shadow-sm active:scale-[0.97]',
  };

  const sizes = {
    sm: 'h-8 px-3 text-xs gap-1.5',
    md: 'h-10 px-4 text-sm gap-2',
    lg: 'h-12 px-6 text-base gap-2.5',
  };

  return (
    <button
      className={base + ' ' + variants[variant] + ' ' + sizes[size] + (fullWidth ? ' w-full' : '')}
      disabled={disabled}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => { setIsHovered(false); setIsPressed(false); }}
      onMouseDown={() => setIsPressed(true)}
      onMouseUp={() => setIsPressed(false)}
      onClick={onPress}
      data-hovered={isHovered || undefined}
      data-pressed={isPressed || undefined}
      {...props}
    >
      {children}
    </button>
  );
}''',
    'tags': ['button', 'react', 'action', 'primary', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 2. HeroUI Card
components.append({
    'name': 'HeroUI Card',
    'description': 'A flexible container for grouping related content with header, content, and footer sections. Supports variants and custom styling.',
    'category': 'card',
    'framework': 'react',
    'code': '''export function Card({ children, variant = 'default', className = '' }) {
  const variants = {
    default: 'bg-surface text-foreground shadow-sm',
    transparent: 'bg-transparent',
    secondary: 'bg-surface-secondary shadow-sm',
    tertiary: 'bg-surface-tertiary shadow-md',
  };

  return (
    <div className={(variants[variant] || variants.default) + ' rounded-2xl border border-border p-6 ' + className}>
      {children}
    </div>
  );
}

Card.Header = function CardHeader({ children, className = '' }) {
  return <div className={'flex flex-col gap-1 pb-4 ' + className}>{children}</div>;
};

Card.Title = function CardTitle({ children, className = '' }) {
  return <h3 className={'text-lg font-semibold text-foreground ' + className}>{children}</h3>;
};

Card.Description = function CardDescription({ children, className = '' }) {
  return <p className={'text-sm text-muted-foreground ' + className}>{children}</p>;
};

Card.Content = function CardContent({ children, className = '' }) {
  return <div className={'py-2 ' + className}>{children}</div>;
};

Card.Footer = function CardFooter({ children, className = '' }) {
  return <div className={'flex items-center gap-3 pt-4 border-t border-border mt-4 ' + className}>{children}</div>;
};''',
    'tags': ['card', 'react', 'container', 'layout', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 3. HeroUI Modal Dialog
components.append({
    'name': 'HeroUI Modal Dialog',
    'description': 'A dialog overlay for focused user interactions with backdrop, sizing variants, and keyboard dismiss support.',
    'category': 'modal',
    'framework': 'react',
    'code': '''import { useState, useEffect, useCallback } from 'react';
import { createPortal } from 'react-dom';

export function Modal({ isOpen, onOpenChange, children }) {
  if (!isOpen) return null;

  return createPortal(
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <Modal.Backdrop isOpen={isOpen} onOpenChange={onOpenChange} />
      <Modal.Container>
        <Modal.Dialog>{children}</Modal.Dialog>
      </Modal.Container>
    </div>,
    document.body
  );
}

Modal.Backdrop = function Backdrop({ isOpen, onOpenChange }) {
  return (
    <div
      className="absolute inset-0 bg-black/50"
      onClick={() => onOpenChange?.(false)}
    />
  );
};

Modal.Container = function Container({ size = 'md', children }) {
  const sizes = { xs: 'max-w-sm', sm: 'max-w-md', md: 'max-w-lg', lg: 'max-w-2xl', xl: 'max-w-4xl' };
  return <div className={'relative w-full ' + (sizes[size] || sizes.md) + ' mx-4'}>{children}</div>;
};

Modal.Dialog = function Dialog({ children }) {
  return (
    <div className="relative bg-background rounded-2xl shadow-2xl border border-border p-6 max-h-[85vh] overflow-y-auto" role="dialog" aria-modal="true">
      {children}
    </div>
  );
};

Modal.CloseTrigger = function CloseTrigger({ onClose }) {
  return (
    <button
      onClick={onClose}
      className="absolute top-4 right-4 rounded-full p-1.5 text-muted-foreground hover:bg-muted hover:text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40"
      aria-label="Close"
    >
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M18 6L6 18"/><path d="M6 6l12 12"/></svg>
    </button>
  );
};

Modal.Header = function Header({ children }) {
  return <div className="flex flex-col gap-1 mb-4">{children}</div>;
};

Modal.Heading = function Heading({ children }) {
  return <h2 className="text-xl font-semibold text-foreground">{children}</h2>;
};

Modal.Body = function Body({ children }) {
  return <div className="py-2">{children}</div>;
};

Modal.Footer = function Footer({ children }) {
  return <div className="flex items-center justify-end gap-3 pt-4 mt-4 border-t border-border">{children}</div>;
};''',
    'tags': ['modal', 'dialog', 'react', 'overlay', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'react-dom', 'tailwindcss']
})

# 4. HeroUI Input Field
components.append({
    'name': 'HeroUI Input Field',
    'description': 'A primitive single-line text input with hover, focus, invalid, and disabled states. Supports all standard HTML input attributes.',
    'category': 'form',
    'framework': 'react',
    'code': '''import { useState, forwardRef } from 'react';

export const Input = forwardRef(function Input({
  className = '', variant = 'primary', type = 'text', label, error, fullWidth = false, ...props
}, ref) {
  const [isFocused, setIsFocused] = useState(false);

  const variants = {
    primary: 'bg-background border-border shadow-sm focus-visible:border-primary focus-visible:ring-2 focus-visible:ring-primary/20',
    secondary: 'bg-surface-secondary border-border/70 focus-visible:border-primary focus-visible:ring-2 focus-visible:ring-primary/20',
  };

  return (
    <div className={'flex flex-col gap-1.5 ' + (fullWidth ? 'w-full' : '')}>
      {label && <label className="text-sm font-medium text-foreground">{label}</label>}
      <input
        ref={ref}
        type={type}
        className={'rounded-xl border px-4 py-2.5 text-sm text-foreground placeholder:text-muted-foreground/60 transition-colors outline-none hover:border-border/80 hover:bg-surface-secondary disabled:opacity-50 disabled:cursor-not-allowed disabled:bg-muted aria-invalid:border-danger aria-invalid:ring-2 aria-invalid:ring-danger/20 ' + (variants[variant] || variants.primary) + ' ' + (fullWidth ? 'w-full' : '') + ' ' + className}
        onFocus={(e) => { setIsFocused(true); props.onFocus?.(e); }}
        onBlur={(e) => { setIsFocused(false); props.onBlur?.(e); }}
        aria-invalid={error ? 'true' : undefined}
        {...props}
      />
      {error && <span className="text-xs text-danger">{error}</span>}
    </div>
  );
});''',
    'tags': ['input', 'form', 'react', 'text-field', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 5. HeroUI Accordion
components.append({
    'name': 'HeroUI Accordion',
    'description': 'A collapsible content panel for organizing information in a compact space. Supports multiple expanded items.',
    'category': 'section',
    'framework': 'react',
    'code': '''import { useState, createContext, useContext } from 'react';

const AccordionCtx = createContext(null);

export function Accordion({ children, multiple = false, className = '' }) {
  const [expandedItems, setExpandedItems] = useState(new Set());

  const toggle = (id) => {
    setExpandedItems(prev => {
      const next = new Set(multiple ? prev : []);
      if (prev.has(id)) next.delete(id);
      else if (multiple) next.add(id);
      else next.add(id);
      return next;
    });
  };

  return (
    <AccordionCtx.Provider value={{ expandedItems, toggle }}>
      <div className={'divide-y divide-border rounded-xl border border-border bg-background ' + className}>
        {children}
      </div>
    </AccordionCtx.Provider>
  );
}

Accordion.Item = function AccordionItem({ id, title, children, disabled = false }) {
  const { expandedItems, toggle } = useContext(AccordionCtx);
  const isExpanded = expandedItems.has(id);

  return (
    <div className={disabled ? 'opacity-50 pointer-events-none' : ''}>
      <button
        className="flex w-full items-center justify-between px-5 py-4 text-left text-sm font-medium text-foreground hover:bg-muted/50 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-inset focus-visible:ring-primary/40"
        onClick={() => !disabled && toggle(id)}
        disabled={disabled}
        aria-expanded={isExpanded}
      >
        {title}
        <svg className={'size-4 text-muted-foreground transition-transform duration-200 ' + (isExpanded ? 'rotate-180' : '')} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="m6 9 6 6 6-6"/></svg>
      </button>
      <div className={'overflow-hidden transition-all duration-200 ' + (isExpanded ? 'max-h-96' : 'max-h-0')}>
        <div className="px-5 pb-4 text-sm text-muted-foreground">{children}</div>
      </div>
    </div>
  );
};''',
    'tags': ['accordion', 'collapsible', 'react', 'section', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 6. HeroUI Alert
components.append({
    'name': 'HeroUI Alert',
    'description': 'Display important messages and notifications with status indicators for success, warning, error, and info variants.',
    'category': 'notification',
    'framework': 'react',
    'code': '''export function Alert({ children, status = 'info', className = '' }) {
  const styles = {
    info: 'bg-info/10 border-info/30 text-info',
    success: 'bg-success/10 border-success/30 text-success',
    warning: 'bg-warning/10 border-warning/30 text-warning',
    danger: 'bg-danger/10 border-danger/30 text-danger',
    accent: 'bg-accent/10 border-accent/30 text-accent',
  };

  const icons = {
    info: 'M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm1-13h-2v6h2V7zm0 8h-2v2h2v-2z',
    success: 'M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z',
    warning: 'M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z',
    danger: 'M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z',
  };

  return (
    <div className={'flex items-start gap-3 rounded-2xl border p-4 ' + (styles[status] || styles.info) + ' ' + className} role="alert">
      <svg className="size-5 shrink-0 mt-0.5" viewBox="0 0 24 24" fill="currentColor">
        <path d={(icons)[status] || icons.info} />
      </svg>
      <div className="flex flex-col gap-0.5 flex-1">{children}</div>
    </div>
  );
}

Alert.Title = function Title({ children }) {
  return <p className="font-semibold text-sm">{children}</p>;
};

Alert.Description = function Description({ children }) {
  return <p className="text-sm opacity-80">{children}</p>;
};''',
    'tags': ['alert', 'notification', 'react', 'feedback', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 7. HeroUI Avatar
components.append({
    'name': 'HeroUI Avatar',
    'description': 'Display user profile images with customizable fallback content (initials or icon). Supports sizes, colors, and avatar groups.',
    'category': 'other',
    'framework': 'react',
    'code': '''import { useState } from 'react';

export function Avatar({ src, alt = '', fallback, size = 'md', color = 'default', className = '' }) {
  const [imgError, setImgError] = useState(false);
  const [loaded, setLoaded] = useState(false);

  const sizes = { sm: 'size-8 text-xs', md: 'size-10 text-sm', lg: 'size-12 text-base', xl: 'size-16 text-lg' };
  const colors = {
    default: 'bg-muted text-foreground',
    primary: 'bg-primary/20 text-primary',
    success: 'bg-success/20 text-success',
    warning: 'bg-warning/20 text-warning',
    danger: 'bg-danger/20 text-danger',
  };

  const initials = fallback || (alt ? alt.split(' ').map(w => w[0]).join('').toUpperCase().slice(0, 2) : '?');

  return (
    <div className={'relative inline-flex items-center justify-center rounded-full overflow-hidden ' + (sizes[size] || sizes.md) + ' ' + (colors[color] || colors.default) + ' ' + className}>
      {src && !imgError && (
        <img
          src={src}
          alt={alt}
          className={'absolute inset-0 size-full object-cover ' + (loaded ? 'opacity-100' : 'opacity-0')}
          onLoad={() => setLoaded(true)}
          onError={() => setImgError(true)}
        />
      )}
      {(!src || imgError) && (
        <span className="font-medium">{initials}</span>
      )}
    </div>
  );
}

export function AvatarGroup({ children, max = 4, size = 'md' }) {
  const childrenArray = Array.isArray(children) ? children : [children];
  const visible = childrenArray.slice(0, max);
  const overflow = childrenArray.length - max;

  return (
    <div className="flex -space-x-2">
      {visible.map((child, i) => (
        <div key={i} className="ring-2 ring-background rounded-full">{child}</div>
      ))}
      {overflow > 0 && (
        <div className="inline-flex items-center justify-center rounded-full ring-2 ring-background bg-muted text-muted-foreground text-xs font-medium size-10">
          +{overflow}
        </div>
      )}
    </div>
  );
}''',
    'tags': ['avatar', 'react', 'user', 'profile', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 8. HeroUI Badge
components.append({
    'name': 'HeroUI Badge',
    'description': 'Displays a small indicator positioned relative to another element for notification counts, status dots, and labels.',
    'category': 'other',
    'framework': 'react',
    'code': '''export function Badge({ children, color = 'primary', size = 'md', variant = 'solid', placement = 'top-right', ...props }) {
  const colors = {
    primary: 'bg-primary text-white',
    success: 'bg-success text-white',
    warning: 'bg-warning text-warning-foreground',
    danger: 'bg-danger text-white',
    default: 'bg-muted text-muted-foreground',
  };

  const sizes = { sm: 'min-w-[18px] h-[18px] px-1 text-[10px]', md: 'min-w-[22px] h-[22px] px-1.5 text-xs', lg: 'min-w-[26px] h-[26px] px-2 text-sm' };

  const placements = {
    'top-right': '-top-1 -right-1',
    'top-left': '-top-1 -left-1',
    'bottom-right': '-bottom-1 -right-1',
    'bottom-left': '-bottom-1 -left-1',
  };

  if (!children) {
    return (
      <span className={'absolute size-2.5 rounded-full ring-2 ring-background ' + (colors[color] || colors.primary) + ' ' + (placements[placement] || placements['top-right'])} />
    );
  }

  return (
    <span className={'absolute inline-flex items-center justify-center rounded-full font-medium ring-2 ring-background ' + (colors[color] || colors.primary) + ' ' + (sizes[size] || sizes.md) + ' ' + (placements[placement] || placements['top-right'])}>
      {typeof children === 'number' && children > 99 ? '99+' : children}
    </span>
  );
}

Badge.Anchor = function Anchor({ children }) {
  return <div className="relative inline-flex">{children}</div>;
};

Badge.Label = function Label({ children }) {
  return <span>{children}</span>;
};''',
    'tags': ['badge', 'react', 'notification', 'indicator', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 9. HeroUI Tabs
components.append({
    'name': 'HeroUI Tabs',
    'description': 'Tab-based navigation organizing content into multiple sections with animated indicator and keyboard navigation.',
    'category': 'section',
    'framework': 'react',
    'code': '''import { useState } from 'react';

export function Tabs({ defaultValue, value, onValueChange, children, className = '', orientation = 'horizontal' }) {
  const [internalValue, setInternalValue] = useState(defaultValue || '');
  const selected = value !== undefined ? value : internalValue;

  const handleChange = (val) => {
    if (onValueChange) onValueChange(val);
    else setInternalValue(val);
  };

  return (
    <div className={'flex flex-col gap-4 ' + (orientation === 'vertical' ? 'sm:flex-row' : '') + ' ' + className}>
      {typeof children === 'function' ? children({ selected, onSelect: handleChange }) : children}
    </div>
  );
}

Tabs.List = function TabList({ children, className = '' }) {
  return (
    <div className={'inline-flex h-10 items-center rounded-xl bg-muted p-1 text-muted-foreground ' + className}>
      {children}
    </div>
  );
};

Tabs.Tab = function Tab({ value, children, selected, onSelect, disabled = false, className = '' }) {
  const isSelected = selected === value;
  return (
    <button
      className={'inline-flex items-center justify-center whitespace-nowrap rounded-lg px-3 py-1.5 text-sm font-medium transition-all focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40 disabled:pointer-events-none disabled:opacity-50 ' + (isSelected ? 'bg-background text-foreground shadow-sm' : 'hover:text-foreground') + ' ' + className}
      onClick={() => onSelect?.(value)}
      disabled={disabled}
      role="tab"
      aria-selected={isSelected}
    >
      {children}
    </button>
  );
};

Tabs.Panel = function Panel({ value, selected, children, className = '' }) {
  if (selected !== value) return null;
  return (
    <div className={className} role="tabpanel">
      {children}
    </div>
  );
};''',
    'tags': ['tabs', 'react', 'navigation', 'section', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 10. HeroUI Select Dropdown
components.append({
    'name': 'HeroUI Select Dropdown',
    'description': 'A select component displaying a collapsible list of options with trigger, popover, and list box sections.',
    'category': 'form',
    'framework': 'react',
    'code': '''import { useState, useRef, useEffect } from 'react';

export function Select({ children, placeholder = 'Select an option', value, onChange, label, error, fullWidth = false, variant = 'primary' }) {
  const [isOpen, setIsOpen] = useState(false);
  const [internalValue, setInternalValue] = useState(value || '');
  const ref = useRef(null);

  const selectedValue = value !== undefined ? value : internalValue;

  const handleSelect = (val) => {
    if (onChange) onChange(val);
    else setInternalValue(val);
    setIsOpen(false);
  };

  useEffect(() => {
    const handleClickOutside = (e) => {
      if (ref.current && !ref.current.contains(e.target)) setIsOpen(false);
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const variants = {
    primary: 'bg-background border-border shadow-sm',
    secondary: 'bg-surface-secondary border-border/70',
  };

  return (
    <div className={'relative flex flex-col gap-1.5 ' + (fullWidth ? 'w-full' : '')} ref={ref}>
      {label && <label className="text-sm font-medium text-foreground">{label}</label>}
      <button
        className={'flex items-center justify-between rounded-xl border px-4 py-2.5 text-sm text-left transition-colors outline-none hover:border-border/80 focus-visible:border-primary focus-visible:ring-2 focus-visible:ring-primary/20 ' + (variants[variant] || variants.primary) + ' ' + (fullWidth ? 'w-full' : 'min-w-[200px]') + ' ' + (error ? 'border-danger ring-2 ring-danger/20' : '')}
        onClick={() => setIsOpen(!isOpen)}
        aria-expanded={isOpen}
      >
        <span className={selectedValue ? 'text-foreground' : 'text-muted-foreground/60'}>
          {selectedValue || placeholder}
        </span>
        <svg className={'size-4 text-muted-foreground transition-transform ' + (isOpen ? 'rotate-180' : '')} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="m6 9 6 6 6-6"/></svg>
      </button>
      {error && <span className="text-xs text-danger">{error}</span>}
      {isOpen && (
        <div className="absolute top-full mt-1 z-50 w-full min-w-[200px] rounded-xl border border-border bg-background shadow-lg p-1" role="listbox">
          {typeof children === 'function' ? children({ selectedValue, onSelect: handleSelect }) : children}
        </div>
      )}
    </div>
  );
}

Select.Option = function Option({ value, children, selectedValue, onSelect, disabled = false }) {
  const isSelected = selectedValue === value;
  return (
    <button
      className={'flex w-full items-center rounded-lg px-3 py-2 text-sm text-left transition-colors ' + (isSelected ? 'bg-primary/10 text-primary font-medium' : 'text-foreground hover:bg-muted') + ' ' + (disabled ? 'opacity-50 cursor-not-allowed' : '')}
      onClick={() => !disabled && onSelect?.(value)}
      role="option"
      aria-selected={isSelected}
    >
      <span className="flex-1">{children}</span>
      {isSelected && (
        <svg className="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M20 6L9 17l-5-5"/></svg>
      )}
    </button>
  );
};''',
    'tags': ['select', 'dropdown', 'form', 'react', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 11. HeroUI Tooltip
components.append({
    'name': 'HeroUI Tooltip',
    'description': 'A tooltip that displays informative text when users hover over an element. Supports placement variants.',
    'category': 'other',
    'framework': 'react',
    'code': '''import { useState, useRef } from 'react';

export function Tooltip({ children, content, placement = 'top', delay = 200 }) {
  const [isVisible, setIsVisible] = useState(false);
  const timeoutRef = useRef(null);

  const placements = {
    top: 'bottom-full left-1/2 -translate-x-1/2 mb-2',
    bottom: 'top-full left-1/2 -translate-x-1/2 mt-2',
    left: 'right-full top-1/2 -translate-y-1/2 mr-2',
    right: 'left-full top-1/2 -translate-y-1/2 ml-2',
  };

  const arrows = {
    top: 'top-full left-1/2 -translate-x-1/2 border-l-transparent border-r-transparent border-b-transparent border-t-muted',
    bottom: 'bottom-full left-1/2 -translate-x-1/2 border-l-transparent border-r-transparent border-t-transparent border-b-muted',
    left: 'left-full top-1/2 -translate-y-1/2 border-t-transparent border-b-transparent border-r-transparent border-l-muted',
    right: 'right-full top-1/2 -translate-y-1/2 border-t-transparent border-b-transparent border-l-transparent border-r-muted',
  };

  const show = () => {
    clearTimeout(timeoutRef.current);
    timeoutRef.current = setTimeout(() => setIsVisible(true), delay);
  };

  const hide = () => {
    clearTimeout(timeoutRef.current);
    setIsVisible(false);
  };

  return (
    <div className="relative inline-flex" onMouseEnter={show} onMouseLeave={hide} onFocus={show} onBlur={hide}>
      {children}
      {isVisible && content && (
        <div className={'absolute z-50 ' + (placements[placement] || placements.top)} role="tooltip">
          <div className="rounded-lg bg-muted text-muted-foreground text-xs px-3 py-1.5 shadow-md whitespace-nowrap">
            {content}
          </div>
          <div className={'absolute size-0 border-4 ' + (arrows[placement] || arrows.top)} />
        </div>
      )}
    </div>
  );
}''',
    'tags': ['tooltip', 'react', 'popover', 'hover', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 12. HeroUI Spinner
components.append({
    'name': 'HeroUI Spinner',
    'description': 'A loading spinner indicating an indeterminate loading state with customizable size and color.',
    'category': 'loading',
    'framework': 'react',
    'code': '''export function Spinner({ size = 'md', color = 'primary', label }) {
  const sizes = { sm: 'size-4 border-2', md: 'size-6 border-2', lg: 'size-8 border-3', xl: 'size-12 border-3' };
  const colors = {
    primary: 'border-primary/30 border-t-primary',
    foreground: 'border-foreground/30 border-t-foreground',
    muted: 'border-muted-foreground/30 border-t-muted-foreground',
    white: 'border-white/30 border-t-white',
  };

  return (
    <div className="inline-flex flex-col items-center justify-center gap-2">
      <div className={'animate-spin rounded-full ' + (sizes[size] || sizes.md) + ' ' + (colors[color] || colors.primary)} />
      {label && <span className="text-xs text-muted-foreground">{label}</span>}
    </div>
  );
}''',
    'tags': ['spinner', 'loading', 'react', 'progress', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 13. HeroUI Progress Bar
components.append({
    'name': 'HeroUI Progress Bar',
    'description': 'A progress bar showing determinate or indeterminate progress with customizable value, label, and color.',
    'category': 'loading',
    'framework': 'react',
    'code': '''export function ProgressBar({ value = 0, max = 100, color = 'primary', size = 'md', label, showValue = false, className = '' }) {
  const percentage = Math.min(Math.max((value / max) * 100, 0), 100);
  const sizes = { sm: 'h-1.5', md: 'h-2.5', lg: 'h-4' };
  const colors = {
    primary: 'bg-primary',
    success: 'bg-success',
    warning: 'bg-warning',
    danger: 'bg-danger',
    accent: 'bg-accent',
  };

  return (
    <div className={'flex flex-col gap-1.5 ' + className}>
      {(label || showValue) && (
        <div className="flex items-center justify-between">
          {label && <span className="text-sm font-medium text-foreground">{label}</span>}
          {showValue && <span className="text-xs text-muted-foreground">{Math.round(percentage)}%</span>}
        </div>
      )}
      <div className={'w-full overflow-hidden rounded-full bg-muted ' + (sizes[size] || sizes.md)} role="progressbar" aria-valuenow={value} aria-valuemin={0} aria-valuemax={max}>
        <div
          className={'h-full rounded-full transition-all duration-500 ease-out ' + (colors[color] || colors.primary)}
          style={{ width: percentage + '%' }}
        />
      </div>
    </div>
  );
}

export function ProgressCircle({ value = 0, size = 'md', color = 'primary', showValue = false }) {
  const percentage = Math.min(Math.max((value / 100) * 100, 0), 100);
  const sizes = { sm: 32, md: 48, lg: 64 };
  const strokeWidths = { sm: 3, md: 4, lg: 5 };
  const dim = sizes[size] || sizes.md;
  const sw = strokeWidths[size] || strokeWidths.md;
  const radius = (dim - sw) / 2;
  const circumference = 2 * Math.PI * radius;
  const offset = circumference - (percentage / 100) * circumference;
  const colors = { primary: 'stroke-primary', success: 'stroke-success', warning: 'stroke-warning', danger: 'stroke-danger' };

  return (
    <div className="relative inline-flex items-center justify-center">
      <svg width={dim} height={dim} className="-rotate-90">
        <circle cx={dim/2} cy={dim/2} r={radius} fill="none" stroke="currentColor" strokeWidth={sw} className="text-muted" />
        <circle cx={dim/2} cy={dim/2} r={radius} fill="none" strokeWidth={sw} strokeDasharray={circumference} strokeDashoffset={offset} strokeLinecap="round" className={'transition-all duration-500 ' + (colors[color] || colors.primary)} />
      </svg>
      {showValue && <span className="absolute text-xs font-medium text-foreground">{Math.round(percentage)}%</span>}
    </div>
  );
}''',
    'tags': ['progress', 'loading', 'react', 'bar', 'circle', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 14. HeroUI Skeleton
components.append({
    'name': 'HeroUI Skeleton',
    'description': 'A skeleton placeholder for loading states that mimics the shape of content with a shimmer animation.',
    'category': 'loading',
    'framework': 'react',
    'code': '''export function Skeleton({ children, isLoading = true, className = '', variant = 'default' }) {
  if (!isLoading) return children;

  const variants = {
    default: 'rounded-lg',
    circle: 'rounded-full',
    text: 'rounded h-4',
    title: 'rounded h-6 w-2/3',
  };

  const base = 'animate-pulse bg-muted/60';

  if (!children) {
    return <div className={base + ' ' + (variants[variant] || variants.default) + ' ' + className} />;
  }

  return (
    <div className={base + ' ' + (variants[variant] || variants.default) + ' ' + className}>
      <div className="invisible">{children}</div>
    </div>
  );
}

export function SkeletonCard() {
  return (
    <div className="rounded-2xl border border-border p-6 space-y-4">
      <Skeleton variant="title" />
      <Skeleton className="h-32 w-full" />
      <div className="space-y-2">
        <Skeleton variant="text" className="w-full" />
        <Skeleton variant="text" className="w-4/5" />
        <Skeleton variant="text" className="w-3/5" />
      </div>
      <div className="flex gap-3">
        <Skeleton className="h-10 w-24 rounded-xl" />
        <Skeleton className="h-10 w-24 rounded-xl" />
      </div>
    </div>
  );
}''',
    'tags': ['skeleton', 'loading', 'react', 'placeholder', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 15. HeroUI Switch/Toggle
components.append({
    'name': 'HeroUI Switch Toggle',
    'description': 'A toggle switch component for binary settings with labels and disabled state support.',
    'category': 'form',
    'framework': 'react',
    'code': '''import { useState } from 'react';

export function Switch({ checked, onCheckedChange, label, disabled = false, size = 'md' }) {
  const [internalChecked, setInternalChecked] = useState(false);
  const isChecked = checked !== undefined ? checked : internalChecked;

  const handleToggle = () => {
    if (disabled) return;
    const newValue = !isChecked;
    if (onCheckedChange) onCheckedChange(newValue);
    else setInternalChecked(newValue);
  };

  const sizes = {
    sm: { track: 'h-5 w-9', thumb: 'size-4' },
    md: { track: 'h-6 w-11', thumb: 'size-5' },
    lg: { track: 'h-7 w-14', thumb: 'size-6' },
  };

  const s = sizes[size] || sizes.md;

  return (
    <label className={'inline-flex items-center gap-3 ' + (disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer')}>
      <button
        className={'relative inline-flex shrink-0 rounded-full border-2 border-transparent transition-colors duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40 focus-visible:ring-offset-2 ' + (isChecked ? 'bg-primary' : 'bg-muted-foreground/30') + ' ' + s.track}
        onClick={handleToggle}
        disabled={disabled}
        role="switch"
        aria-checked={isChecked}
        data-checked={isChecked || undefined}
      >
        <span
          className={'pointer-events-none block rounded-full bg-white shadow-sm ring-0 transition-transform duration-200 ' + s.thumb + ' ' + (isChecked ? 'translate-x-full' : 'translate-x-0')}
          data-checked={isChecked || undefined}
        />
      </button>
      {label && <span className="text-sm font-medium text-foreground">{label}</span>}
    </label>
  );
}''',
    'tags': ['switch', 'toggle', 'form', 'react', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 16. HeroUI Checkbox
components.append({
    'name': 'HeroUI Checkbox',
    'description': 'A checkbox input for selecting multiple items from a list. Supports indeterminate state, labels, and disabled.',
    'category': 'form',
    'framework': 'react',
    'code': '''import { useState } from 'react';

export function Checkbox({ checked, onCheckedChange, label, disabled = false, indeterminate = false }) {
  const [internalChecked, setInternalChecked] = useState(false);
  const isChecked = checked !== undefined ? checked : internalChecked;

  const handleToggle = () => {
    if (disabled) return;
    const newValue = !isChecked;
    if (onCheckedChange) onCheckedChange(newValue);
    else setInternalChecked(newValue);
  };

  return (
    <label className={'inline-flex items-center gap-2.5 ' + (disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer')}>
      <button
        className={'flex size-5 items-center justify-center rounded-md border-2 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40 ' + (isChecked || indeterminate ? 'bg-primary border-primary text-white' : 'border-border bg-background hover:border-primary/50')}
        onClick={handleToggle}
        disabled={disabled}
        role="checkbox"
        aria-checked={indeterminate ? 'mixed' : isChecked}
        data-checked={isChecked || undefined}
      >
        {indeterminate ? (
          <svg className="size-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3"><path d="M5 12h14"/></svg>
        ) : isChecked ? (
          <svg className="size-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3"><path d="M20 6L9 17l-5-5"/></svg>
        ) : null}
      </button>
      {label && <span className="text-sm text-foreground">{label}</span>}
    </label>
  );
}''',
    'tags': ['checkbox', 'form', 'react', 'selection', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 17. HeroUI Breadcrumbs
components.append({
    'name': 'HeroUI Breadcrumbs',
    'description': 'Navigation breadcrumbs showing the current page location within a hierarchy with separators.',
    'category': 'section',
    'framework': 'react',
    'code': '''export function Breadcrumbs({ children, separator = '/' }) {
  const items = Array.isArray(children) ? children : [children];
  return (
    <nav aria-label="Breadcrumb" className="flex items-center">
      <ol className="flex items-center gap-1.5 text-sm text-muted-foreground">
        {items.map((child, i) => (
          <li key={i} className="flex items-center gap-1.5">
            {i > 0 && <span className="text-muted-foreground/50">{separator}</span>}
            {child}
          </li>
        ))}
      </ol>
    </nav>
  );
}

Breadcrumbs.Item = function BreadcrumbItem({ href, children, isCurrent = false }) {
  if (isCurrent) {
    return <span className="text-foreground font-medium" aria-current="page">{children}</span>;
  }
  return (
    <a href={href} className="hover:text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40 rounded">
      {children}
    </a>
  );
};''',
    'tags': ['breadcrumbs', 'navigation', 'react', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 18. HeroUI Pagination
components.append({
    'name': 'HeroUI Pagination',
    'description': 'A pagination component for navigating through multiple pages with ellipsis truncation.',
    'category': 'other',
    'framework': 'react',
    'code': '''export function Pagination({ total, page, onChange, siblings = 1, boundaries = 1 }) {
  const range = (start, end) => Array.from({ length: end - start + 1 }, (_, i) => start + i);

  const generatePages = () => {
    const totalPages = Math.max(total, 1);
    const pages = new Set();

    range(1, boundaries).forEach(p => pages.add(p));
    range(totalPages - boundaries + 1, totalPages).forEach(p => pages.add(p));

    range(page - siblings, page + siblings).forEach(p => {
      if (p >= 1 && p <= totalPages) pages.add(p);
    });

    return Array.from(pages).sort((a, b) => a - b);
  };

  const pages = generatePages();
  const items = [];

  for (let i = 0; i < pages.length; i++) {
    if (i > 0 && pages[i] - pages[i-1] > 1) {
      items.push('ellipsis');
    }
    items.push(pages[i]);
  }

  const buttonBase = 'inline-flex items-center justify-center rounded-xl text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40 disabled:opacity-50 disabled:pointer-events-none';

  return (
    <nav aria-label="Pagination" className="flex items-center gap-1">
      <button className={buttonBase + ' size-9 hover:bg-muted'} disabled={page <= 1} onClick={() => onChange?.(page - 1)} aria-label="Previous page">
        <svg className="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="m15 18-6-6 6-6"/></svg>
      </button>

      {items.map((item, i) =>
        item === 'ellipsis' ? (
          <span key={'e-' + i} className="flex size-9 items-center justify-center text-muted-foreground">...</span>
        ) : (
          <button
            key={item}
            className={buttonBase + ' size-9 ' + (item === page ? 'bg-primary text-white shadow-sm' : 'hover:bg-muted text-foreground')}
            onClick={() => onChange?.(item)}
            aria-current={item === page ? 'page' : undefined}
            aria-label={'Page ' + item}
          >
            {item}
          </button>
        )
      )}

      <button className={buttonBase + ' size-9 hover:bg-muted'} disabled={page >= total} onClick={() => onChange?.(page + 1)} aria-label="Next page">
        <svg className="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="m9 18 6-6-6-6"/></svg>
      </button>
    </nav>
  );
}''',
    'tags': ['pagination', 'navigation', 'react', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 19. HeroUI Chip Tag
components.append({
    'name': 'HeroUI Chip Tag',
    'description': 'A compact chip component for displaying tags, labels, or status indicators with optional dismiss and icon.',
    'category': 'other',
    'framework': 'react',
    'code': '''import { useState } from 'react';

export function Chip({ children, color = 'default', variant = 'solid', size = 'md', onClose, startContent, endContent, className = '' }) {
  const [visible, setVisible] = useState(true);

  if (!visible) return null;

  const colors = {
    default: { solid: 'bg-muted text-muted-foreground', bordered: 'border-border bg-background text-foreground', flat: 'bg-muted/50 text-foreground' },
    primary: { solid: 'bg-primary text-white', bordered: 'border-primary/50 bg-primary/5 text-primary', flat: 'bg-primary/10 text-primary' },
    success: { solid: 'bg-success text-white', bordered: 'border-success/50 bg-success/5 text-success', flat: 'bg-success/10 text-success' },
    warning: { solid: 'bg-warning text-warning-foreground', bordered: 'border-warning/50 bg-warning/5 text-warning', flat: 'bg-warning/10 text-warning' },
    danger: { solid: 'bg-danger text-white', bordered: 'border-danger/50 bg-danger/5 text-danger', flat: 'bg-danger/10 text-danger' },
  };

  const sizes = { sm: 'h-6 px-2 text-[11px] gap-1', md: 'h-7 px-2.5 text-xs gap-1.5', lg: 'h-9 px-3 text-sm gap-2' };

  const handleClose = (e) => {
    e.stopPropagation();
    if (onClose) onClose();
    else setVisible(false);
  };

  const colorSet = (colors)[color] || colors.default;
  const variantStyle = colorSet[variant] || colorSet.solid;

  return (
    <span className={'inline-flex items-center rounded-full font-medium ' + variantStyle + ' ' + (sizes[size] || sizes.md) + ' ' + (variant === 'bordered' ? 'border' : '') + ' ' + className}>
      {startContent}
      {children}
      {endContent}
      {onClose !== undefined && (
        <button onClick={handleClose} className="ml-0.5 rounded-full hover:bg-black/10 p-0.5 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40" aria-label="Remove">
          <svg className="size-3" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M18 6L6 18"/><path d="M6 6l12 12"/></svg>
        </button>
      )}
    </span>
  );
}''',
    'tags': ['chip', 'tag', 'badge', 'react', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 20. HeroUI Dropdown Menu
components.append({
    'name': 'HeroUI Dropdown Menu',
    'description': 'A dropdown menu displaying a list of actions or options triggered by a button click with keyboard navigation.',
    'category': 'other',
    'framework': 'react',
    'code': '''import { useState, useRef, useEffect } from 'react';

export function Dropdown({ trigger, children, align = 'start' }) {
  const [isOpen, setIsOpen] = useState(false);
  const ref = useRef(null);

  useEffect(() => {
    const handleClickOutside = (e) => {
      if (ref.current && !ref.current.contains(e.target)) setIsOpen(false);
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  useEffect(() => {
    const handleEscape = (e) => {
      if (e.key === 'Escape') setIsOpen(false);
    };
    if (isOpen) document.addEventListener('keydown', handleEscape);
    return () => document.removeEventListener('keydown', handleEscape);
  }, [isOpen]);

  const alignments = { start: 'left-0', center: 'left-1/2 -translate-x-1/2', end: 'right-0' };

  return (
    <div className="relative inline-flex" ref={ref}>
      <div onClick={() => setIsOpen(!isOpen)}>{trigger}</div>
      {isOpen && (
        <div className={'absolute top-full mt-1 z-50 min-w-[180px] rounded-xl border border-border bg-background shadow-lg p-1 ' + (alignments[align] || alignments.start)} role="menu">
          {typeof children === 'function' ? children({ onClose: () => setIsOpen(false) }) : children}
        </div>
      )}
    </div>
  );
}

Dropdown.Item = function DropdownItem({ children, onSelect, danger = false, disabled = false, shortcut }) {
  return (
    <button
      className={'flex w-full items-center rounded-lg px-3 py-2 text-sm transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40 ' + (danger ? 'text-danger hover:bg-danger/10' : 'text-foreground hover:bg-muted') + ' ' + (disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer')}
      onClick={() => !disabled && onSelect?.()}
      disabled={disabled}
      role="menuitem"
    >
      <span className="flex-1 text-left">{children}</span>
      {shortcut && <span className="ml-6 text-xs text-muted-foreground">{shortcut}</span>}
    </button>
  );
};

Dropdown.Separator = function DropdownSeparator() {
  return <div className="my-1 border-t border-border" />;
};''',
    'tags': ['dropdown', 'menu', 'react', 'navigation', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 21. HeroUI Table
components.append({
    'name': 'HeroUI Table',
    'description': 'A table component for displaying structured data with header, body, row, and cell subcomponents.',
    'category': 'table',
    'framework': 'react',
    'code': '''export function Table({ children, className = '' }) {
  return (
    <div className={'w-full overflow-auto rounded-xl border border-border ' + className}>
      <table className="w-full caption-bottom text-sm">{children}</table>
    </div>
  );
}

Table.Header = function TableHeader({ children }) {
  return <thead className="[&_tr]:border-b bg-muted/50">{children}</thead>;
};

Table.Body = function TableBody({ children }) {
  return <tbody className="[&_tr:last-child]:border-0">{children}</tbody>;
};

Table.Row = function TableRow({ children, className = '' }) {
  return (
    <tr className={'border-b border-border transition-colors hover:bg-muted/30 data-[state=selected]:bg-muted ' + className}>
      {children}
    </tr>
  );
};

Table.Head = function TableHead({ children, className = '' }) {
  return (
    <th className={'h-12 px-4 text-left align-middle font-medium text-muted-foreground ' + className}>
      {children}
    </th>
  );
};

Table.Cell = function TableCell({ children, className = '' }) {
  return (
    <td className={'p-4 align-middle ' + className}>
      {children}
    </td>
  );
};

Table.Caption = function TableCaption({ children }) {
  return <caption className="mt-4 text-xs text-muted-foreground">{children}</caption>;
};''',
    'tags': ['table', 'data', 'react', 'grid', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 22. HeroUI Toast Notification
components.append({
    'name': 'HeroUI Toast Notification',
    'description': 'A toast notification system for displaying brief, auto-dismissing messages with stacking and animation.',
    'category': 'notification',
    'framework': 'react',
    'code': '''import { useState, useEffect, useCallback } from 'react';
import { createPortal } from 'react-dom';

export function ToastContainer({ toasts, removeToast }) {
  if (!toasts || toasts.length === 0) return null;

  return createPortal(
    <div className="fixed bottom-4 right-4 z-[100] flex flex-col gap-2 max-w-sm">
      {toasts.map((toast) => (
        <Toast key={toast.id} toast={toast} onDismiss={() => removeToast?.(toast.id)} />
      ))}
    </div>,
    document.body
  );
}

function Toast({ toast, onDismiss }) {
  const [isExiting, setIsExiting] = useState(false);

  useEffect(() => {
    if (toast.duration === Infinity) return;
    const timer = setTimeout(() => {
      setIsExiting(true);
      setTimeout(onDismiss, 200);
    }, toast.duration || 4000);
    return () => clearTimeout(timer);
  }, [toast, onDismiss]);

  const styles = {
    info: 'border-info/30 bg-info/5 text-info',
    success: 'border-success/30 bg-success/5 text-success',
    warning: 'border-warning/30 bg-warning/5 text-warning',
    error: 'border-danger/30 bg-danger/5 text-danger',
  };

  const icons = {
    info: 'M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm1-13h-2v6h2V7zm0 8h-2v2h2v-2z',
    success: 'M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z',
    warning: 'M1 21h22L12 2 1 21zm12-3h-2v-2h2v2zm0-4h-2v-4h2v4z',
    error: 'M12 2C6.47 2 2 6.47 2 12s4.47 10 10 10 10-4.47 10-10S17.53 2 12 2zm5 13.59L15.59 17 12 13.41 8.41 17 7 15.59 10.59 12 7 8.41 8.41 7 12 10.59 15.59 7 17 8.41 13.41 12 17 15.59z',
  };

  return (
    <div className={'flex items-start gap-3 rounded-xl border p-4 shadow-lg backdrop-blur-sm transition-all duration-200 ' + (isExiting ? 'opacity-0 translate-x-4' : 'opacity-100 translate-x-0') + ' ' + (styles[toast.type] || styles.info)} role="alert">
      <svg className="size-5 shrink-0 mt-0.5" viewBox="0 0 24 24" fill="currentColor">
        <path d={(icons)[toast.type] || icons.info} />
      </svg>
      <div className="flex-1 min-w-0">
        {toast.title && <p className="font-semibold text-sm">{toast.title}</p>}
        {toast.description && <p className="text-sm opacity-80">{toast.description}</p>}
      </div>
      <button onClick={onDismiss} className="shrink-0 rounded-full p-0.5 hover:bg-black/10 transition-colors focus-visible:outline-none focus-visible:ring-2" aria-label="Dismiss">
        <svg className="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M18 6L6 18"/><path d="M6 6l12 12"/></svg>
      </button>
    </div>
  );
}

export function useToast() {
  const [toasts, setToasts] = useState([]);

  const addToast = useCallback((toast) => {
    const id = Date.now() + Math.random();
    setToasts(prev => [...prev, { ...toast, id }]);
    return id;
  }, []);

  const removeToast = useCallback((id) => {
    setToasts(prev => prev.filter(t => t.id !== id));
  }, []);

  const toast = {
    success: (msg) => addToast({ type: 'success', title: 'Success', description: msg, duration: 4000 }),
    error: (msg) => addToast({ type: 'error', title: 'Error', description: msg, duration: 5000 }),
    info: (msg) => addToast({ type: 'info', title: 'Info', description: msg, duration: 3000 }),
    warning: (msg) => addToast({ type: 'warning', title: 'Warning', description: msg, duration: 4000 }),
  };

  return { toasts, addToast, removeToast, toast };
}''',
    'tags': ['toast', 'notification', 'react', 'feedback', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'react-dom', 'tailwindcss']
})

# 23. HeroUI Link
components.append({
    'name': 'HeroUI Link',
    'description': 'An accessible link component with underline hover effect, variants for inline and standalone usage.',
    'category': 'other',
    'framework': 'react',
    'code': '''export function Link({ href, children, variant = 'default', isExternal = false, className = '', ...props }) {
  const variants = {
    default: 'text-primary hover:text-primary/80 underline-offset-4 hover:underline',
    muted: 'text-muted-foreground hover:text-foreground underline-offset-4 hover:underline',
    subtle: 'text-foreground no-underline hover:text-primary hover:underline underline-offset-4',
    danger: 'text-danger hover:text-danger/80 underline-offset-4 hover:underline',
  };

  return (
    <a
      href={href}
      className={'inline-flex items-center gap-1 text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40 rounded ' + (variants[variant] || variants.default) + ' ' + className}
      target={isExternal ? '_blank' : undefined}
      rel={isExternal ? 'noopener noreferrer' : undefined}
      {...props}
    >
      {children}
      {isExternal && (
        <svg className="size-3.5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
          <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>
          <polyline points="15 3 21 3 21 9"/>
          <line x1="10" y1="14" x2="21" y2="3"/>
        </svg>
      )}
    </a>
  );
}''',
    'tags': ['link', 'react', 'navigation', 'anchor', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 24. HeroUI TextArea
components.append({
    'name': 'HeroUI TextArea',
    'description': 'A multi-line text input for longer form content with resizing, character count, and label support.',
    'category': 'form',
    'framework': 'react',
    'code': '''import { forwardRef, useState } from 'react';

export const TextArea = forwardRef(function TextArea({ label, error, maxLength, showCount = false, variant = 'primary', fullWidth = false, className = '', ...props }, ref) {
  const [charCount, setCharCount] = useState(0);

  const variants = {
    primary: 'bg-background border-border shadow-sm focus-visible:border-primary focus-visible:ring-2 focus-visible:ring-primary/20',
    secondary: 'bg-surface-secondary border-border/70 focus-visible:border-primary focus-visible:ring-2 focus-visible:ring-primary/20',
  };

  return (
    <div className={'flex flex-col gap-1.5 ' + (fullWidth ? 'w-full' : '')}>
      {label && <label className="text-sm font-medium text-foreground">{label}</label>}
      <textarea
        ref={ref}
        className={'min-h-[80px] rounded-xl border px-4 py-2.5 text-sm text-foreground placeholder:text-muted-foreground/60 transition-colors outline-none resize-y hover:border-border/80 hover:bg-surface-secondary disabled:opacity-50 disabled:cursor-not-allowed disabled:bg-muted focus-visible:border-primary focus-visible:ring-2 focus-visible:ring-primary/20 aria-invalid:border-danger aria-invalid:ring-2 aria-invalid:ring-danger/20 ' + (variants[variant] || variants.primary) + ' ' + (fullWidth ? 'w-full' : '') + ' ' + className}
        onChange={(e) => { setCharCount(e.target.value.length); props.onChange?.(e); }}
        maxLength={maxLength}
        aria-invalid={error ? 'true' : undefined}
        {...props}
      />
      <div className="flex items-center justify-between">
        {error && <span className="text-xs text-danger">{error}</span>}
        {showCount && maxLength && (
          <span className={'text-xs ml-auto ' + (charCount > maxLength * 0.9 ? 'text-danger' : 'text-muted-foreground')}>
            {charCount}/{maxLength}
          </span>
        )}
      </div>
    </div>
  );
});''',
    'tags': ['textarea', 'form', 'react', 'input', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'tailwindcss']
})

# 25. HeroUI Drawer Panel
components.append({
    'name': 'HeroUI Drawer Panel',
    'description': 'A slide-out panel for additional content and navigation, commonly used for mobile menus and side panels.',
    'category': 'sidebar',
    'framework': 'react',
    'code': '''import { useState, useEffect } from 'react';
import { createPortal } from 'react-dom';

export function Drawer({ isOpen, onClose, children, placement = 'right', size = 'md' }) {
  const [isAnimating, setIsAnimating] = useState(false);

  useEffect(() => {
    if (isOpen) {
      setIsAnimating(true);
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = '';
    }
    return () => { document.body.style.overflow = ''; };
  }, [isOpen]);

  useEffect(() => {
    const handleEscape = (e) => { if (e.key === 'Escape') onClose?.(); };
    if (isOpen) document.addEventListener('keydown', handleEscape);
    return () => document.removeEventListener('keydown', handleEscape);
  }, [isOpen, onClose]);

  const placements = {
    left: { container: 'left-0 top-0 h-full', translateFrom: '-translate-x-full', translateTo: 'translate-x-0' },
    right: { container: 'right-0 top-0 h-full', translateFrom: 'translate-x-full', translateTo: 'translate-x-0' },
    top: { container: 'top-0 left-0 w-full', translateFrom: '-translate-y-full', translateTo: 'translate-y-0' },
    bottom: { container: 'bottom-0 left-0 w-full', translateFrom: 'translate-y-full', translateTo: 'translate-y-0' },
  };

  const sizes = { sm: 'max-w-sm', md: 'max-w-md', lg: 'max-w-lg', xl: 'max-w-xl', full: 'max-w-full' };

  if (!isOpen && !isAnimating) return null;

  const pl = placements[placement] || placements.right;

  return createPortal(
    <div className="fixed inset-0 z-50">
      <div
        className={'absolute inset-0 bg-black/50 transition-opacity duration-300 ' + (isOpen ? 'opacity-100' : 'opacity-0')}
        onClick={onClose}
      />
      <div
        className={'absolute bg-background shadow-2xl border-border ' + pl.container + ' ' + (sizes[size] || sizes.md) + ' transition-transform duration-300 ease-in-out ' + (isOpen ? pl.translateTo : pl.translateFrom)}
        onTransitionEnd={() => { if (!isOpen) setIsAnimating(false); }}
      >
        <div className="flex items-center justify-between p-4 border-b border-border">
          <h2 className="text-lg font-semibold text-foreground">Drawer</h2>
          <button onClick={onClose} className="rounded-full p-1.5 text-muted-foreground hover:bg-muted hover:text-foreground transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-primary/40">
            <svg className="size-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2"><path d="M18 6L6 18"/><path d="M6 6l12 12"/></svg>
          </button>
        </div>
        <div className="p-4 overflow-y-auto" style={{ maxHeight: 'calc(100vh - 73px)' }}>
          {children}
        </div>
      </div>
    </div>,
    document.body
  );
}''',
    'tags': ['drawer', 'sidebar', 'react', 'panel', 'heroui'],
    'dependencies': ['@heroui/react', 'react', 'react-dom', 'tailwindcss']
})

with open('data/heroui_components.json', 'w') as f:
    json.dump(components, f, indent=2)

print(f'Created data/heroui_components.json with {len(components)} components')
for c in components:
    print(f'  - {c["name"]} [{c["category"]}]')
