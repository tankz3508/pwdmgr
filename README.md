# pwdmgr
NOTE: This is an early project, to gain some knowledge. Feel free to give me feedback.

`pwdmgr` is a simple command-line password manager written in Rust.

---

## Features
- Store website, username, password to a file via command.
- Read website, username and password from file
- Flexible file_path config (changeable via `pwdmgr -f`/`pwdmgr --file_path`)
- Lightweight, fast.

---

## Installation
1. Download the latest release zip from the [Releases page](https://github.com/tankz3508/pwdmgr/releases).
2. Extract the zip anywhere you want (e.g., `C:\pwdmgr`).
3. Add the folder to your PATH. (Win + S -> Search "env" -> open "Edit the System Enviroment Variables" -> on the Bottom click "Enviroment Variables" -> click PATH -> "New" -> "New" -> Add the file to your path)

---

## Usage
```
pwdmgr -a https://google.com JohnDoe JohnDoe
pwdmgr -l
pwdmgr -f example.json (DEFAULTS TO: C:\Users\<USERNAME>\)
```
For more information run: `pwdmgr -h`

##⚠️ DISCLAIMER ⚠️
Windows might flag the pwdmgr.exe on execution because it reads and writes .json files. This is nothing serious. You can exculde the pwdmgr.exe in your Antivirus settings.
