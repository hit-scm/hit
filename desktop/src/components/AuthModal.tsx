import React, { useState } from "react";
import { X, ShieldCheck, Key, LogOut, CheckCircle2, User, Globe } from "lucide-react";
import { AuthState } from "../types";
import { api } from "../api";

interface AuthModalProps {
  isOpen: boolean;
  onClose: () => void;
  authState: AuthState | null;
  onRefresh: () => void;
}

export const AuthModal: React.FC<AuthModalProps> = ({
  isOpen,
  onClose,
  authState,
  onRefresh,
}) => {
  const [provider, setProvider] = useState("github.com");
  const [tokenInput, setTokenInput] = useState("");
  const [isLoggingIn, setIsLoggingIn] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  if (!isOpen) return null;

  const handleOpenBrowserForToken = () => {
    let url = "https://github.com/settings/tokens/new?description=Hit+CLI&scopes=repo,read:org,workflow";
    if (provider === "gitlab.com") {
      url = "https://gitlab.com/-/user_settings/personal_access_tokens?name=Hit+CLI&scopes=api,read_user,read_repository,write_repository";
    } else if (provider === "codeberg.org") {
      url = "https://codeberg.org/user/settings/applications";
    } else if (provider === "bitbucket.org") {
      url = "https://bitbucket.org/account/settings/app-passwords/";
    }
    api.openBrowser(url);
  };

  const handleLoginSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!tokenInput.trim()) return;

    setIsLoggingIn(true);
    setErrorMessage(null);
    try {
      await api.loginAccount(provider, tokenInput.trim());
      setTokenInput("");
      onRefresh();
    } catch (err) {
      setErrorMessage(String(err));
    } finally {
      setIsLoggingIn(false);
    }
  };

  const handleLogout = async (host: string, user: string) => {
    await api.logoutAccount(host, user);
    onRefresh();
  };

  const hostsList = authState ? Object.entries(authState.hosts) : [];

  return (
    <div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
      <div className="w-full max-w-xl bg-[#0e1422] border border-[#223049] rounded-2xl shadow-2xl overflow-hidden flex flex-col">
        {/* Header */}
        <div className="h-14 px-6 border-b border-[#1f2c42] flex items-center justify-between">
          <div className="flex items-center gap-2">
            <ShieldCheck className="w-5 h-5 text-cyan-400" />
            <h2 className="text-base font-bold text-slate-100">Provider Authentication</h2>
          </div>
          <button
            onClick={onClose}
            className="p-1 rounded-md text-slate-400 hover:text-slate-100 hover:bg-[#1a2538] transition-colors"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Content */}
        <div className="p-6 space-y-6 overflow-y-auto max-h-[75vh]">
          {/* Active Accounts List */}
          <div>
            <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider mb-3">
              Connected Accounts ({hostsList.length})
            </h3>
            {hostsList.length === 0 ? (
              <div className="p-4 rounded-xl bg-[#111724] border border-[#1d273a] text-center text-xs text-slate-500">
                No accounts connected yet. Connect an account below to push & pull without passwords.
              </div>
            ) : (
              <div className="space-y-2">
                {hostsList.map(([host, hostAuth]) => {
                  return Object.entries(hostAuth.users).map(([username, userAuth]) => {
                    return (
                      <div
                        key={`${host}-${username}`}
                        className="p-3.5 rounded-xl bg-[#131b2a] border border-[#1f2d44] flex items-center justify-between"
                      >
                        <div className="flex items-center gap-3">
                          <div className="w-8 h-8 rounded-full bg-slate-800 border border-cyan-500/30 flex items-center justify-center text-cyan-400">
                            <User className="w-4 h-4" />
                          </div>
                          <div className="text-xs">
                            <div className="flex items-center gap-2">
                              <span className="font-bold text-slate-100">{username}</span>
                              <span className="text-[10px] px-1.5 py-0.2 rounded bg-cyan-950/70 text-cyan-300 font-mono">
                                {host}
                              </span>
                            </div>
                            <span className="text-slate-400 text-[11px] block mt-0.5">
                              {userAuth.email || "Git operations via HTTPS"}
                            </span>
                          </div>
                        </div>

                        <button
                          onClick={() => handleLogout(host, username)}
                          className="p-1.5 rounded-md text-slate-500 hover:text-rose-400 hover:bg-rose-950/30 transition-colors"
                          title="Log out"
                        >
                          <LogOut className="w-4 h-4" />
                        </button>
                      </div>
                    );
                  });
                })}
              </div>
            )}
          </div>

          {/* Connect New Account Section */}
          <div className="pt-4 border-t border-[#1b2537]">
            <h3 className="text-xs font-bold text-slate-400 uppercase tracking-wider mb-3">
              Connect Provider Account
            </h3>

            {errorMessage && (
              <div className="p-3 mb-3 rounded-lg bg-rose-950/50 border border-rose-800/50 text-xs text-rose-300">
                {errorMessage}
              </div>
            )}

            <form onSubmit={handleLoginSubmit} className="space-y-3">
              <div className="grid grid-cols-2 sm:grid-cols-4 gap-2">
                {["github.com", "gitlab.com", "codeberg.org", "bitbucket.org"].map((h) => (
                  <button
                    key={h}
                    type="button"
                    onClick={() => setProvider(h)}
                    className={`py-2 px-3 rounded-lg text-xs font-semibold border transition-all text-center truncate ${
                      provider === h
                        ? "bg-cyan-950/60 border-cyan-500 text-cyan-300"
                        : "bg-[#131b28] border-[#223049] text-slate-400 hover:text-slate-200"
                    }`}
                  >
                    {h.split(".")[0]}
                  </button>
                ))}
              </div>

              <div className="flex items-center justify-between pt-1">
                <span className="text-xs text-slate-400">Step 1: Open provider token page</span>
                <button
                  type="button"
                  onClick={handleOpenBrowserForToken}
                  className="text-xs text-cyan-400 hover:text-cyan-300 font-semibold flex items-center gap-1"
                >
                  <Globe className="w-3.5 h-3.5" />
                  <span>Open Browser</span>
                </button>
              </div>

              <div>
                <input
                  type="password"
                  placeholder="Step 2: Paste authentication token here..."
                  value={tokenInput}
                  onChange={(e) => setTokenInput(e.target.value)}
                  className="w-full bg-[#131b28] border border-[#223049] rounded-lg px-3 py-2 text-xs text-slate-100 placeholder-slate-500 focus:outline-none focus:border-cyan-500 transition-colors"
                />
              </div>

              <button
                type="submit"
                disabled={!tokenInput.trim() || isLoggingIn}
                className="w-full py-2 px-4 rounded-lg bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 text-white font-semibold text-xs disabled:opacity-40 transition-all shadow-md shadow-cyan-600/20"
              >
                {isLoggingIn ? "Verifying Token with API..." : `Connect ${provider}`}
              </button>
            </form>
          </div>
        </div>
      </div>
    </div>
  );
};
