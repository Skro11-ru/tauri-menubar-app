export interface PasswordEntry {
  id: string;
  serviceName: string;
  domain: string;
  url: string;
  username: string;
  password: string;
  has2FA: boolean;
  hasPasskey: boolean;
  isWeak: boolean;
  isReused: boolean;
  lastUpdated: string; // e.g. "3d ago", "2w ago"
  isFrequentlyUsed: boolean;
  notes?: string;
}

export type FilterType =
  | "all"
  | "frequentlyUsed"
  | "recentlyAdded"
  | "with2FA"
  | "securityIssues";
