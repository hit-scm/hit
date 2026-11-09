import {
  RepoInfo,
  RepoStatus,
  CommitItem,
  SubmoduleItem,
  DoctorReport,
  AuthState,
  AccountAuth,
} from "./types";

// Check if running inside native Tauri runtime
const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

async function tauriInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (isTauri) {
    const { invoke } = await import("@tauri-apps/api/core");
    return invoke<T>(cmd, args);
  }
  return mockInvoke<T>(cmd, args);
}

// Mock fallback for browser-based UI development and preview
const mockRepo: RepoInfo = {
  path: "/Users/developer/projects/hit",
  name: "hit",
  current_branch: "master",
  remotes: [["origin", "https://github.com/hit-scm/hit.git"]],
  is_clean: false,
  ahead: 2,
  behind: 0,
};

let mockStaged: RepoStatus["staged"] = [
  { path: "src/auth/provider.rs", status: "M", is_staged: true },
  { path: "src/commands/auth.rs", status: "A", is_staged: true },
];

let mockUnstaged: RepoStatus["unstaged"] = [
  { path: "README.md", status: "M", is_staged: false },
  { path: ".github/workflows/release.yml", status: "M", is_staged: false },
];

let mockUntracked: RepoStatus["untracked"] = [
  { path: "desktop/package.json", status: "?", is_staged: false },
  { path: "desktop/src/App.tsx", status: "?", is_staged: false },
];

let mockBranches = ["master", "main", "feature/desktop-ui", "refactor/simd-hash"];

const mockHistory: CommitItem[] = [
  {
    sha: "65180c4a9238e819bce410294b219c018274b819",
    short_sha: "65180c4",
    message: "feat(auth): introduce guided multi-provider authentication and credential helper sync",
    author_name: "ByJonas",
    author_email: "me@byjonas.dev",
    date: "10 minutes ago",
  },
  {
    sha: "905002ce761928bc917402847192847192847192",
    short_sha: "905002c",
    message: "docs: provide technical overview, performance benchmarks, and automated install scripts",
    author_name: "ByJonas",
    author_email: "me@byjonas.dev",
    date: "45 minutes ago",
  },
  {
    sha: "7c5b9d7a81928471928471928471928471928471",
    short_sha: "7c5b9d7",
    message: "feat(submodule): introduce submodule management with live progress and sync",
    author_name: "ByJonas",
    author_email: "me@byjonas.dev",
    date: "1 hour ago",
  },
  {
    sha: "143d979b81928471928471928471928471928471",
    short_sha: "143d979",
    message: "feat(remote): add accelerated clone, push, pull, and single-step sync with ref fast-paths",
    author_name: "ByJonas",
    author_email: "me@byjonas.dev",
    date: "2 hours ago",
  },
  {
    sha: "ceb89fda81928471928471928471928471928471",
    short_sha: "ceb89fd",
    message: "feat(commands): implement core repository commands, status, diff, log, and safe undo",
    author_name: "ByJonas",
    author_email: "me@byjonas.dev",
    date: "3 hours ago",
  },
];

async function mockInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  // Artificial latency for realism (30-80ms)
  await new Promise((r) => setTimeout(r, 40));

  switch (cmd) {
    case "open_repository":
      return mockRepo as unknown as T;

    case "get_status":
      return {
        branch: mockRepo.current_branch,
        ahead: mockRepo.ahead,
        behind: mockRepo.behind,
        staged: mockStaged,
        unstaged: mockUnstaged,
        untracked: mockUntracked,
        is_clean: mockStaged.length === 0 && mockUnstaged.length === 0 && mockUntracked.length === 0,
      } as unknown as T;

    case "get_file_diff": {
      const file = (args?.file as string) || "file.rs";
      return `diff --git a/${file} b/${file}
index 81b2a9..9fc01e 100644
--- a/${file}
+++ b/${file}
@@ -14,6 +14,14 @@ impl HitEngine {
     pub fn execute_flow(&self) -> HitResult<()> {
-        println!("Legacy execution path");
+        // High-performance bare-metal execution path
+        let timer = Instant::now();
+        let result = self.run_optimized()?;
+        Ui::success_with_time("Operation finished", timer.elapsed());
+        Ok(result)
     }
+
+    pub fn is_fast_path_ready(&self) -> bool {
+        true
+    }
 }` as unknown as T;
    }

    case "stage_file": {
      const f = args?.file as string;
      mockUnstaged = mockUnstaged.filter((x) => x.path !== f);
      mockUntracked = mockUntracked.filter((x) => x.path !== f);
      mockStaged.push({ path: f, status: "M", is_staged: true });
      return undefined as unknown as T;
    }

    case "stage_all": {
      mockStaged = [...mockStaged, ...mockUnstaged, ...mockUntracked].map((x) => ({
        ...x,
        is_staged: true,
      }));
      mockUnstaged = [];
      mockUntracked = [];
      return undefined as unknown as T;
    }

    case "unstage_file": {
      const f = args?.file as string;
      mockStaged = mockStaged.filter((x) => x.path !== f);
      mockUnstaged.push({ path: f, status: "M", is_staged: false });
      return undefined as unknown as T;
    }

    case "unstage_all": {
      mockUnstaged = [...mockUnstaged, ...mockStaged].map((x) => ({
        ...x,
        is_staged: false,
      }));
      mockStaged = [];
      return undefined as unknown as T;
    }

    case "commit": {
      const msg = args?.message as string;
      mockStaged = [];
      mockRepo.ahead += 1;
      return "0cd0d5b" as unknown as T;
    }

    case "sync_repo":
      mockRepo.ahead = 0;
      mockRepo.behind = 0;
      return undefined as unknown as T;

    case "push_repo":
      mockRepo.ahead = 0;
      return undefined as unknown as T;

    case "pull_repo":
      mockRepo.behind = 0;
      return undefined as unknown as T;

    case "get_history":
      return mockHistory as unknown as T;

    case "get_branches":
      return [mockRepo.current_branch, mockBranches] as unknown as T;

    case "switch_branch": {
      const b = args?.branch as string;
      mockRepo.current_branch = b;
      if (!mockBranches.includes(b)) mockBranches.push(b);
      return undefined as unknown as T;
    }

    case "get_submodules":
      return [
        {
          name: "vendor/libblake3",
          path: "vendor/libblake3",
          url: "https://github.com/hit-scm/libblake3.git",
          branch: "main",
          status: "clean",
        },
        {
          name: "plugins/theme-pack",
          path: "plugins/theme-pack",
          url: "https://github.com/hit-scm/theme-pack.git",
          branch: "v1.0.0",
          status: "clean",
        },
      ] as unknown as T;

    case "get_auth_state":
      return {
        hosts: {
          "github.com": {
            active_user: "builtbyjonas",
            users: {
              builtbyjonas: {
                username: "builtbyjonas",
                name: "Jonas",
                email: "me@byjonas.dev",
                token: "ghp_mock_token_12345678",
                git_protocol: "https",
                scopes: ["repo", "read:org", "workflow"],
                authenticated_at: "2026-11-09T14:48:00Z",
              },
            },
          },
        },
      } as unknown as T;

    case "run_doctor":
      return {
        core_version: "hit v0.1.0 (protocol-v2, blake3-simd)",
        git_version: "git v2.54.0 (active)",
        hashing_engine: "BLAKE3 - Hardware accelerated (ARM NEON SIMD)",
        global_config: true,
        authenticated_hosts: ["github.com (builtbyjonas)"],
        author_name: "ByJonas",
        author_email: "me@byjonas.dev",
      } as unknown as T;

    case "open_browser":
      if (typeof window !== "undefined") {
        window.open(args?.url as string, "_blank");
      }
      return undefined as unknown as T;

    default:
      return undefined as unknown as T;
  }
}

export const api = {
  openRepository: (path: string) => tauriInvoke<RepoInfo>("open_repository", { path }),
  getStatus: (path: string) => tauriInvoke<RepoStatus>("get_status", { path }),
  getFileDiff: (path: string, file: string, staged: boolean) =>
    tauriInvoke<string>("get_file_diff", { path, file, staged }),
  stageFile: (path: string, file: string) => tauriInvoke<void>("stage_file", { path, file }),
  stageAll: (path: string) => tauriInvoke<void>("stage_all", { path }),
  unstageFile: (path: string, file: string) => tauriInvoke<void>("unstage_file", { path, file }),
  unstageAll: (path: string) => tauriInvoke<void>("unstage_all", { path }),
  commit: (path: string, message: string) => tauriInvoke<string>("commit", { path, message }),
  syncRepo: (path: string) => tauriInvoke<void>("sync_repo", { path }),
  pushRepo: (path: string) => tauriInvoke<void>("push_repo", { path }),
  pullRepo: (path: string) => tauriInvoke<void>("pull_repo", { path }),
  fetchRepo: (path: string) => tauriInvoke<void>("fetch_repo", { path }),
  getHistory: (path: string, limit = 50) =>
    tauriInvoke<CommitItem[]>("get_history", { path, limit }),
  getBranches: (path: string) => tauriInvoke<[string, string[]]>("get_branches", { path }),
  switchBranch: (path: string, branch: string, create = false) =>
    tauriInvoke<void>("switch_branch", { path, branch, create }),
  getSubmodules: (path: string) => tauriInvoke<SubmoduleItem[]>("get_submodules", { path }),
  updateSubmodules: (path: string) => tauriInvoke<void>("update_submodules", { path }),
  getAuthState: () => tauriInvoke<AuthState>("get_auth_state"),
  loginAccount: (host: string, token: string) =>
    tauriInvoke<AccountAuth>("login_account", { host, token }),
  logoutAccount: (host: string, user?: string) =>
    tauriInvoke<void>("logout_account", { host, user }),
  runDoctor: () => tauriInvoke<DoctorReport>("run_doctor"),
  openBrowser: (url: string) => tauriInvoke<void>("open_browser", { url }),
};
