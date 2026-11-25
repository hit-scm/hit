use hit::core::hash;
use hit::core::index::Index;
use hit::core::object::{Commit, ObjectType, Tree, TreeEntry};
use hit::core::storage::ObjectStore;
use hit::error::HitError;
use hit::git::GitBackend;
use std::fs;
use std::sync::Mutex;
use tempfile::tempdir;

static CWD_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn test_blake3_hashing() {
    let data = b"hello hit version control";
    let h1 = hash::hash_bytes(data);
    let h2 = hash::hash_bytes(data);
    assert_eq!(h1, h2);
    assert_eq!(h1.len(), 64);
    assert_eq!(hash::short_hash(&h1).len(), 7);
}

#[test]
fn test_object_store_roundtrip() {
    let tmp = tempdir().unwrap();
    let repo_root = tmp.path();
    let hit_dir = repo_root.join(".hit");
    fs::create_dir_all(hit_dir.join("objects")).unwrap();

    let content = b"fn main() { println!(\"blazingly fast\"); }";
    let hash = ObjectStore::write_object(repo_root, ObjectType::Blob, content).unwrap();

    // Read by full hash
    let (obj_type, data) = ObjectStore::read_object(repo_root, &hash).unwrap();
    assert_eq!(obj_type, ObjectType::Blob);
    assert_eq!(&data, content);

    // Read by prefix
    let prefix = &hash[0..8];
    let (obj_type_pref, data_pref) = ObjectStore::read_object(repo_root, prefix).unwrap();
    assert_eq!(obj_type_pref, ObjectType::Blob);
    assert_eq!(&data_pref, content);
}

#[test]
fn test_tree_serialization() {
    let entry1 = TreeEntry::new(0o100644, ObjectType::Blob, "a1b2c3d4e5f6", "main.rs");
    let entry2 = TreeEntry::new(0o040000, ObjectType::Tree, "f6e5d4c3b2a1", "src");
    let tree = Tree::new(vec![entry1, entry2]);

    let bytes = tree.serialize();
    let deserialized = Tree::deserialize(&bytes).unwrap();
    assert_eq!(deserialized.entries.len(), 2);
}

#[test]
fn test_commit_serialization() {
    let commit = Commit::new(
        "treehash123",
        vec!["parent1".to_string()],
        "Linus",
        "linus@example.com",
        1700000000,
        "Initial commit\n\nFull release notes.",
    );

    let bytes = commit.serialize();
    let deserialized = Commit::deserialize(&bytes).unwrap();
    assert_eq!(deserialized.tree, "treehash123");
    assert_eq!(deserialized.parents, vec!["parent1".to_string()]);
    assert_eq!(deserialized.author_name, "Linus");
    assert_eq!(deserialized.author_email, "linus@example.com");
    assert_eq!(deserialized.timestamp, 1700000000);
    assert_eq!(
        deserialized.message,
        "Initial commit\n\nFull release notes."
    );
}

#[test]
fn test_index_save_and_load() {
    let tmp = tempdir().unwrap();
    let repo_root = tmp.path();
    fs::create_dir_all(repo_root.join(".hit")).unwrap();

    let mut index = Index::new();
    index.add(
        "src/lib.rs".to_string(),
        "hash1".to_string(),
        120,
        1000,
        0o100644,
    );
    index.add(
        "README.md".to_string(),
        "hash2".to_string(),
        450,
        1001,
        0o100644,
    );
    index.save(repo_root).unwrap();

    let loaded = Index::load(repo_root).unwrap();
    assert_eq!(loaded.entries.len(), 2);
    assert_eq!(loaded.entries.get("src/lib.rs").unwrap().hash, "hash1");
    assert_eq!(loaded.entries.get("README.md").unwrap().size, 450);
}

#[test]
fn test_error_formatting_with_hint() {
    let err = HitError::branch_not_found("featuer", Some("feature"));
    let rendered = err.render();
    assert!(rendered.contains("Branch 'featuer' does not exist"));
    assert!(rendered.contains("Did you mean 'feature'?"));
    assert!(rendered.contains("hint:"));
}

#[test]
fn test_end_to_end_git_compatible_repo_lifecycle() {
    let _lock = CWD_LOCK.lock().unwrap();
    let tmp = tempdir().unwrap();
    let repo_path = tmp.path().join("my-project");
    fs::create_dir_all(&repo_path).unwrap();

    // Switch cwd to repo_path for command execution
    let orig_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&repo_path).unwrap();

    // 1. Init
    hit::commands::init::execute(None, Some("main")).unwrap();
    assert!(GitBackend::is_in_repo());
    assert_eq!(GitBackend::current_branch(), Some("main".to_string()));

    // Ensure user identity is set for commit test
    let _ = GitBackend::run(&["config", "user.name", "Hit Tester"]);
    let _ = GitBackend::run(&["config", "user.email", "test@example.com"]);

    // 2. Add files
    let file1 = repo_path.join("hello.txt");
    fs::write(&file1, "Hello Hit Git Compatibility!").unwrap();

    hit::commands::add::execute(vec!["hello.txt".to_string()]).unwrap();

    // 3. Commit
    hit::commands::commit::execute(Some("Initial commit".to_string()), false, false).unwrap();

    // 4. Create & switch branch
    hit::commands::switch::execute(None, Some("feature/remotes".to_string()), false).unwrap();
    assert_eq!(
        GitBackend::current_branch(),
        Some("feature/remotes".to_string())
    );

    // 5. Test Remote commands
    hit::commands::remote::execute(
        Some("add".to_string()),
        vec![
            "origin".to_string(),
            "https://github.com/hit-scm/example.git".to_string(),
        ],
        false,
    )
    .unwrap();

    let remotes = GitBackend::list_remotes();
    assert_eq!(remotes.len(), 1);
    assert_eq!(remotes[0].0, "origin");
    assert_eq!(remotes[0].1, "https://github.com/hit-scm/example.git");

    // 6. Test Stash
    let file2 = repo_path.join("wip.txt");
    fs::write(&file2, "work in progress").unwrap();
    hit::commands::add::execute(vec!["wip.txt".to_string()]).unwrap();
    hit::commands::stash::execute(Some("push".to_string()), Some("WIP stash test".to_string()))
        .unwrap();

    // Restore original cwd
    std::env::set_current_dir(orig_dir).unwrap();
}

#[test]
fn test_parse_git_progress_variations() {
    use hit::ui::parse_git_progress;

    // Push writing objects
    let p1 =
        parse_git_progress("Writing objects: 100% (3/3), 223 bytes | 223.00 KiB/s, done.").unwrap();
    assert_eq!(p1.stage, "Writing objects");
    assert_eq!(p1.percent, Some(100));
    assert!(p1.detail.contains("3/3"));
    assert!(p1.detail.contains("223.00 KiB/s"));

    // Clone receiving objects
    let p2 = parse_git_progress("remote: Receiving objects:  75% (15/20), 1.20 MiB | 2.40 MiB/s")
        .unwrap();
    assert_eq!(p2.stage, "Receiving objects");
    assert_eq!(p2.percent, Some(75));
    assert!(p2.detail.contains("15/20"));
    assert!(p2.detail.contains("2.40 MiB/s"));

    // Counting objects
    let p3 = parse_git_progress("Counting objects:  40% (2/5)").unwrap();
    assert_eq!(p3.stage, "Counting objects");
    assert_eq!(p3.percent, Some(40));
    assert_eq!(p3.detail, "2/5");

    // Total line
    let p4 = parse_git_progress("Total 13 (delta 0), reused 0 (delta 0), pack-reused 13 (from 1)")
        .unwrap();
    assert_eq!(p4.stage, "Transferring objects");
    assert_eq!(p4.percent, Some(100));
    assert_eq!(p4.detail, "13 objects");

    // Non-progress line should be ignored
    assert!(parse_git_progress("* [new branch]      main -> main").is_none());
    assert!(parse_git_progress("To https://github.com/test.git").is_none());
}

#[test]
fn test_up_to_date_remote_detection() {
    let _lock = CWD_LOCK.lock().unwrap();
    let tmp = tempdir().unwrap();
    let repo_path = tmp.path().join("repo-test-remote");
    fs::create_dir_all(&repo_path).unwrap();

    let orig_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&repo_path).unwrap();

    hit::commands::init::execute(None, Some("main")).unwrap();
    let _ = GitBackend::run(&["config", "user.name", "Tester"]);
    let _ = GitBackend::run(&["config", "user.email", "t@t.com"]);

    let bare_remote = tmp.path().join("bare.git");
    let _ = std::process::Command::new("git")
        .args(["clone", "--bare", ".", bare_remote.to_str().unwrap()])
        .output();

    hit::commands::remote::execute(
        Some("add".to_string()),
        vec![
            "origin".to_string(),
            bare_remote.to_str().unwrap().to_string(),
        ],
        false,
    )
    .unwrap();

    fs::write(repo_path.join("file.txt"), "hello").unwrap();
    hit::commands::add::execute(vec!["file.txt".to_string()]).unwrap();
    hit::commands::commit::execute(Some("c1".to_string()), false, false).unwrap();

    // Push first time
    hit::commands::push::execute(
        Some("origin".to_string()),
        Some("main".to_string()),
        true,
        false,
        false,
    )
    .unwrap();

    // Verify it is detected as up to date
    assert!(GitBackend::is_up_to_date_with_remote("main", "origin"));

    // Push second time should trigger instant fast-path (sub-millisecond)
    let start = std::time::Instant::now();
    hit::commands::push::execute(
        Some("origin".to_string()),
        Some("main".to_string()),
        false,
        false,
        false,
    )
    .unwrap();
    let duration = start.elapsed();
    assert!(
        duration.as_millis() < 50,
        "Fast-path push took too long: {:?}",
        duration
    );

    // Pull should also trigger instant fast-path (sub-millisecond, no download)
    let start_pull = std::time::Instant::now();
    hit::commands::pull::execute(
        Some("origin".to_string()),
        Some("main".to_string()),
        false,
        false,
    )
    .unwrap();
    let duration_pull = start_pull.elapsed();
    assert!(
        duration_pull.as_millis() < 50,
        "Fast-path pull took too long: {:?}",
        duration_pull
    );

    // Sync should also trigger instant fast-path (sub-millisecond, no download)
    let start_sync = std::time::Instant::now();
    hit::commands::sync::execute(Some("origin".to_string())).unwrap();
    let duration_sync = start_sync.elapsed();
    assert!(
        duration_sync.as_millis() < 50,
        "Fast-path sync took too long: {:?}",
        duration_sync
    );

    std::env::set_current_dir(orig_dir).unwrap();
}

#[test]
fn test_submodule_management() {
    let _lock = CWD_LOCK.lock().unwrap();
    let tmp = tempdir().unwrap();

    let child_path = tmp.path().join("child-repo");
    fs::create_dir_all(&child_path).unwrap();

    let parent_path = tmp.path().join("parent-repo");
    fs::create_dir_all(&parent_path).unwrap();

    let orig_dir = std::env::current_dir().unwrap();

    // 1. Setup child repository
    std::env::set_current_dir(&child_path).unwrap();
    hit::commands::init::execute(None, Some("main")).unwrap();
    let _ = GitBackend::run(&["config", "user.name", "Child Tester"]);
    let _ = GitBackend::run(&["config", "user.email", "child@example.com"]);
    fs::write(child_path.join("lib.rs"), "pub fn child() {}").unwrap();
    hit::commands::add::execute(vec!["lib.rs".to_string()]).unwrap();
    hit::commands::commit::execute(Some("feat: initial child lib".to_string()), false, false)
        .unwrap();

    // 2. Setup parent repository
    std::env::set_current_dir(&parent_path).unwrap();
    hit::commands::init::execute(None, Some("main")).unwrap();
    let _ = GitBackend::run(&["config", "user.name", "Parent Tester"]);
    let _ = GitBackend::run(&["config", "user.email", "parent@example.com"]);
    fs::write(parent_path.join("README.md"), "# Parent Repo").unwrap();
    hit::commands::add::execute(vec!["README.md".to_string()]).unwrap();
    hit::commands::commit::execute(
        Some("feat: initial parent commit".to_string()),
        false,
        false,
    )
    .unwrap();

    // 3. Add submodule
    let child_url = child_path.to_str().unwrap().to_string();
    hit::commands::submodule::execute(
        Some("add".to_string()),
        vec![child_url, "vendor/child".to_string()],
        false,
        false,
        false,
        None,
    )
    .unwrap();

    // 4. Verify .gitmodules parsing
    let modules = hit::commands::submodule::parse_gitmodules();
    assert_eq!(modules.len(), 1);
    let sub = modules.get("vendor/child").unwrap();
    assert_eq!(sub.path, "vendor/child");

    // 5. Test submodule status
    hit::commands::submodule::execute(
        Some("status".to_string()),
        vec![],
        false,
        false,
        false,
        None,
    )
    .unwrap();

    // 6. Test submodule sync
    hit::commands::submodule::execute(Some("sync".to_string()), vec![], false, false, false, None)
        .unwrap();

    std::env::set_current_dir(orig_dir).unwrap();
}

#[test]
fn test_auth_subsystem_and_providers() {
    let _lock = CWD_LOCK.lock().unwrap();
    use hit::auth::{AuthState, GitProvider};

    // 1. Test Provider detection
    let gh = GitProvider::from_host("https://github.com");
    assert_eq!(gh, GitProvider::GitHub);
    assert!(gh.supports_device_flow());
    assert!(gh.token_creation_url().contains("description=Hit+CLI"));

    let gl = GitProvider::from_host("gitlab.com");
    assert_eq!(gl, GitProvider::GitLab("gitlab.com".to_string()));
    assert!(gl.token_creation_url().contains("name=Hit+CLI"));
    assert!(gl.token_creation_url().contains("scopes=api"));

    let cb = GitProvider::from_host("codeberg.org");
    assert_eq!(cb, GitProvider::Codeberg("codeberg.org".to_string()));
    assert!(cb
        .token_creation_url()
        .contains("/user/settings/applications"));

    let custom = GitProvider::from_host("git.internal.acme.com");
    assert_eq!(
        custom,
        GitProvider::Custom("git.internal.acme.com".to_string())
    );

    // 2. Test AuthState persistence in isolated directory
    let tmp = tempdir().unwrap();
    let hit_dir = tmp.path().join(".hit");
    std::env::set_var("HIT_DIR", &hit_dir);

    let mut state = AuthState::default();

    let account1 = AuthState::new_account(
        "alice".to_string(),
        Some("Alice Smith".to_string()),
        Some("alice@example.com".to_string()),
        "ghp_test_token_123".to_string(),
        "https".to_string(),
        vec!["repo".to_string(), "read:org".to_string()],
    );
    state.set_account("github.com", account1);

    let account2 = AuthState::new_account(
        "alice_work".to_string(),
        Some("Alice Work".to_string()),
        Some("alice@work.com".to_string()),
        "ghp_work_token_456".to_string(),
        "https".to_string(),
        vec!["repo".to_string()],
    );
    state.set_account("github.com", account2);

    assert_eq!(
        state.get_active_token("github.com"),
        Some("ghp_work_token_456".to_string())
    );

    // Switch active user back to alice
    assert!(state.switch_active_user("github.com", "alice"));
    assert_eq!(
        state.get_active_token("github.com"),
        Some("ghp_test_token_123".to_string())
    );

    // Save and reload
    state.save().unwrap();
    let loaded = AuthState::load();
    assert_eq!(
        loaded.get_active_token("github.com"),
        Some("ghp_test_token_123".to_string())
    );

    let host_auth = loaded.hosts.get("github.com").unwrap();
    assert_eq!(host_auth.users.len(), 2);
    assert_eq!(host_auth.active_user, "alice");

    // Remove user
    let mut modified = loaded;
    assert!(modified.remove_user("github.com", "alice"));
    assert_eq!(
        modified.get_active_token("github.com"),
        Some("ghp_work_token_456".to_string())
    );

    std::env::remove_var("HIT_DIR");
}

#[test]
fn test_credential_resolver_with_auth_state() {
    let _lock = CWD_LOCK.lock().unwrap();
    use hit::auth::AuthState;
    use hit::core::credentials::CredentialResolver;

    let tmp = tempdir().unwrap();
    let hit_dir = tmp.path().join(".hit");
    std::env::set_var("HIT_DIR", &hit_dir);

    let mut state = AuthState::default();
    let account = AuthState::new_account(
        "bob".to_string(),
        Some("Bob".to_string()),
        Some("bob@gitlab.com".to_string()),
        "glpat_secure_token_789".to_string(),
        "https".to_string(),
        vec!["api".to_string()],
    );
    state.set_account("gitlab.com", account);
    state.save().unwrap();

    // Verify CredentialResolver resolves token
    assert_eq!(
        CredentialResolver::get_token_for_host("https://gitlab.com"),
        Some("glpat_secure_token_789".to_string())
    );

    let header = CredentialResolver::get_auth_header_for_host("gitlab.com").unwrap();
    assert_eq!(header, "Bearer glpat_secure_token_789");

    std::env::remove_var("HIT_DIR");
}
