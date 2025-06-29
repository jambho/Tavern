export interface ThemeColors {
  // Primary colors
  primary: {
    50: string;
    100: string;
    200: string;
    300: string;
    400: string;
    500: string;
    600: string;
    700: string;
    800: string;
    900: string;
  };
  
  // Neutral colors
  neutral: {
    50: string;
    100: string;
    200: string;
    300: string;
    400: string;
    500: string;
    600: string;
    700: string;
    800: string;
    900: string;
  };
  
  // Semantic colors
  success: {
    50: string;
    100: string;
    500: string;
    600: string;
  };
  
  warning: {
    50: string;
    100: string;
    500: string;
    600: string;
  };
  
  error: {
    50: string;
    100: string;
    500: string;
    600: string;
  };
  
  // UI specific colors
  background: {
    primary: string;
    secondary: string;
    tertiary: string;
  };
  
  surface: {
    primary: string;
    secondary: string;
    border: string;
  };
  
  text: {
    primary: string;
    secondary: string;
    tertiary: string;
    inverse: string;
  };
}

export interface Theme {
  name: string;
  colors: ThemeColors;
  borderRadius: {
    sm: string;
    md: string;
    lg: string;
    xl: string;
  };
  spacing: {
    xs: string;
    sm: string;
    md: string;
    lg: string;
    xl: string;
  };
  shadows: {
    sm: string;
    md: string;
    lg: string;
    xl: string;
  };
}

export type ThemeMode = 'light' | 'dark'; 