# Work Notes Management System Design

**Date:** 2026-01-16
**Purpose:** Fast-capture note system integrated with Shortcut project management

## Problem Statement

Current notes setup has multiple pain points:
- **Context loss**: Notes lack structured connection to work context (tickets, iterations, epics)
- **Disorganization**: Hard to browse and find notes related to specific work items
- **Scattered information**: No unified view of tickets, notes, and tmux sessions

Primary use cases:
- Maintaining context-rich notes for each ticket, epic, or iteration
- Browsing work history and notes in an organized interface
- Quickly accessing ticket-related information and notes

Primary environment: TUI application with Obsidian-compatible markdown editing in nvim.

## Solution Overview

Build an integrated TUI workspace manager that tracks Shortcut tickets, manages tmux sessions, and maintains one note per work item (ticket/epic/iteration). Notes are Obsidian-compatible markdown files that open automatically when selecting work items in the TUI.

**Core principle**: Unified workspace where every work item has exactly one note, eliminating the need to hunt for or organize multiple related notes.

## System Architecture

### Component Diagram

```
┌──────────────┐      ┌─────────────┐
│  State File  │◀─────│  TUI App    │
│  (JSON/TOML) │      │             │
└──────────────┘      └─────────────┘
                             │
                             ▼
                      ┌──────────────┐
                      │ Notes Index  │
                      │  (SQLite)    │
                      └──────────────┘
                             │
                             ▼
                      ┌──────────────┐
                      │  Markdown    │
                      │   Files      │
                      └──────────────┘
                             ▲
                             │
                      ┌──────────────┐
                      │  $EDITOR     │
                      │   (nvim)     │
                      └──────────────┘

         ┌──────────────┐
         │  Shortcut    │
         │     API      │
         └──────────────┘
                ▲
                │
         ┌──────────────┐
         │  TUI App     │
         │  (API sync)  │
         └──────────────┘
```

### Components

**1. TUI Application**
Central workspace manager with three main views:
- **Tickets view**: Browse Shortcut stories, select ticket to open/create its note, manage tmux sessions
- **Epics view**: Browse epics, select epic to open/create its note
- **Iterations view**: Browse iterations, select iteration to open/create its note
- **Notes browser**: Filter/search all notes by entity type, date, tags. Opens notes in $EDITOR

**2. State File** (`~/.config/worknotes/state.json`)
Application configuration and state:
- Shortcut workspace/API token
- Ticket → tmux session mapping
- Last sync timestamp
- Notes directory path
- Editor command ($EDITOR override)

**3. Notes Index** (SQLite)
Fast searchable index of all notes:
- Schema: `notes(id, entity_type, entity_id, path, created, updated, title, content_preview, tags)`
- Indexed on: entity_type, entity_id, created, updated
- Full-text search on title + content_preview
- Rebuilt on TUI startup

**4. Markdown Files**
Obsidian-compatible notes with simple frontmatter:
- Storage: `notes/tickets/sc-12345-slug.md`, `notes/epics/sc-500-slug.md`, `notes/iterations/iter-25-slug.md`
- Format: Standard markdown with YAML frontmatter
- Compatible with existing Obsidian nvim setup
- One note per entity (1:1 relationship)

## Data Model

### Note Frontmatter

**For Ticket Notes:**
```yaml
---
entity_type: ticket
entity_id: sc-12345
created: 2026-01-16T15:30:42Z
updated: 2026-01-16T16:45:12Z
title: "Auth Bug Fix"
epic: sc-500
iteration: 25
tags: [auth, bug, backend]
---
```

**For Epic Notes:**
```yaml
---
entity_type: epic
entity_id: sc-500
created: 2026-01-16T10:00:00Z
updated: 2026-01-20T09:30:00Z
title: "Authentication Refactor"
tags: [auth, epic-planning]
---
```

**For Iteration Notes:**
```yaml
---
entity_type: iteration
entity_id: 25
created: 2026-01-13T09:00:00Z
updated: 2026-01-26T17:00:00Z
title: "Iteration 25"
tags: [sprint-planning, retrospective]
---
```

**Field Descriptions:**
- `entity_type`: Type of work item (ticket, epic, or iteration)
- `entity_id`: Shortcut ID or iteration number
- `created`: ISO timestamp when note was first created
- `updated`: ISO timestamp of last modification
- `title`: Human-readable title (pulled from Shortcut or user-defined)
- `epic`: Parent epic ID (for ticket notes only)
- `iteration`: Iteration number (for ticket notes only)
- `tags`: Free-form tags for additional organization

### File Storage

Entity-based directory structure with 1:1 note relationship:
```
notes/
  tickets/
    sc-12345-auth-bug-fix.md
    sc-12346-api-refactor.md
    sc-12347-ui-improvements.md
  epics/
    sc-500-authentication-refactor.md
    sc-501-performance-optimization.md
  iterations/
    iter-25-sprint-planning.md
    iter-26-sprint-planning.md
```

Benefits:
- Clear 1:1 relationship between entity and note
- Predictable file paths (entity type + ID + slug)
- Easy to navigate by work item type
- Obsidian handles this structure fine
- No duplicate or orphaned notes

## Workflows

### Creating/Opening a Ticket Note

1. Launch TUI application
2. Browse tickets in tickets view
3. Select a ticket (press `Enter` or navigate to it)
4. TUI automatically:
   - Checks if note exists at `notes/tickets/sc-12345-slug.md`
   - If exists: opens the note in $EDITOR
   - If not: creates note with frontmatter (entity_type, entity_id, title from Shortcut API, epic, iteration)
   - Opens the newly created note in $EDITOR
5. User edits note using Obsidian nvim features (wikilinks, etc.)
6. Save and exit, return to TUI
7. TUI index is rebuilt on next startup to reflect changes

### Creating/Opening an Epic Note

1. In TUI, switch to epics view
2. Browse available epics
3. Select an epic (press `Enter`)
4. TUI automatically creates/opens `notes/epics/sc-500-slug.md`
5. User edits note in $EDITOR
6. Save and exit, return to TUI

### Creating/Opening an Iteration Note

1. In TUI, switch to iterations view
2. Browse iterations (current, past, upcoming)
3. Select an iteration (press `Enter`)
4. TUI automatically creates/opens `notes/iterations/iter-25-slug.md`
5. User edits note in $EDITOR
6. Save and exit, return to TUI

### Browsing and Searching Notes

1. In TUI, switch to notes browser view
2. Press `/` to enter filter mode
3. Type filter: `entity:ticket` or `entity:epic` or search by tags
4. Browse filtered list chronologically
5. Press `Enter` on note to open in $EDITOR
6. Wikilinks work as normal in Obsidian nvim

## Shortcut Integration

**One-way sync**: Shortcut → TUI/Notes (read-only)

**What gets synced:**
- Stories (tickets) for current iteration
- Epic associations for stories
- Iteration numbers
- Story metadata (title, status, owner, etc.)

**What doesn't sync:**
- Notes don't write back to Shortcut
- No automatic comment creation
- Manual process if you want to link notes in Shortcut

**API Usage:**
- TUI fetches stories on startup or manual refresh
- When setting active ticket, TUI queries API for epic + iteration
- Cached locally in state file to minimize API calls
- Auto-populate metadata when creating notes

## Benefits

**Eliminates note organization complexity:**
- One note per work item (no duplicate or scattered notes)
- Predictable file locations (always know where to find ticket/epic/iteration notes)
- Automatic note creation on selection (no manual file creation)
- Clear entity-based directory structure

**Rich browsing and search:**
- Filter by entity type (tickets, epics, iterations)
- Full-text search across all notes
- Multiple views of work items and their associated notes
- Chronological browsing within entity types

**Obsidian compatibility:**
- Existing nvim setup continues to work
- Wikilinks function normally
- Can use Obsidian graph view, backlinks, etc.
- Just adds structured metadata, doesn't break anything

**Integrated workspace:**
- Tickets, notes, and tmux sessions in one interface
- Tmux sessions tied to tickets
- Work context always visible
- Seamless navigation between work items and their notes

## Implementation Considerations

**Technology choices** (TBD based on existing TUI):
- TUI framework: (what are you using currently?)
- CLI: Shell script or compiled binary
- State format: JSON or TOML
- Index: SQLite for robustness

**Future enhancements** (not initial scope):
- Note templates per type
- Review workflows (inbox processing)
- Daily/weekly note aggregation
- Export to different formats
- Mobile capture integration
- Bi-directional Shortcut sync

**Out of scope:**
- Automatic alias generation
- Rich markdown rendering in TUI (use $EDITOR)
- Wikilink parsing in TUI (Obsidian nvim handles it)
- Note versioning/history
- Collaborative features
