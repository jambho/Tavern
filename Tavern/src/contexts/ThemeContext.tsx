import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { Theme, ThemeMode } from '../types/theme';
import { themes } from '../utils/themes';

interface ThemeContextType {
  theme: Theme;
  themeMode: ThemeMode;
  toggleTheme: () => void;
  setThemeMode: (mode: ThemeMode) => void;
}

const ThemeContext = createContext<ThemeContextType | undefined>(undefined);

interface ThemeProviderProps {
  children: ReactNode;
}

export const ThemeProvider: React.FC<ThemeProviderProps> = ({ children }) => {
  const [themeMode, setThemeMode] = useState<ThemeMode>('light');
  
  // Get theme based on current mode
  const theme = themes[themeMode];
  
  // Toggle between light and dark themes
  const toggleTheme = () => {
    setThemeMode(prev => prev === 'light' ? 'dark' : 'light');
  };
  
  // Load theme preference from localStorage on mount
  useEffect(() => {
    const savedTheme = localStorage.getItem('theme-mode') as ThemeMode;
    if (savedTheme && (savedTheme === 'light' || savedTheme === 'dark')) {
      setThemeMode(savedTheme);
    }
  }, []);
  
  // Save theme preference to localStorage when it changes
  useEffect(() => {
    localStorage.setItem('theme-mode', themeMode);
    
    // Apply theme to document root for CSS custom properties
    const root = document.documentElement;
    
    // Set CSS custom properties for the theme
    Object.entries(theme.colors).forEach(([category, colors]) => {
      if (typeof colors === 'object' && colors !== null) {
        Object.entries(colors as Record<string, string>).forEach(([shade, color]) => {
          root.style.setProperty(`--color-${category}-${shade}`, color);
        });
      }
    });
    
    // Set other theme properties
    Object.entries(theme.borderRadius).forEach(([size, value]) => {
      root.style.setProperty(`--radius-${size}`, value);
    });
    
    Object.entries(theme.spacing).forEach(([size, value]) => {
      root.style.setProperty(`--spacing-${size}`, value);
    });
    
    Object.entries(theme.shadows).forEach(([size, value]) => {
      root.style.setProperty(`--shadow-${size}`, value);
    });
    
    // Set theme mode class on body
    document.body.className = `theme-${themeMode}`;
  }, [theme, themeMode]);
  
  const value: ThemeContextType = {
    theme,
    themeMode,
    toggleTheme,
    setThemeMode,
  };
  
  return (
    <ThemeContext.Provider value={value}>
      {children}
    </ThemeContext.Provider>
  );
};

export const useTheme = (): ThemeContextType => {
  const context = useContext(ThemeContext);
  if (context === undefined) {
    throw new Error('useTheme must be used within a ThemeProvider');
  }
  return context;
}; 