import subprocess
import ast
import json
import os
from collections import Counter, defaultdict
from datetime import datetime

def execute_shell_command(cmd):
    """
    Executes a shell command and returns its output as a string.
    """
    result = subprocess.check_output(cmd, shell=True, text=True)
    return result

def get_git_history_analysis():
    """
    Returns a string with the git log graph for the current repository.
    """
    return execute_shell_command("git log --oneline --graph")

def get_commit_history(repo_path, file_path):
    """
    Retrieves the commit history for a specific file.
    Returns a list of (commit_hash, code_content, commit_date) tuples.
    """
    # Get commit hashes and dates for the file
    cmd = [
        "git", "-C", repo_path, "log", "--pretty=format:%H|%cI", "--", file_path
    ]
    commit_lines = subprocess.check_output(cmd, text=True).splitlines()
    print(f"Found {len(commit_lines)} commits for {file_path}")
    history = []
    for line in commit_lines:
        if "|" not in line:
            continue
        commit, date = line.split("|", 1)
        show_cmd = [
            "git", "-C", repo_path, "show", f"{commit}:{file_path}"
        ]
        try:
            code = subprocess.check_output(show_cmd, text=True, stderr=subprocess.DEVNULL)
            history.append((commit, code, date))
        except subprocess.CalledProcessError:
            print(f"File {file_path} not found in commit {commit}")
            continue  # File may not exist in this commit
    return history

def analyze_code_semantics(code):
    """
    Parses Python code and returns semantic features:
    - Number of functions
    - Number of classes
    - Number of imports
    - Number of lines
    """
    try:
        tree = ast.parse(code)
    except Exception:
        return None
    features = {
        "functions": 0,
        "classes": 0,
        "imports": 0,
        "lines": len(code.splitlines())
    }
    for node in ast.walk(tree):
        if isinstance(node, ast.FunctionDef):
            features["functions"] += 1
        elif isinstance(node, ast.ClassDef):
            features["classes"] += 1
        elif isinstance(node, (ast.Import, ast.ImportFrom)):
            features["imports"] += 1
    return features

def analyze_code_evolution(repo_path, file_path):
    """
    Analyzes code evolution trends for a file in a git repo.
    Returns a list of semantic feature dicts per commit (oldest to newest).
    """
    history = get_commit_history(repo_path, file_path)
    trends = []
    for commit, code, date in reversed(history):  # oldest to newest
        features = analyze_code_semantics(code)
        if features:
            features["commit"] = commit
            features["date"] = date
            trends.append(features)
    return trends

def save_git_log_database(repo_path, file_path, db_path="git_log_db.json"):
    """
    Creates a git logging database (JSON) with semantic features per commit.
    """
    trends = analyze_code_evolution(repo_path, file_path)
    git_graph = get_git_history_analysis()
    db = {
        "repo_path": repo_path,
        "file_path": file_path,
        "generated_at": datetime.utcnow().isoformat() + "Z",
        "history": trends,
        "git_history_graph": git_graph
    }
    with open(db_path, "w", encoding="utf-8") as f:
        json.dump(db, f, indent=2)
    print(f"Git logging database saved to {db_path}")

def generate_recursive_workspace_index(root_dir=".", index_path="workspace_index.json"):
    file_list = []
    for dirpath, _, filenames in os.walk(root_dir):
        for fname in filenames:
            fpath = os.path.join(dirpath, fname)
            try:
                stat = os.stat(fpath)
                file_list.append({
                    "name": os.path.relpath(fpath, root_dir),
                    "size": stat.st_size,
                    "last_modified": datetime.fromtimestamp(stat.st_mtime).isoformat()
                })
            except Exception as e:
                print(f"Error reading {fpath}: {e}")
    with open(index_path, "w", encoding="utf-8") as f:
        json.dump({"files": file_list}, f, indent=2)
    print(f"Recursive workspace index saved to {index_path}")

def generate_recursive_workspace_index_with_summaries(root_dir=".", index_path="workspace_index.json"):
    file_list = []
    for dirpath, _, filenames in os.walk(root_dir):
        for fname in filenames:
            fpath = os.path.join(dirpath, fname)
            entry = {
                "name": os.path.relpath(fpath, root_dir),
                "size": None,
                "last_modified": None,
                "summary": None
            }
            try:
                stat = os.stat(fpath)
                entry["size"] = stat.st_size
                entry["last_modified"] = datetime.fromtimestamp(stat.st_mtime).isoformat()
                if fname.endswith(".py"):
                    entry["summary"] = summarize_python_file(fpath)
            except Exception as e:
                entry["error"] = str(e)
            file_list.append(entry)
    with open(index_path, "w", encoding="utf-8") as f:
        json.dump({"files": file_list}, f, indent=2)
    print(f"Recursive workspace index with summaries saved to {index_path}")

def summarize_python_file(filepath):
    import ast
    try:
        with open(filepath, "r", encoding="utf-8") as f:
            code = f.read()
        tree = ast.parse(code)
        return {
            "functions": sum(isinstance(n, ast.FunctionDef) for n in ast.walk(tree)),
            "classes": sum(isinstance(n, ast.ClassDef) for n in ast.walk(tree)),
            "imports": sum(isinstance(n, (ast.Import, ast.ImportFrom)) for n in ast.walk(tree)),
            "lines": len(code.splitlines())
        }
    except Exception as e:
        return {"error": str(e)}

def generate_codebot_manifest(index_path="workspace_index.json", manifest_path="codebot.json"):
    """
    Generates a codebot.json manifest describing the project for AI tools.
    """
    import platform
    try:
        with open(index_path, "r", encoding="utf-8") as f:
            index = json.load(f)
    except Exception as e:
        index = {"files": [], "error": str(e)}
    manifest = {
        "project_name": "Rust-Macros",
        "description": "AI-ready Rust and Python macro playground with workspace indexing and semantic introspection.",
        "language": ["Rust", "Python"],
        "platform": platform.system(),
        "main_files": ["macros.rs", "main.py"],
        "index_file": index_path,
        "files": index.get("files", []),
        "generated_at": datetime.now().isoformat()
    }
    with open(manifest_path, "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2)
    print(f"Codebot manifest saved to {manifest_path}")

if __name__ == "__main__":
    repo_path = r"C:\Users\jesus\projects\macros"
    file_path = "macros.rs"
    save_git_log_database(repo_path, file_path)
    print("\nGit history graph:")
    print(get_git_history_analysis())
