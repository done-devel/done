<div align="center">
  <br>
  <img src="https://raw.githubusercontent.com/edfloreshz/done/4c8632c3ed21ecf2be72c2f0b1b2dfab428fe15f/data/icons/hicolor/scalable/apps/dev.edfloreshz.Done.svg" width="150" />
  <h1>Done</h1>
  <h3>To-do lists reimagined</h3>
  <a href="https://github.com/edfloreshz/done/actions/workflows/rust.yml">
    <img src="https://img.shields.io/github/workflow/status/edfloreshz/sensei/Rust?logo=GitHub" alt="build"/>
  </a>
  <a href="https://crates.io/crates/done">
    <img src="https://img.shields.io/crates/v/done?label=Done" alt="crate"/>
  </a>
   <a href="https://crates.io/crates/done">
    <img src="https://img.shields.io/crates/d/done" alt="downloads"/>
  </a>
  <br/>
  <a href="https://github.com/sponsors/edfloreshz">
    <img src="https://img.shields.io/badge/sponsor-30363D?style=for-the-badge&logo=GitHub-Sponsors&logoColor=#white"/>
  </a>
  <a href="https://matrix.to/#/#done-gh:matrix.org">
    <img src="https://img.shields.io/badge/matrix-000000?style=for-the-badge&logo=Matrix&logoColor=white"/>
  </a>
  <a href="https://github.com/edfloreshz/done">
    <img src="https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white"/>
  </a>
  <a href="https://t.me/done_gh">
    <img src="https://img.shields.io/badge/Telegram-2CA5E0?style=for-the-badge&logo=telegram&logoColor=white"/>
  </a>
</div>
<br/>

Done is a simple to do app that lets you combine your existing set of task providers into one database, easily.

<div align="center">
  <img src="https://raw.githubusercontent.com/edfloreshz/done/81ea1f6d32cd491d1893f9ba730f511bc1cb0aea/data/resources/screenshots/tasks.png"/>
</div>


## Install
| Platform   | Command                                 |
|------------|-----------------------------------------|
| Arch Linux | `paru -S done-git`                    |
| Flathub    | <a href="https://flathub.org/apps/details/dev.edfloreshz.Done"><img src="https://flathub.org/assets/badges/flathub-badge-en.png" width="150"/></a> |



## To do

### Accounts

- [ ] Allow multiple providers (Google, Microsoft To Do, Microsoft Exchange, Todoist, Nextcloud)

### Lists

- [x] Show lists
- [x] Add a new list
- [ ] Delete an existing list
- [ ] Rename an existing list
- [x] Update task counters

### Smart Lists
- [ ] Inbox
- [ ] Today
- [ ] Next 7 Days
- [x] All
- [x] Starred
- [ ] Archived

### Tasks
- [x] Add a new task
- [x] Show tasks for every list
- [x] Mark a task as completed
- [ ] Delete a task
- [ ] Rename a task
- [ ] Add steps
- [ ] Add tags
- [ ] Add to My Day
- [x] Mark as Favorite
- [ ] Add notes

### Reminders
- [ ] Set a reminder
- [ ] Set a due date
- [ ] Set recurrence for a task

### Backups
- [ ] Export tasks

## Dependencies to build
- gtk4
- libadwaita
- pkg-config

Ubuntu 22.04:
```bash
sudo apt install libadwaita-1-dev libgtk-4-dev libsqlite3-dev
```
Arch Linux:
```bash
sudo pacman -S libadwaita gtk4 sqlite
```

Copyright and licensing
-----------------------

Copyright 2022 © Eduardo Flores

Done is released under the terms of the [GNU General Public License version 2.0.](https://github.com/edfloreshz/done/blob/main/LICENSE)
