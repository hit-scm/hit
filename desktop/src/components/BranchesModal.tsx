import React, { useState, useEffect } from "react";
import { X, GitBranch, Plus, Search, Check } from "lucide-react";
import { api } from "../api";

interface BranchesModalProps {
  isOpen: boolean;
  onClose: () => void;
  repoPath: string;
  currentBranch: string;
  onBranchSwitched: () => void;
}

export const BranchesModal: React.FC<BranchesModalProps> = ({
  isOpen,
  onClose,
  repoPath,
  currentBranch,
  onBranchSwitched,
}) => {
  const [branches, setBranches] = useState<string[]>([]);
  const [search, setSearch] = useState("");
  const [newBranchName, setNewBranchName] = useState("");
  const [isCreating, setIsCreating] = useState(false);
  const [isSwitching, setIsSwitching] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (isOpen && repoPath) {
      api.getBranches(repoPath).then(([_, list]) => {
        setBranches(list);
      });
    }
  }, [isOpen, repoPath]);

  if (!isOpen) return null;

  const handleSwitch = async (branch: string) => {
    if (branch === currentBranch) return;
    setIsSwitching(true);
    setError(null);
    try {
      await api.switchBranch(repoPath, branch, false);
      onBranchSwitched();
      onClose();
    } catch (err) {
      setError(String(err));
    } finally {
      setIsSwitching(false);
    }
  };

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!newBranchName.trim()) return;

    setIsCreating(true);
    setError(null);
    try {
      await api.switchBranch(repoPath, newBranchName.trim(), true);
      onBranchSwitched();
      onClose();
    } catch (err) {
      setError(String(err));
    } finally {
      setIsCreating(false);
    }
  };

  const filtered = branches.filter((b) =>
    b.toLowerCase().includes(search.toLowerCase())
  );

  return (
    <div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
      <div className="w-full max-w-md bg-[#0e1422] border border-[#223049] rounded-2xl shadow-2xl overflow-hidden flex flex-col">
        {/* Header */}
        <div className="h-14 px-6 border-b border-[#1f2c42] flex items-center justify-between">
          <div className="flex items-center gap-2">
            <GitBranch className="w-5 h-5 text-emerald-400" />
            <h2 className="text-base font-bold text-slate-100">Branches</h2>
          </div>
          <button
            onClick={onClose}
            className="p-1 rounded-md text-slate-400 hover:text-slate-100 hover:bg-[#1a2538] transition-colors"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Content */}
        <div className="p-6 space-y-4">
          {error && (
            <div className="p-3 rounded-lg bg-rose-950/50 border border-rose-800/50 text-xs text-rose-300">
              {error}
            </div>
          )}

          {/* Create new branch form */}
          <form onSubmit={handleCreate} className="flex gap-2">
            <input
              type="text"
              placeholder="Create and switch to new branch..."
              value={newBranchName}
              onChange={(e) => setNewBranchName(e.target.value)}
              className="flex-1 bg-[#131b28] border border-[#223049] rounded-lg px-3 py-1.5 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-cyan-500 font-mono"
            />
            <button
              type="submit"
              disabled={!newBranchName.trim() || isCreating}
              className="px-3 py-1.5 rounded-lg bg-cyan-600 hover:bg-cyan-500 text-white font-semibold text-xs disabled:opacity-40 flex items-center gap-1 shadow-md shadow-cyan-600/20"
            >
              <Plus className="w-3.5 h-3.5" />
              <span>Create</span>
            </button>
          </form>

          {/* Search branches */}
          <div className="relative flex items-center">
            <Search className="w-3.5 h-3.5 text-slate-500 absolute left-2.5 pointer-events-none" />
            <input
              type="text"
              placeholder="Filter branches..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              className="w-full bg-[#131b28] border border-[#223049] rounded-lg pl-8 pr-3 py-1.5 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-cyan-500"
            />
          </div>

          {/* Branch list */}
          <div className="max-h-60 overflow-y-auto space-y-1">
            {filtered.map((b) => {
              const isCurrent = b === currentBranch;
              return (
                <div
                  key={b}
                  onClick={() => handleSwitch(b)}
                  className={`flex items-center justify-between px-3 py-2 rounded-lg text-xs cursor-pointer font-mono transition-colors ${
                    isCurrent
                      ? "bg-emerald-950/40 text-emerald-300 border border-emerald-500/30"
                      : "hover:bg-[#151c2a] text-slate-300 border border-transparent"
                  }`}
                >
                  <div className="flex items-center gap-2">
                    <GitBranch
                      className={`w-3.5 h-3.5 ${
                        isCurrent ? "text-emerald-400" : "text-slate-500"
                      }`}
                    />
                    <span>{b}</span>
                  </div>
                  {isCurrent && (
                    <span className="text-[10px] font-bold px-1.5 py-0.2 rounded bg-emerald-950/80 text-emerald-400 border border-emerald-800/50 flex items-center gap-1">
                      <Check className="w-2.5 h-2.5" />
                      Current
                    </span>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
};
