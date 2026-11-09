import React, { useState, useEffect } from "react";
import {
  Check,
  Plus,
  Minus,
  FileText,
  CornerDownLeft,
  CheckCircle2,
  FileQuestion,
  FileX,
  FileDiff,
} from "lucide-react";
import { RepoStatus, ChangedFile } from "../types";
import { api } from "../api";

interface ChangesViewProps {
  repoPath: string;
  status: RepoStatus | null;
  onRefresh: () => void;
  onCommit: (message: string) => Promise<void>;
  isCommitting: boolean;
}

export const ChangesView: React.FC<ChangesViewProps> = ({
  repoPath,
  status,
  onRefresh,
  onCommit,
  isCommitting,
}) => {
  const [selectedFile, setSelectedFile] = useState<ChangedFile | null>(null);
  const [diffContent, setDiffContent] = useState<string>("");
  const [isLoadingDiff, setIsLoadingDiff] = useState(false);
  const [commitSummary, setCommitSummary] = useState("");
  const [commitDesc, setCommitDesc] = useState("");

  const allChangedFiles = [
    ...(status?.staged || []),
    ...(status?.unstaged || []),
    ...(status?.untracked || []),
  ];

  // Auto-select first file if none selected
  useEffect(() => {
    if (!selectedFile && allChangedFiles.length > 0) {
      setSelectedFile(allChangedFiles[0]);
    } else if (
      selectedFile &&
      !allChangedFiles.some((f) => f.path === selectedFile.path)
    ) {
      setSelectedFile(allChangedFiles[0] || null);
    }
  }, [status]);

  // Load diff when selected file changes
  useEffect(() => {
    if (!selectedFile || !repoPath) {
      setDiffContent("");
      return;
    }

    let isCurrent = true;
    setIsLoadingDiff(true);

    api
      .getFileDiff(repoPath, selectedFile.path, selectedFile.is_staged)
      .then((diff) => {
        if (isCurrent) {
          setDiffContent(diff);
          setIsLoadingDiff(false);
        }
      })
      .catch((err) => {
        if (isCurrent) {
          setDiffContent(`Error loading diff: ${err}`);
          setIsLoadingDiff(false);
        }
      });

    return () => {
      isCurrent = false;
    };
  }, [selectedFile, repoPath]);

  const handleStageFile = async (file: string) => {
    await api.stageFile(repoPath, file);
    onRefresh();
  };

  const handleUnstageFile = async (file: string) => {
    await api.unstageFile(repoPath, file);
    onRefresh();
  };

  const handleStageAll = async () => {
    await api.stageAll(repoPath);
    onRefresh();
  };

  const handleUnstageAll = async () => {
    await api.unstageAll(repoPath);
    onRefresh();
  };

  const handleCommitSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!commitSummary.trim()) return;

    const fullMessage = commitDesc.trim()
      ? `${commitSummary.trim()}\n\n${commitDesc.trim()}`
      : commitSummary.trim();

    await onCommit(fullMessage);
    setCommitSummary("");
    setCommitDesc("");
  };

  const stagedCount = status?.staged.length || 0;

  const renderBadge = (statusChar: string) => {
    switch (statusChar) {
      case "A":
        return <span className="text-emerald-400 font-mono text-[11px] font-bold">A</span>;
      case "D":
        return <span className="text-rose-400 font-mono text-[11px] font-bold">D</span>;
      case "R":
        return <span className="text-purple-400 font-mono text-[11px] font-bold">R</span>;
      case "?":
        return <span className="text-cyan-400 font-mono text-[11px] font-bold">?</span>;
      default:
        return <span className="text-amber-400 font-mono text-[11px] font-bold">M</span>;
    }
  };

  return (
    <div className="flex-1 flex overflow-hidden">
      {/* Left Column: File Tree & Commit Box */}
      <div className="w-80 border-r border-[#1f293d] bg-[#0c101a] flex flex-col shrink-0">
        {/* Working tree header */}
        <div className="h-10 border-b border-[#1b2537] px-3 flex items-center justify-between text-xs text-slate-300 font-semibold">
          <span>Changed Files ({allChangedFiles.length})</span>
          <div className="flex items-center gap-1">
            {stagedCount > 0 ? (
              <button
                onClick={handleUnstageAll}
                className="px-2 py-0.5 rounded text-[11px] font-medium bg-[#162031] hover:bg-[#1e2b42] text-slate-300 border border-[#23324c]"
              >
                Unstage All
              </button>
            ) : (
              <button
                onClick={handleStageAll}
                className="px-2 py-0.5 rounded text-[11px] font-medium bg-[#162031] hover:bg-[#1e2b42] text-cyan-400 border border-[#23324c]"
              >
                Stage All
              </button>
            )}
          </div>
        </div>

        {/* Files List */}
        <div className="flex-1 overflow-y-auto p-2 space-y-4">
          {allChangedFiles.length === 0 ? (
            <div className="h-48 flex flex-col items-center justify-center text-center p-4">
              <CheckCircle2 className="w-10 h-10 text-emerald-400 mb-2 opacity-80" />
              <p className="font-semibold text-slate-200 text-sm">Working Tree Clean</p>
              <p className="text-xs text-slate-500 mt-1">No unstaged or untracked changes.</p>
            </div>
          ) : (
            <>
              {/* Staged files section */}
              {status && status.staged.length > 0 && (
                <div>
                  <div className="flex items-center justify-between text-[11px] font-bold text-slate-400 uppercase tracking-wider px-2 mb-1.5">
                    <span>Staged Changes ({status.staged.length})</span>
                    <button
                      onClick={handleUnstageAll}
                      className="text-slate-500 hover:text-slate-300 lowercase font-normal"
                    >
                      unstage
                    </button>
                  </div>
                  <div className="space-y-0.5">
                    {status.staged.map((f) => {
                      const isSelected = selectedFile?.path === f.path && selectedFile.is_staged;
                      return (
                        <div
                          key={`staged-${f.path}`}
                          onClick={() => setSelectedFile(f)}
                          className={`flex items-center justify-between px-2 py-1.5 rounded-md text-xs cursor-pointer group transition-colors ${
                            isSelected
                              ? "bg-cyan-950/60 text-cyan-200 border border-cyan-500/30"
                              : "hover:bg-[#151c2a] text-slate-300 border border-transparent"
                          }`}
                        >
                          <div className="flex items-center gap-2 overflow-hidden">
                            <button
                              onClick={(e) => {
                                e.stopPropagation();
                                handleUnstageFile(f.path);
                              }}
                              className="text-emerald-400 hover:text-slate-400 shrink-0"
                              title="Unstage file"
                            >
                              <Check className="w-3.5 h-3.5" />
                            </button>
                            <span className="truncate font-mono text-[11.5px]">{f.path}</span>
                          </div>
                          <div className="shrink-0 ml-2">{renderBadge(f.status)}</div>
                        </div>
                      );
                    })}
                  </div>
                </div>
              )}

              {/* Unstaged files section */}
              {status && status.unstaged.length > 0 && (
                <div>
                  <div className="flex items-center justify-between text-[11px] font-bold text-slate-400 uppercase tracking-wider px-2 mb-1.5">
                    <span>Unstaged Changes ({status.unstaged.length})</span>
                    <button
                      onClick={handleStageAll}
                      className="text-cyan-400 hover:text-cyan-300 lowercase font-normal"
                    >
                      stage all
                    </button>
                  </div>
                  <div className="space-y-0.5">
                    {status.unstaged.map((f) => {
                      const isSelected = selectedFile?.path === f.path && !selectedFile.is_staged;
                      return (
                        <div
                          key={`unstaged-${f.path}`}
                          onClick={() => setSelectedFile(f)}
                          className={`flex items-center justify-between px-2 py-1.5 rounded-md text-xs cursor-pointer group transition-colors ${
                            isSelected
                              ? "bg-cyan-950/60 text-cyan-200 border border-cyan-500/30"
                              : "hover:bg-[#151c2a] text-slate-300 border border-transparent"
                          }`}
                        >
                          <div className="flex items-center gap-2 overflow-hidden">
                            <button
                              onClick={(e) => {
                                e.stopPropagation();
                                handleStageFile(f.path);
                              }}
                              className="text-slate-600 hover:text-cyan-400 shrink-0"
                              title="Stage file"
                            >
                              <Plus className="w-3.5 h-3.5" />
                            </button>
                            <span className="truncate font-mono text-[11.5px]">{f.path}</span>
                          </div>
                          <div className="shrink-0 ml-2">{renderBadge(f.status)}</div>
                        </div>
                      );
                    })}
                  </div>
                </div>
              )}

              {/* Untracked files section */}
              {status && status.untracked.length > 0 && (
                <div>
                  <div className="flex items-center justify-between text-[11px] font-bold text-slate-400 uppercase tracking-wider px-2 mb-1.5">
                    <span>Untracked Files ({status.untracked.length})</span>
                  </div>
                  <div className="space-y-0.5">
                    {status.untracked.map((f) => {
                      const isSelected = selectedFile?.path === f.path;
                      return (
                        <div
                          key={`untracked-${f.path}`}
                          onClick={() => setSelectedFile(f)}
                          className={`flex items-center justify-between px-2 py-1.5 rounded-md text-xs cursor-pointer group transition-colors ${
                            isSelected
                              ? "bg-cyan-950/60 text-cyan-200 border border-cyan-500/30"
                              : "hover:bg-[#151c2a] text-slate-300 border border-transparent"
                          }`}
                        >
                          <div className="flex items-center gap-2 overflow-hidden">
                            <button
                              onClick={(e) => {
                                e.stopPropagation();
                                handleStageFile(f.path);
                              }}
                              className="text-slate-600 hover:text-cyan-400 shrink-0"
                              title="Stage untracked file"
                            >
                              <Plus className="w-3.5 h-3.5" />
                            </button>
                            <span className="truncate font-mono text-[11.5px] text-slate-400">
                              {f.path}
                            </span>
                          </div>
                          <div className="shrink-0 ml-2">{renderBadge("?")}</div>
                        </div>
                      );
                    })}
                  </div>
                </div>
              )}
            </>
          )}
        </div>

        {/* Commit Dock at bottom */}
        <div className="border-t border-[#1f293d] p-3 bg-[#0d121d] flex flex-col gap-2">
          <form onSubmit={handleCommitSubmit} className="space-y-2">
            <input
              type="text"
              placeholder="Commit summary (e.g. feat: add token parser)"
              value={commitSummary}
              onChange={(e) => setCommitSummary(e.target.value)}
              className="w-full bg-[#131b28] border border-[#223049] rounded-md px-2.5 py-1.5 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-cyan-500 transition-colors"
            />
            <textarea
              placeholder="Extended description (optional)"
              rows={2}
              value={commitDesc}
              onChange={(e) => setCommitDesc(e.target.value)}
              className="w-full bg-[#131b28] border border-[#223049] rounded-md px-2.5 py-1.5 text-xs text-slate-200 placeholder-slate-500 focus:outline-none focus:border-cyan-500 resize-none transition-colors"
            />
            <button
              type="submit"
              disabled={!commitSummary.trim() || stagedCount === 0 || isCommitting}
              className="w-full py-2 px-3 rounded-md bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 text-white font-semibold text-xs disabled:opacity-40 disabled:cursor-not-allowed shadow-md shadow-cyan-600/20 flex items-center justify-center gap-1.5 transition-all"
            >
              <CornerDownLeft className="w-3.5 h-3.5" />
              <span>
                {isCommitting
                  ? "Committing..."
                  : `Commit to ${status?.branch || "main"} (${stagedCount} staged)`}
              </span>
            </button>
          </form>
        </div>
      </div>

      {/* Right Column: Unified Diff Viewer */}
      <div className="flex-1 bg-[#090d15] flex flex-col overflow-hidden">
        {selectedFile ? (
          <>
            {/* Diff Header */}
            <div className="h-10 border-b border-[#1b2537] px-4 flex items-center justify-between text-xs text-slate-300 bg-[#0d121d]">
              <div className="flex items-center gap-2">
                <FileDiff className="w-4 h-4 text-cyan-400" />
                <span className="font-mono font-medium">{selectedFile.path}</span>
                <span className="text-[11px] px-1.5 py-0.5 rounded bg-[#172133] text-slate-400 font-mono">
                  {selectedFile.is_staged ? "Staged Diff" : "Working Tree Diff"}
                </span>
              </div>
              <div>
                {selectedFile.is_staged ? (
                  <button
                    onClick={() => handleUnstageFile(selectedFile.path)}
                    className="flex items-center gap-1 px-2.5 py-1 rounded bg-[#162031] hover:bg-[#1e2b42] text-xs text-amber-400 border border-[#253651] transition-colors"
                  >
                    <Minus className="w-3.5 h-3.5" />
                    <span>Unstage</span>
                  </button>
                ) : (
                  <button
                    onClick={() => handleStageFile(selectedFile.path)}
                    className="flex items-center gap-1 px-2.5 py-1 rounded bg-[#162031] hover:bg-[#1e2b42] text-xs text-emerald-400 border border-[#253651] transition-colors"
                  >
                    <Plus className="w-3.5 h-3.5" />
                    <span>Stage File</span>
                  </button>
                )}
              </div>
            </div>

            {/* Diff content view */}
            <div className="flex-1 overflow-auto p-3 font-mono text-xs leading-relaxed">
              {isLoadingDiff ? (
                <div className="h-full flex items-center justify-center text-slate-500">
                  <span className="animate-pulse">Loading diff...</span>
                </div>
              ) : diffContent.trim() ? (
                <div className="space-y-0.5">
                  {diffContent.split("\n").map((line, idx) => {
                    let className = "text-slate-300 px-2 py-0.5 rounded-sm";
                    if (line.startsWith("+") && !line.startsWith("+++")) {
                      className += " diff-line-add";
                    } else if (line.startsWith("-") && !line.startsWith("---")) {
                      className += " diff-line-del";
                    } else if (line.startsWith("@@")) {
                      className += " diff-line-hunk text-blue-400 my-1";
                    } else if (line.startsWith("diff ") || line.startsWith("index ")) {
                      className += " text-slate-500 font-semibold";
                    }
                    return (
                      <div key={idx} className={className}>
                        <span className="inline-block w-8 text-right pr-3 select-none text-slate-600 text-[10px]">
                          {idx + 1}
                        </span>
                        <span>{line}</span>
                      </div>
                    );
                  })}
                </div>
              ) : (
                <div className="h-full flex items-center justify-center text-slate-500">
                  No textual differences found.
                </div>
              )}
            </div>
          </>
        ) : (
          <div className="h-full flex flex-col items-center justify-center text-slate-500">
            <FileText className="w-12 h-12 mb-2 opacity-50" />
            <p className="text-sm font-medium">Select a changed file to inspect the diff</p>
          </div>
        )}
      </div>
    </div>
  );
};
