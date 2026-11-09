import React, { useEffect, useState } from "react";
import { X, Activity, Cpu, CheckCircle2, ShieldCheck, Terminal, User } from "lucide-react";
import { DoctorReport } from "../types";
import { api } from "../api";

interface DoctorModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const DoctorModal: React.FC<DoctorModalProps> = ({ isOpen, onClose }) => {
  const [report, setReport] = useState<DoctorReport | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    if (isOpen) {
      setIsLoading(true);
      api
        .runDoctor()
        .then((r) => {
          setReport(r);
          setIsLoading(false);
        })
        .catch(() => setIsLoading(false));
    }
  }, [isOpen]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4">
      <div className="w-full max-w-lg bg-[#0e1422] border border-[#223049] rounded-2xl shadow-2xl overflow-hidden flex flex-col">
        {/* Header */}
        <div className="h-14 px-6 border-b border-[#1f2c42] flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Activity className="w-5 h-5 text-cyan-400" />
            <h2 className="text-base font-bold text-slate-100">Hit Engine Doctor</h2>
          </div>
          <button
            onClick={onClose}
            className="p-1 rounded-md text-slate-400 hover:text-slate-100 hover:bg-[#1a2538] transition-colors"
          >
            <X className="w-4 h-4" />
          </button>
        </div>

        {/* Content */}
        <div className="p-6 space-y-4 overflow-y-auto max-h-[70vh]">
          {isLoading || !report ? (
            <div className="p-8 text-center text-slate-500 text-xs animate-pulse">
              Running hardware and repository diagnostics...
            </div>
          ) : (
            <div className="space-y-3">
              {/* Core Engine */}
              <div className="flex items-start gap-3 p-3 rounded-xl bg-[#131b2a] border border-[#1d293d]">
                <Cpu className="w-4 h-4 text-emerald-400 shrink-0 mt-0.5" />
                <div className="text-xs">
                  <span className="font-semibold text-slate-200 block">Core VCS Engine</span>
                  <span className="text-slate-400 font-mono text-[11px]">{report.core_version}</span>
                </div>
              </div>

              {/* Hashing Engine */}
              <div className="flex items-start gap-3 p-3 rounded-xl bg-[#131b2a] border border-[#1d293d]">
                <CheckCircle2 className="w-4 h-4 text-cyan-400 shrink-0 mt-0.5" />
                <div className="text-xs">
                  <span className="font-semibold text-slate-200 block">Hardware Accelerated Hashing</span>
                  <span className="text-slate-400 font-mono text-[11px]">{report.hashing_engine}</span>
                </div>
              </div>

              {/* Git Compatibility Layer */}
              <div className="flex items-start gap-3 p-3 rounded-xl bg-[#131b2a] border border-[#1d293d]">
                <Terminal className="w-4 h-4 text-blue-400 shrink-0 mt-0.5" />
                <div className="text-xs">
                  <span className="font-semibold text-slate-200 block">Compatibility Protocol Layer</span>
                  <span className="text-slate-400 font-mono text-[11px]">Git v{report.git_version}</span>
                </div>
              </div>

              {/* Author Identity */}
              <div className="flex items-start gap-3 p-3 rounded-xl bg-[#131b2a] border border-[#1d293d]">
                <User className="w-4 h-4 text-purple-400 shrink-0 mt-0.5" />
                <div className="text-xs">
                  <span className="font-semibold text-slate-200 block">Git Author Identity</span>
                  <span className="text-slate-400 font-mono text-[11px]">
                    {report.author_name ? `${report.author_name} <${report.author_email}>` : "Not configured"}
                  </span>
                </div>
              </div>

              {/* Remote Authentication */}
              <div className="flex items-start gap-3 p-3 rounded-xl bg-[#131b2a] border border-[#1d293d]">
                <ShieldCheck className="w-4 h-4 text-emerald-400 shrink-0 mt-0.5" />
                <div className="text-xs">
                  <span className="font-semibold text-slate-200 block">Authenticated Providers</span>
                  <span className="text-slate-400 font-mono text-[11px]">
                    {report.authenticated_hosts.length > 0
                      ? report.authenticated_hosts.join(", ")
                      : "No active accounts"}
                  </span>
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="p-4 bg-[#0a0f19] border-t border-[#1f2c42] flex justify-end">
          <button
            onClick={onClose}
            className="px-4 py-2 rounded-lg bg-[#192438] hover:bg-[#22314c] text-xs font-semibold text-slate-200 transition-colors"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};
