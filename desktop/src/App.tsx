import React, { useState, useEffect } from "react";
import { Header } from "./components/Header";
import { ChangesView } from "./components/ChangesView";
import { HistoryView } from "./components/HistoryView";
import { SubmodulesView } from "./components/SubmodulesView";
import { BranchesModal } from "./components/BranchesModal";
import { DoctorModal } from "./components/DoctorModal";
import { AuthModal } from "./components/AuthModal";
import { RepoInfo, RepoStatus, ViewMode, AuthState } from "./types";
import { api } from "./api";

export const App: React.FC = () => {
  const [repoPath, setRepoPath] = useState<string>("/Users/byjonas/Documents/GitHub/hit");
  const [repo, setRepo] = useState<RepoInfo | null>(null);
  const [status, setStatus] = useState<RepoStatus | null>(null);
  const [authState, setAuthState] = useState<AuthState | null>(null);
  const [viewMode, setViewMode] = useState<ViewMode>("changes");

  // Modals state
  const [isBranchesOpen, setIsBranchesOpen] = useState(false);
  const [isDoctorOpen, setIsDoctorOpen] = useState(false);
  const [isAuthOpen, setIsAuthOpen] = useState(false);

  // Operation flags
  const [isSyncing, setIsSyncing] = useState(false);
  const [isCommitting, setIsCommitting] = useState(false);
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const showToast = (msg: string) => {
    setToastMessage(msg);
    setTimeout(() => setToastMessage(null), 3000);
  };

  const loadRepoData = async () => {
    try {
      const info = await api.openRepository(repoPath);
      setRepo(info);
      const st = await api.getStatus(repoPath);
      setStatus(st);
    } catch (err) {
      console.error("Failed to load repo:", err);
    }
  };

  const loadAuth = async () => {
    try {
      const state = await api.getAuthState();
      setAuthState(state);
    } catch (err) {
      console.error("Failed to load auth:", err);
    }
  };

  useEffect(() => {
    loadRepoData();
    loadAuth();
  }, [repoPath]);

  // Sync action (One-step fetch, rebase, and push)
  const handleSync = async () => {
    setIsSyncing(true);
    try {
      await api.syncRepo(repoPath);
      showToast("Repository synchronized with remote upstream.");
      await loadRepoData();
    } catch (err) {
      showToast(`Sync error: ${err}`);
    } finally {
      setIsSyncing(false);
    }
  };

  // Push action
  const handlePush = async () => {
    setIsSyncing(true);
    try {
      await api.pushRepo(repoPath);
      showToast("Pushed commits to remote.");
      await loadRepoData();
    } catch (err) {
      showToast(`Push error: ${err}`);
    } finally {
      setIsSyncing(false);
    }
  };

  // Pull action
  const handlePull = async () => {
    setIsSyncing(true);
    try {
      await api.pullRepo(repoPath);
      showToast("Pulled and integrated remote changes.");
      await loadRepoData();
    } catch (err) {
      showToast(`Pull error: ${err}`);
    } finally {
      setIsSyncing(false);
    }
  };

  // Commit action
  const handleCommit = async (message: string) => {
    setIsCommitting(true);
    try {
      const sha = await api.commit(repoPath, message);
      showToast(`Committed [${sha}] ${message.split("\n")[0]}`);
      await loadRepoData();
    } catch (err) {
      showToast(`Commit error: ${err}`);
    } finally {
      setIsCommitting(false);
    }
  };

  // Repository path prompt
  const handleSelectRepo = () => {
    const newPath = prompt("Enter full path to Hit / Git repository:", repoPath);
    if (newPath && newPath.trim()) {
      setRepoPath(newPath.trim());
    }
  };

  return (
    <div className="h-screen w-screen flex flex-col bg-[#0b0f17] text-slate-100 overflow-hidden font-sans">
      {/* Top Header */}
      <Header
        repo={repo}
        status={status}
        viewMode={viewMode}
        setViewMode={setViewMode}
        authState={authState}
        onOpenBranches={() => setIsBranchesOpen(true)}
        onOpenDoctor={() => setIsDoctorOpen(true)}
        onOpenAuth={() => setIsAuthOpen(true)}
        onSync={handleSync}
        onPush={handlePush}
        onPull={handlePull}
        onSelectRepo={handleSelectRepo}
        isSyncing={isSyncing}
      />

      {/* Main View Area */}
      <main className="flex-1 flex overflow-hidden">
        {viewMode === "changes" && (
          <ChangesView
            repoPath={repoPath}
            status={status}
            onRefresh={loadRepoData}
            onCommit={handleCommit}
            isCommitting={isCommitting}
          />
        )}

        {viewMode === "history" && <HistoryView repoPath={repoPath} />}

        {viewMode === "submodules" && <SubmodulesView repoPath={repoPath} />}
      </main>

      {/* Toast Notification */}
      {toastMessage && (
        <div className="fixed bottom-5 right-5 z-50 px-4 py-2.5 rounded-xl bg-[#111827] border border-cyan-500/40 text-slate-100 text-xs font-semibold shadow-2xl flex items-center gap-2 animate-bounce">
          <span className="w-2 h-2 rounded-full bg-cyan-400"></span>
          <span>{toastMessage}</span>
        </div>
      )}

      {/* Modals */}
      <BranchesModal
        isOpen={isBranchesOpen}
        onClose={() => setIsBranchesOpen(false)}
        repoPath={repoPath}
        currentBranch={repo?.current_branch || "main"}
        onBranchSwitched={loadRepoData}
      />

      <DoctorModal
        isOpen={isDoctorOpen}
        onClose={() => setIsDoctorOpen(false)}
      />

      <AuthModal
        isOpen={isAuthOpen}
        onClose={() => setIsAuthOpen(false)}
        authState={authState}
        onRefresh={loadAuth}
      />
    </div>
  );
};
export default App;
