export interface RepoInfo {
  path: string;
  name: string;
  current_branch: string;
  remotes: [string, string][];
  is_clean: boolean;
  ahead: number;
  behind: number;
}

export interface ChangedFile {
  path: string;
  status: string; // 'M' | 'A' | 'D' | 'R' | '?'
  is_staged: boolean;
}

export interface RepoStatus {
  branch: string;
  ahead: number;
  behind: number;
  staged: ChangedFile[];
  unstaged: ChangedFile[];
  untracked: ChangedFile[];
  is_clean: boolean;
}

export interface CommitItem {
  sha: string;
  short_sha: string;
  message: string;
  author_name: string;
  author_email: string;
  date: string;
}

export interface SubmoduleItem {
  name: string;
  path: string;
  url: string;
  branch?: string;
  status: 'clean' | 'modified' | 'uninitialized' | 'conflict' | string;
}

export interface DoctorReport {
  core_version: string;
  git_version: string;
  hashing_engine: string;
  global_config: boolean;
  authenticated_hosts: string[];
  author_name: string;
  author_email: string;
}

export interface AccountAuth {
  username: string;
  name?: string;
  email?: string;
  token: string;
  git_protocol: string;
  scopes: string[];
  authenticated_at: string;
}

export interface HostAuth {
  active_user: string;
  users: Record<string, AccountAuth>;
}

export interface AuthState {
  hosts: Record<string, HostAuth>;
}

export type ViewMode = 'changes' | 'history' | 'submodules';
