# NotebookOS product specification

Status: Draft for product and design review  
Version: 0.1  
Date: 2026-10-07

## 1. Product summary

NotebookOS is a local-first workspace for research, thinking, planning, and meeting knowledge. It combines typed documents, freeform handwritten canvases, images and files, linked knowledge, Kanban boards, calendars, and imported meeting notes in one coherent information model.

The primary experience is a quiet document workspace: open a workspace, find or capture an idea, write or draw without setup, and connect the result to the people, projects, sources, meetings, and tasks it belongs to.

The first product is personal-first. Its data model and sync protocol must leave room for shared workspaces, but real-time multi-user editing is not required for the first major release.

## 2. Goals

1. Make capture faster than choosing where information belongs.
2. Support research-quality notes: citations, attachments, PDFs, links, backlinks, search, and durable export.
3. Make Apple devices first-class, especially Apple Pencil on iPad, while keeping macOS, Windows, and Linux useful from one Rust codebase.
4. Work fully offline for local content and reconcile changes when connectivity returns.
5. Keep information portable. A user must be able to export a workspace without depending on NotebookOS servers.
6. Integrate external systems without making them the hidden source of truth.
7. Use one stable vocabulary and information hierarchy across desktop, tablet, and phone.

## 3. Product principles

- **Local-first:** opening, editing, searching, linking, and drawing work without a network.
- **Capture first:** the quick-capture path asks for content before organization.
- **Quiet by default:** content receives more visual weight than chrome, badges, dashboards, or decorative cards.
- **Progressive structure:** pages can begin in the Inbox and gain a notebook, tags, links, and project context later.
- **Visible provenance:** imported content shows its source, last sync, and whether changes will sync back.
- **One concept, one name:** Workspace, Notebook, Section, Page, Board, Card, Event, Meeting, and Link have stable meanings.
- **Keyboard, pointer, and touch parity:** important actions always have a path that does not depend on hover.
- **Accessible state:** focus, selection, offline, syncing, conflict, readonly, and failure states are explicit.

## 4. Primary users and jobs

### Independent researcher

- Capture reading notes, screenshots, papers, citations, experiments, and questions.
- Link claims to sources and related pages.
- Review a project through a notebook, timeline, backlinks, and saved searches.

### Knowledge worker

- Prepare for meetings from calendar context.
- Import Granola notes and turn decisions or action items into durable project knowledge.
- Track deliverables on a board and link cards to supporting notes.

### Idea-driven creator

- Sketch with Apple Pencil, arrange text and images spatially, and connect related concepts.
- Move fluidly between a freeform canvas and structured documents.
- Retrieve old ideas by content, tags, people, dates, and links.

## 5. Information architecture

```text
Account / local profile
└── Workspace (Personal, Work, Research Lab, …)
    ├── Inbox
    ├── Notebooks
    │   └── Notebook
    │       ├── Section
    │       │   └── Page
    │       └── Section group (optional)
    ├── Boards
    │   └── Board → Column → Card
    ├── Calendar
    │   └── Connected calendar → Event
    ├── Meetings
    │   └── Meeting page (native or imported)
    ├── Files
    ├── Tags
    └── Trash
```

Every first-party object receives a stable ID. Pages, blocks, headings, cards, events, meetings, attachments, people, and external records can all be link targets. Moving or renaming an object must not break links.

### Core navigation

- **Home:** resume recent work, capture quickly, see today's events and due cards.
- **Notes:** notebook and section hierarchy, recent pages, favorites, and saved searches.
- **Boards:** native boards and linked external boards.
- **Calendar:** agenda, day, week, and month views across enabled calendars.
- **Meetings:** meeting notes grouped by upcoming, recent, folder, project, and person.
- **Search:** global search and command palette.

Home is a resumption surface rather than a metrics dashboard. It should favor recent context, the next event, and incomplete work over charts and counts.

## 6. Page and editor model

NotebookOS supports two page layouts that share the same block types and link system.

### Document page

A vertically ordered document for research journals, meeting notes, daily notes, long-form writing, and structured reference material.

Supported blocks:

- paragraph, headings, quote, callout, divider;
- bulleted, numbered, task, and toggle lists;
- table;
- code block with language metadata;
- equation and inline math;
- image, gallery, audio, video, file, and web bookmark;
- PDF/document attachment with page preview;
- ink block with an expandable drawing surface;
- board, calendar event, meeting, and page reference;
- citation and bibliography entry;
- synced excerpt or transclusion from another block.

### Canvas page

A zoomable spatial surface for handwriting, diagrams, brainstorming, image annotation, and loose arrangement. Text blocks, ink, images, files, shapes, links, and embedded page excerpts can be freely positioned and layered.

The canvas has a finite content extent that expands as objects move. It should feel open-ended without relying on unbounded coordinates that harm export, search previews, or performance.

### Editing behavior

- Slash command inserts a block; keyboard shortcuts cover common formatting.
- Drag handles move blocks and open block commands on pointer platforms.
- Touch presents selection and block actions without requiring hover.
- Undo and redo operate per page and survive ordinary navigation.
- Paste preserves useful semantic structure from rich text and falls back cleanly to plain text.
- Autosave writes locally after coherent edits; the UI distinguishes `Saved locally`, `Syncing`, `Up to date`, `Offline`, and `Couldn’t sync`.
- Version history restores a prior page as a new revision rather than silently erasing later work.
- Markdown is an interchange format, not the internal data model.

### Handwriting and Apple Pencil

- Pen, fountain pen, pencil, highlighter, eraser, lasso, ruler, shapes, color, and stroke-width tools.
- Pressure, tilt, azimuth, predicted touches, coalesced touches, and palm rejection where the platform provides them.
- Finger drawing can be enabled or disabled independently from touch navigation.
- Lasso supports move, scale, duplicate, group, copy, and convert-to-image.
- Ink remains editable vector data. Exports may also include a flattened representation.
- Searchable handwriting uses optional on-device OCR/indexing; original strokes remain authoritative.
- On iPad and iPhone, a native PencilKit surface is the preferred capture bridge until GPUI mobile input is proven equivalent on physical devices.
- Desktop uses the shared stroke model through a GPUI custom element and accepts mouse, trackpad, stylus, and tablet input according to platform capability.

### Images, files, and PDFs

- Insert from files, clipboard, drag and drop, camera, Photos, scanner, and Share extension where available.
- Resize, crop, rotate, caption, add alt text, and annotate.
- PDFs support page thumbnails, text extraction when available, highlights, ink annotation, and deep links to a page and region.
- Attachments are content-addressed to avoid duplicate storage while preserving separate captions and placements.
- Missing, downloading, failed, and permission-denied assets have explicit placeholders and recovery actions.

## 7. Links, references, and knowledge retrieval

### Linking

- `[[` opens page search and page creation.
- `@` references people, meetings, events, cards, or other named objects.
- A block can expose a stable anchor for a block-level link.
- Pasting a NotebookOS link offers Link, Mention, Synced excerpt, or Embed when applicable.
- Renames and moves preserve references through stable IDs.

### Backlinks and related context

- Each page has a collapsible backlinks region grouped by linked mentions and unlinked mentions.
- A right inspector can show outgoing links, backlinks, properties, versions, and source provenance.
- Hover previews appear on desktop; tap previews or sheets are used on touch devices.
- A graph view is a later analysis tool, not primary navigation.

### Search

- Local full-text search across page text, headings, tags, filenames, OCR text, meeting metadata, and indexed transcripts.
- Filters for workspace, notebook, content type, tag, person, date, source, and sync state.
- Results show matched context and the path to the object.
- Search works offline for indexed local content.
- Saved searches can appear in a workspace sidebar.

## 8. Workspaces and organization

- Workspaces isolate notebooks, boards, integrations, calendars, search indexes, encryption keys, and settings.
- The workspace switcher shows identity, sync state, and recent workspaces.
- Default workspaces are not imposed; onboarding suggests Personal and Work.
- Workspace templates may create a research notebook, meeting folder, idea inbox, and project board.
- Pages can move between workspaces through an explicit transfer flow that reports links or integrations that cannot follow.
- Workspace-level settings control data location, sync, connectors, default templates, export, and retention.

## 9. Boards and task management

### Native boards

- Multiple boards per workspace.
- Columns, cards, subtasks, assignees, labels, due dates, priority, estimates, attachments, comments, and archive.
- Drag and keyboard commands move cards and columns.
- Card detail opens in a sheet on desktop and a full screen on phone.
- A card can link to any page, block, meeting, event, file, or external URL.
- Table and list views reuse the same cards after the core board view is stable.
- Offline changes queue and reconcile when connected.

### Trello connector

Trello is the recommended first external board connector because it provides an iOS widget, a free tier, OAuth, a REST API, and webhooks.

- A linked Trello board appears beside native boards with a visible Trello source label.
- Trello remains canonical for linked board structure, card ordering, memberships, and comments.
- NotebookOS keeps an offline cache and a durable link map between Trello IDs and local references.
- Supported bidirectional fields: board/list/card title, description, position, due date, labels, completion/archive state, checklist items, and attachments where API permissions allow.
- NotebookOS-only relationships, such as links to private notes, stay local and are never written into Trello without an explicit representation chosen by the user.
- Webhooks update connected clients through the sync service; periodic reconciliation catches missed events.
- Origin identifiers prevent webhook echo loops. Conflicts are recorded in activity history and surface a choice when automatic field-level merge is unsafe.
- The free Trello limit of 10 open boards per workspace is shown during connection rather than discovered after setup.

The connector seam must allow later providers such as Todoist, Linear, Jira, or GitHub Projects without forcing their data models into the native board schema.

## 10. Calendar

### Views

- Agenda is the default because it works at every window width.
- Day, week, and month views are available on larger screens.
- Enabled calendars keep their provider color, but selection and status never rely on color alone.
- Upcoming events appear on Home and can be hidden per workspace.

### Connections

- Apple Calendar through EventKit in the Apple host.
- Google Calendar through OAuth and the Calendar API.
- Microsoft 365 through OAuth and Microsoft Graph.
- ICS subscription and import; CalDAV can follow after interoperability testing.

### Event relationships

- Open an event to see time, participants, location, links, agenda, related pages, and meeting notes.
- Create a meeting note from an event using a template.
- Link a page or card to an event without copying the event into the note.
- Initial calendar integration is readonly except for creating a linked NotebookOS note. Event creation and editing follow after permission, recurrence, timezone, and attendee behavior are tested per provider.
- Cached calendars remain visible offline with a clear last-updated time.

## 11. Granola and meetings

### Meetings area

- Meetings is a dedicated collection inside each workspace and can also be represented as a `Meetings` notebook for familiar browsing.
- Meeting pages contain event metadata, attendees, private notes, imported summary, decisions, action items, transcript availability, related pages, and source provenance.
- Views group meetings by day, folder, project, person, and import source.
- Imported content is editable through a user-owned notes section; source-owned sections remain visibly synced or readonly according to connector capability.

### Granola connection modes

1. **Granola MCP:** browser OAuth, useful for personal read access and compatible custom clients. The free Granola plan is limited to personal notes from the last 30 days and restricts some folder, search, and transcript tools.
2. **Granola API:** API-key access to notes, transcripts, and summaries on Business and Enterprise plans. Webhooks support automatic updates.
3. **Manual import:** historical export or individual files for users who do not enable either connector.

### Import behavior

- The user chooses a destination workspace and defaults to the `Meetings` collection.
- External meeting ID, workspace, source URL, folder, title, time, attendees, summary, and transcript availability are retained.
- Re-import updates source-owned sections without overwriting personal annotations.
- Deleted or inaccessible source notes remain locally available only when the user's retention settings and source terms permit it; their disconnected state is visible.
- Action items can be copied into a native board or a linked Trello board through an explicit review step.

## 12. Home and capture

Home contains:

- Continue working: recent pages and canvases.
- Quick capture: new page, handwritten note, photo/scan, voice note, or card.
- Today: next calendar events and linked meeting notes.
- Focus: cards due soon or intentionally pinned, limited to a short list.
- Inbox: uncategorized captures awaiting review.

System-wide capture on Apple platforms should include a Share extension, Shortcuts/App Intents, Spotlight indexing, and widgets. The first widget set should support Quick note, New ink note, Inbox, and Today. A board widget is unnecessary for Trello-linked workflows because Trello already supplies one; a native NotebookOS board widget can follow after native boards mature.

## 13. Platform experience

### macOS

- Full document workspace with native menu bar, keyboard shortcuts, multiple windows, drag and drop, Quick Look where appropriate, system sharing, and Services integration.
- Trackpad and tablet input supplement mouse and keyboard workflows.

### iPadOS

- Primary Apple Pencil experience.
- Sidebar plus detail layout in regular width; focused full-screen editor in compact width or Stage Manager constraints.
- Native host owns scene lifecycle, safe areas, keyboard avoidance, document picker, PencilKit, share sheets, and system extensions.
- GPUI Kit hosts validated content and workspace surfaces; unsupported interactions receive a native bridge or platform-specific presentation.

### iOS

- Optimized for capture, reading, search, meeting context, card updates, and focused editing.
- Bottom or native stack navigation replaces the desktop multi-pane shell.
- Canvas tools use compact sheets and prioritize pen, eraser, lasso, undo, and color.

### Windows and Linux

- Full desktop notes, boards, calendar, meetings, linking, search, and sync.
- Ink input uses available pointer/tablet events; platform gaps are surfaced as capabilities rather than silent failures.

## 14. Interaction and visual design

- Desktop uses a stable workspace sidebar, an optional collection pane, and a primary work area. Resizable boundaries persist per workspace and clamp to current window size.
- The primary editor receives remaining space. Inspectors are dismissible and never cover the primary object by default.
- Standard density is used for navigation and editors; compact density is reserved for toolbars, board metadata, menus, and repeated rows.
- Selection appears through the selected item's full surface, stronger foreground, and weight. It does not use a decorative edge bar.
- The principal commit in a dialog or short form uses primary styling. Frequent toolbar commands use ordinary or quiet buttons.
- Menus and context menus hold secondary commands; essential commands also have visible or keyboard paths.
- Every scrollable region has one owner, with its scrollbar at that region's edge.
- Light, dark, high-contrast, reduced-motion, larger-text, and keyboard-only use are required validation modes.

### Keyboard path

- `Cmd/Ctrl+K`: command palette and global object search.
- `Cmd/Ctrl+N`: new page in the current context.
- `Cmd/Ctrl+Shift+N`: quick capture.
- `Cmd/Ctrl+P`: page search.
- `Cmd/Ctrl+Option/Alt+L`: copy link to current object.
- `[[`: insert page link while editing.
- Arrow keys navigate trees, lists, menus, boards, and calendars according to their control conventions.
- Escape dismisses the topmost overlay and restores focus.

Shortcuts are user-configurable after the command model is stable. Platform menu labels and Command/Control notation follow native conventions.

## 15. Empty, loading, offline, and failure states

- A new workspace opens to a useful empty state with New page, New handwritten note, and Import actions.
- Cached content remains usable during connector or sync refresh.
- Skeletons appear only for real waits with known shape.
- Offline state is quiet while edits are safely stored locally; it becomes prominent only when an action needs the network.
- Sync errors name the affected object or connector and offer Retry or Review details.
- Permission denial explains which account or system setting controls access.
- Readonly imported content remains selectable, linkable, and copyable.
- Destructive operations go to Trash and support undo. Permanent deletion names the exact scope and requires confirmation.

## 16. Data ownership, privacy, and sync requirements

- Local storage is authoritative for first-party personal content while the device is offline.
- Workspace encryption keys are stored through platform credential stores. Synced private content must be end-to-end encrypted before leaving the client.
- Attachments use content hashes, encrypted blobs, and resumable transfer.
- The sync protocol transmits operations or revisions with stable object IDs and idempotency keys.
- Conflicts merge automatically only when the result is deterministic and preserves both intents. Other conflicts create a recoverable copy or a review surface.
- Connector credentials are stored in platform credential stores, never the document database or logs.
- Telemetry is opt-in and excludes document content, titles, links, filenames, transcripts, and ink.
- Export supports a complete portable archive plus readable Markdown/HTML, JSON metadata, original attachments, vector ink, and PDF snapshots where applicable.
- Deleting an account does not delete local workspaces without a separate explicit action.

## 17. Technical architecture

### UI and platform boundary

- GPUI Kit is the primary Rust UI framework.
- Desktop uses the standard GPUI Kit application bootstrap and one `Root` per window.
- iOS/iPadOS use the documented external host path. A UIKit or SwiftUI container owns the native scene and embeds GPUI views. The host also owns PencilKit, WidgetKit, EventKit, Share extensions, App Intents, safe areas, and on-screen keyboard layout.
- Mobile surfaces must be validated on simulator and physical devices. A successful compile is not considered mobile support.

### Capability-oriented Rust workspace

```text
crates/
├── app-desktop/          # desktop bootstrap and window composition
├── app-mobile/           # Rust entry points embedded by Apple/Android hosts
├── domain/               # stable IDs, entities, commands, revisions
├── storage/              # local database, blobs, migrations, indexing
├── sync/                 # encrypted operation transport and reconciliation
├── workspace/            # workspace shell and switching
├── notebook/             # notebooks, sections, pages, navigation
├── editor/               # document blocks, selection, history
├── canvas/               # spatial layout, zoom, selection
├── ink/                  # shared stroke model, tools, rendering adapters
├── search/               # index and query model
├── boards/               # native boards and card workflows
├── calendar/             # provider-neutral event model and views
├── meetings/             # meeting pages and provenance
├── integrations/         # connector registry and shared sync contracts
├── integration-trello/
├── integration-granola/
├── integration-google/
├── integration-microsoft/
└── ui/                   # application components and design tokens
apple/
├── NotebookOS/           # iOS/iPadOS/macOS host as needed
├── Widgets/
├── ShareExtension/
└── PencilKitBridge/
```

Crates are introduced when their capability has a stable state and lifecycle; this is the intended boundary map, not a requirement to create every crate on day one.

### GPUI state ownership

| State | Owner | Notes |
| --- | --- | --- |
| Workspace index and active workspace | application service + shell entity | Shared across windows through explicit commands/events |
| Notebook tree and page summaries | notebook model | Long lists are virtualized and keyed by stable IDs |
| Open page, selection, history | editor entity per open page | Never recreated during render |
| Text input/composition | GPUI Kit input/editor state | Subscriptions retained by owning view |
| Canvas viewport and tool | canvas entity | Separate from persisted object geometry |
| Ink strokes | ink document model | Native and GPUI adapters write the same stroke operations |
| Board selection and drag state | board view entity | Cards remain keyed by card ID across moves |
| Connector credentials | platform secure store | Views receive capability/status, never raw secrets |
| Sync status | workspace sync service | Emits narrow updates to affected objects |

### Command and side-effect rules

- UI entry points dispatch a shared semantic command instead of duplicating mutations.
- Rendering reads state and composes elements; storage, network, parsing, OCR, and sync start from named methods, commands, or lifecycle hooks.
- Async results carry object identity and revision so stale results cannot update a newly selected page.
- Platform-specific behavior sits behind narrow capabilities with a defined fallback.
- Application UI uses GPUI theme roles and semantic scale tokens. Raw colors and arbitrary layout pixels live only in theme definitions or audited physical boundaries.

## 18. Delivery plan

### Phase 0 — specification and design

- Approve product scope, information model, and platform architecture.
- Review interactive desktop, iPad, and iPhone HTML concepts.
- Resolve open product decisions listed below.

### Phase 1 — foundation and desktop shell

- Rust workspace, GPUI Kit bootstrap, theme, actions, application menu, and test harness.
- Local profile, workspaces, notebook tree, routing, command palette, and persisted pane layout.
- Local storage schema, migrations, stable IDs, attachment store, and explicit sync-state model.

### Phase 2 — documents and linked knowledge

- Document pages with essential rich blocks, images/files, undo/redo, autosave, and version history.
- Page and block links, backlinks, tags, full-text search, quick capture, and export.
- macOS, Windows, and Linux interaction and UI integration testing.

### Phase 3 — canvas and ink

- Shared ink model, GPUI desktop renderer, canvas page, selection, tools, image/PDF annotation.
- Apple mobile host, PencilKit bridge, device input validation, and iPad-focused layout.

### Phase 4 — encrypted multi-device sync and Apple extensions

- End-to-end encrypted workspace sync, attachment transfer, conflict handling, and device recovery.
- iPhone focused editor, Share extension, Quick note/Ink widgets, App Intents, and Spotlight.

### Phase 5 — boards and Trello

- Native boards, card detail, note relationships, offline queue, activity history.
- Trello OAuth, import/link flow, webhooks, reconciliation, and conflict UI.

### Phase 6 — calendar and meetings

- Agenda/week/month views, EventKit, Google Calendar, and Microsoft Graph readonly connections.
- Meeting-note templates and event linking.
- Granola MCP import, Granola API/webhook mode, historical import, and reviewed action-item extraction.

### Phase 7 — depth and collaboration

- Tables, equations, citations, advanced PDF workflows, OCR improvements, additional board providers.
- Publishing, shared workspaces, comments, presence, and real-time collaboration after personal sync is reliable.

## 19. Release acceptance criteria

The first public release is ready when a user can:

1. Create separate workspaces and organize notebooks, sections, and pages.
2. Write, format, link, search, export, and recover notes fully offline.
3. Add and annotate images and files.
4. Draw with low perceived latency and retain editable vector strokes.
5. Sync supported first-party content across at least macOS, iPadOS, and iOS without silent data loss.
6. Navigate core workflows with keyboard, pointer, and touch, with visible focus and accessible names.
7. Understand local save, remote sync, connector provenance, readonly, failure, and conflict states.
8. Restore from Trash and export a complete workspace independently of the service.

Boards, calendar, and Granola can ship in subsequent minor releases if delaying them protects note reliability, ink quality, or data portability.

## 20. Open product decisions

These decisions materially affect implementation and should be confirmed before Phase 1 or the named phase begins.

1. **Sync service:** NotebookOS-hosted encrypted sync, user-chosen storage, or both? Recommended: hosted encrypted sync first, portable archive always, user-chosen backends later.
2. **Collaboration target:** personal-first, published readonly pages, or shared editing in the first release? Recommended: personal-first with publish/export seams.
3. **Page defaults:** should New page create a document, canvas, or remember the last choice? Recommended: document by default, visible New canvas action, remember per notebook later.
4. **iPhone scope:** full canvas editing or focused capture/reading for the first release? Recommended: focused capture and document editing, with compact ink capture but not the full spatial canvas toolset.
5. **Trello authority:** should linked boards treat Trello as canonical? Recommended: yes, while native boards remain NotebookOS-owned.
6. **Granola plan:** will the primary account be Basic, Business, or Enterprise? This determines whether MCP's 30-day limit or the API/webhook path defines the first connector.
7. **Data sensitivity:** should biometric unlock and per-workspace locking be required for the first release? Recommended: include per-workspace lock in the storage design and ship it with encrypted sync.
8. **Sharing:** should workspace export be enough initially, or is a public web-publishing surface required early?

## 21. External capability notes

Verified on 2026-10-07:

- [GPUI Kit Mobile guide](https://gpui-kit.com/docs/mobile/) describes an experimental Swift-hosted iOS integration and a limited validated component set. It explicitly says mobile is not currently a primary target or full Flutter-like application framework.
- [GPUI Kit release notes](https://gpui-kit.com/releases/) document mobile hosting support introduced in v0.6.2; the current release is v0.7.1.
- [Trello iOS widgets](https://support.atlassian.com/trello/docs/trello-widgets-for-ios/) support quick card creation and completion for a chosen board/list.
- [Trello free workspace limits](https://support.atlassian.com/trello/docs/why-cant-i-create-a-board/) currently allow 10 open boards per free workspace.
- [Trello OAuth 2.0](https://developer.atlassian.com/cloud/trello/guides/rest-api/oauth-2-getting-started/) and [webhooks](https://developer.atlassian.com/cloud/trello/guides/rest-api/webhooks/) provide the basis for user-authorized bidirectional sync.
- [Granola MCP](https://docs.granola.ai/help-center/sharing/integrations/mcp) supports browser OAuth and custom Streamable HTTP clients; Basic access is limited to personal notes from the last 30 days.
- [Granola API](https://docs.granola.ai/help-center/sharing/integrations/granola-api) provides API-key access and webhooks on Business and Enterprise plans.

Connector limits and platform support must be rechecked when implementation begins because they can change independently of NotebookOS.
