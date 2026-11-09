import React from "react";
import {
  GitBranch,
  RefreshCw,
  ArrowUp,
  ArrowDown,
  Activity,
  Layers,
  History,
  FolderGit2,
  User,
  ExternalLink,
} from "lucide-react";
import { RepoInfo, RepoStatus, ViewMode, AuthState } from "../types";

interface HeaderProps {
  repo: RepoInfo | null;
  status: RepoStatus | null;
  viewMode: ViewMode;
  setViewMode: (mode: ViewMode) => void;
  authState: AuthState | null;
  onOpenBranches: () => void;
  onOpenDoctor: () => void;
  onOpenAuth: () => void;
  onSync: () => void;
  onPush: () => void;
  onPull: () => void;
  onSelectRepo: () => void;
  isSyncing: boolean;
}

export const Header: React.FC<HeaderProps> = ({
  repo,
  status,
  viewMode,
  setViewMode,
  authState,
  onOpenBranches,
  onOpenDoctor,
  onOpenAuth,
  onSync,
  onPush,
  onPull,
  onSelectRepo,
  isSyncing,
}) => {
  const changedCount =
    (status?.staged.length || 0) +
    (status?.unstaged.length || 0) +
    (status?.untracked.length || 0);

  // Active user across hosts
  const firstHost = authState?.hosts ? Object.values(authState.hosts)[0] : null;
  const activeUser = firstHost
    ? firstHost.users[firstHost.active_user] || null
    : null;

  return (
    <header className="h-14 bg-[#0d121d] border-b border-[#1f293d] flex items-center justify-between px-4 select-none shrink-0 z-30">
      {/* Left: Brand & Repo selector */}
      <div className="flex items-center gap-3">
        <div className="flex items-center gap-2 pr-3 border-r border-[#1f293d]">
          <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center shadow-lg shadow-cyan-500/20">
            <span className="font-extrabold text-white text-base tracking-tighter">H</span>
          </div>
          <span className="font-bold text-slate-100 tracking-tight text-sm hidden sm:inline">
            Hit <span className="text-cyan-400 font-medium text-xs ml-0.5 px-1.5 py-0.5 rounded bg-cyan-950/60 border border-cyan-800/40">Desktop</span>
          </span>
        </div>

        {/* Current Repository Selector */}
        <button
          onClick={onSelectRepo}
          className="flex items-center gap-2 px-2.5 py-1.5 rounded-md hover:bg-[#192235] text-slate-200 text-xs font-medium border border-transparent hover:border-[#273548] transition-colors"
          title={repo?.path || "Open repository"}
        >
          <FolderGit2 className="w-3.5 h-3.5 text-cyan-400" />
          <span className="font-semibold">{repo?.name || "Select Repository"}</span>
        </button>

        {/* Current Branch Dropdown */}
        {repo && (
          <button
            onClick={onOpenBranches}
            className="flex items-center gap-1.5 px-2.5 py-1.5 rounded-md bg-[#131b2a] hover:bg-[#1a253a] text-slate-200 text-xs font-mono font-medium border border-[#223049] transition-all hover:border-cyan-500/40"
          >
            <GitBranch className="w-3.5 h-3.5 text-emerald-400" />
            <span>{repo.current_branch}</span>
          </button>
        )}
      </div>

      {/* Middle: Navigation tabs */}
      <div className="flex items-center bg-[#090d15] p-1 rounded-lg border border-[#1b2537]">
        <button
          onClick={() => setViewMode("changes")}
          className={`flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-medium transition-all ${
            viewMode === "changes"
              ? "bg-[#1c273a] text-cyan-400 shadow-sm"
              : "text-slate-400 hover:text-slate-200"
          }`}
        >
          <Layers className="w-3.5 h-3.5" />
          <span>Changes</span>
          {changedCount > 0 && (
            <span className="ml-1 px-1.5 py-0.2 rounded-full text-[10px] font-bold bg-cyan-500/20 text-cyan-300">
              {changedCount}
            </span>
          )}
        </button>

        <button
          onClick={() => setViewMode("history")}
          className={`flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-medium transition-all ${
            viewMode === "history"
              ? "bg-[#1c273a] text-cyan-400 shadow-sm"
              : "text-slate-400 hover:text-slate-200"
          }`}
        >
          <History className="w-3.5 h-3.5" />
          <span>History</span>
        </button>

        <button
          onClick={() => setViewMode("submodules")}
          className={`flex items-center gap-1.5 px-3 py-1 rounded-md text-xs font-medium transition-all ${
            viewMode === "submodules"
              ? "bg-[#1c273a] text-cyan-400 shadow-sm"
              : "text-slate-400 hover:text-slate-200"
          }`}
        >
          <ExternalLink className="w-3.5 h-3.5" />
          <span>Submodules</span>
        </button>
      </div>

      {/* Right: Sync, Doctor, Auth */}
      <div className="flex items-center gap-2">
        {/* Sync / Push / Pull actions */}
        {repo && (
          <div className="flex items-center gap-1 bg-[#131b2a] rounded-md border border-[#223049] p-0.5">
            <button
              onClick={onSync}
              disabled={isSyncing}
              className="flex items-center gap-1.5 px-2.5 py-1 rounded hover:bg-[#1c273a] text-xs font-medium text-slate-200 disabled:opacity-50 transition-colors"
              title="One-step Fetch, Rebase, and Push"
            >
              <RefreshCw
                className={`w-3.5 h-3.5 text-cyan-400 ${
                  isSyncing ? "animate-spin" : ""
                }`}
              />
              <span className="font-semibold">Sync</span>
              {(repo.ahead > 0 || repo.behind > 0) && (
                <div className="flex items-center gap-1 ml-1 text-[11px] font-mono">
                  {repo.ahead > 0 && (
                    <span className="text-emerald-400 flex items-center">
                      <ArrowUp className="w-2.5 h-2.5" />
                      {repo.ahead}
                    </span>
                  )}
                  {repo.behind > 0 && (
                    <span className="text-amber-400 flex items-center">
                      <ArrowDown className="w-2.5 h-2.5" />
                      {repo.behind}
                    </span>
                  )}
                </div>
              )}
            </button>

            <button
              onClick={onPull}
              className="p-1 rounded hover:bg-[#1c273a] text-slate-400 hover:text-slate-200 transition-colors"
              title="Pull changes from remote"
            >
              <ArrowDown className="w-3.5 h-3.5 text-slate-300" />
            </button>

            <button
              onClick={onPush}
              className="p-1 rounded hover:bg-[#1c273a] text-slate-400 hover:text-slate-200 transition-colors"
              title="Push changes to remote"
            >
              <ArrowUp className="w-3.5 h-3.5 text-slate-300" />
            </button>
          </div>
        )}

        {/* Doctor Health Diagnostic */}
        <button
          onClick={onOpenDoctor}
          className="p-2 rounded-md hover:bg-[#192235] text-slate-400 hover:text-cyan-400 border border-transparent hover:border-[#273548] transition-colors"
          title="Hit Doctor - System & Engine Diagnostics"
        >
          <Activity className="w-4 h-4" />
        </button>

        {/* Auth profile badge */}
        <button
          onClick={onOpenAuth}
          className="flex items-center gap-1.5 px-2.5 py-1.5 rounded-md hover:bg-[#192235] text-slate-200 text-xs border border-transparent hover:border-[#273548] transition-colors"
          title="Authentication & Connected Providers"
        >
          <div className="w-5 h-5 rounded-full bg-slate-800 border border-cyan-500/40 flex items-center justify-center text-cyan-400">
            <User className="w-3 h-3" />
          </div>
          <span className="font-medium hidden md:inline">
            {activeUser ? activeUser.username : "Sign In"}
          </span>
        </button>
      </div>
    </header>
  );
};
