import React, { useState, useEffect } from "react";
import { ExternalLink, RefreshCw, CheckCircle2, AlertCircle, FolderGit2 } from "lucide-react";
import { SubmoduleItem } from "../types";
import { api } from "../api";

interface SubmodulesViewProps {
  repoPath: string;
}

export const SubmodulesView: React.FC<SubmodulesViewProps> = ({ repoPath }) => {
  const [submodules, setSubmodules] = useState<SubmoduleItem[]>([]);
  const [isUpdating, setIsUpdating] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [message, setMessage] = useState<string | null>(null);

  const loadSubmodules = () => {
    if (!repoPath) return;
    setIsLoading(true);
    api
      .getSubmodules(repoPath)
      .then((items) => {
        setSubmodules(items);
        setIsLoading(false);
      })
      .catch(() => setIsLoading(false));
  };

  useEffect(() => {
    loadSubmodules();
  }, [repoPath]);

  const handleUpdateAll = async () => {
    setIsUpdating(true);
    setMessage(null);
    try {
      await api.updateSubmodules(repoPath);
      setMessage("Submodules synchronized and checked out successfully.");
      loadSubmodules();
    } catch (err) {
      setMessage(`Update failed: ${err}`);
    } finally {
      setIsUpdating(false);
    }
  };

  return (
    <div className="flex-1 bg-[#090d15] flex flex-col p-6 overflow-auto">
      <div className="max-w-4xl mx-auto w-full space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between pb-4 border-b border-[#1f2c42]">
          <div>
            <h1 className="text-lg font-bold text-slate-100 flex items-center gap-2">
              <FolderGit2 className="w-5 h-5 text-cyan-400" />
              <span>Tracked Submodules ({submodules.length})</span>
            </h1>
            <p className="text-xs text-slate-400 mt-1">
              Nested Git submodules managed through Hit's streaming submodule engine.
            </p>
          </div>
          <button
            onClick={handleUpdateAll}
            disabled={isUpdating || submodules.length === 0}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-cyan-600 hover:bg-cyan-500 text-white font-semibold text-xs disabled:opacity-40 transition-colors shadow-md shadow-cyan-600/20"
          >
            <RefreshCw className={`w-3.5 h-3.5 ${isUpdating ? "animate-spin" : ""}`} />
            <span>Update & Init All</span>
          </button>
        </div>

        {/* Status message */}
        {message && (
          <div className="p-3 rounded-lg bg-[#111c2c] border border-cyan-500/30 text-xs text-cyan-200">
            {message}
          </div>
        )}

        {/* Submodules cards */}
        {isLoading ? (
          <div className="p-12 text-center text-slate-500 text-xs animate-pulse">
            Scanning .gitmodules...
          </div>
        ) : submodules.length === 0 ? (
          <div className="p-12 text-center bg-[#0c101a] border border-[#1b2537] rounded-xl text-slate-500">
            <ExternalLink className="w-10 h-10 mx-auto mb-2 opacity-50" />
            <p className="font-semibold text-slate-300 text-sm">No Submodules Configured</p>
            <p className="text-xs text-slate-500 mt-1">
              This repository does not contain any .gitmodules entries.
            </p>
          </div>
        ) : (
          <div className="grid gap-3">
            {submodules.map((sub) => {
              const isClean = sub.status === "clean";
              const isUninit = sub.status === "uninitialized";
              const isMod = sub.status === "modified";

              return (
                <div
                  key={sub.path}
                  className="bg-[#0e1422] border border-[#1f2c42] rounded-xl p-4 flex items-center justify-between shadow-sm hover:border-[#2a3c5a] transition-all"
                >
                  <div className="space-y-1">
                    <div className="flex items-center gap-2">
                      <span className="font-mono font-bold text-slate-100 text-xs">
                        {sub.path}
                      </span>
                      {sub.branch && (
                        <span className="text-[10px] font-mono px-1.5 py-0.2 rounded bg-[#172338] text-cyan-300 border border-cyan-800/40">
                          {sub.branch}
                        </span>
                      )}
                    </div>
                    <p className="text-xs text-slate-400 font-mono flex items-center gap-1">
                      <ExternalLink className="w-3 h-3 text-slate-500 shrink-0" />
                      <span className="truncate">{sub.url}</span>
                    </p>
                  </div>

                  <div className="flex items-center gap-3">
                    <span
                      className={`text-xs font-medium px-2.5 py-1 rounded-full flex items-center gap-1 ${
                        isClean
                          ? "bg-emerald-950/60 text-emerald-300 border border-emerald-800/40"
                          : isUninit
                          ? "bg-amber-950/60 text-amber-300 border border-amber-800/40"
                          : isMod
                          ? "bg-blue-950/60 text-blue-300 border border-blue-800/40"
                          : "bg-rose-950/60 text-rose-300 border border-rose-800/40"
                      }`}
                    >
                      {isClean ? (
                        <CheckCircle2 className="w-3 h-3" />
                      ) : (
                        <AlertCircle className="w-3 h-3" />
                      )}
                      <span className="capitalize">{sub.status}</span>
                    </span>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
};
