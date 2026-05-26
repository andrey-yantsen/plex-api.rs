# plex-api.rs

A Rust library for communicating with Plex Media Server and the plex.tv cloud account service. The library exposes two independent entry points — `MyPlex` and `Server` — and targets both library consumers and the `plex-cli` reference CLI.

## Language

### Account & Identity

**MyPlex**:
The plex.tv cloud account service. Entry point for cloud-authenticated access — handles auth, device registry, sharing, and Plex Home management.
_Avoid_: plex.tv, cloud, account service

**Server**:
A Plex Media Server instance. Reachable directly (no MyPlex auth) or via a **Device** obtained from **MyPlex**.
_Avoid_: PMS, media server, host

**Player**:
A resolved, connectable Plex client device. Obtained by connecting to a **Device** of player type.
_Avoid_: client, endpoint

**Device**:
An unresolved registry entry from **MyPlex** — what MyPlex knows about before a connection is made. Resolves to either a **Server** or a **Player**.
_Avoid_: client, node

**Friend**:
A plex.tv account that has a sharing relationship with the current **MyPlex** account — pending or accepted.
_Avoid_: user, invite, contact

**InviteStatus**:
The lifecycle state of a **Friend** relationship (`PendingSent`, `PendingReceived`, `Accepted`).

### Plex Home

**Plex Home**:
Feature allowing a **MyPlex** account owner to manage restricted sub-accounts on a shared server.
_Avoid_: home, family, multi-user

**Managed User**:
A restricted sub-account under **Plex Home**, owned by the primary **MyPlex** account.
_Avoid_: home user, sub-user, child account

### Content

**Library**:
A named media collection on a **Server** (e.g. Movies, TV Shows, Music, Photos).
_Avoid_: section, collection, folder

**Item**:
A generic content piece inside a **Library**.
_Avoid_: media, content, entry

**MediaItem**:
An **Item** subtype that has playable media attached.
_Avoid_: file, asset, video

### Transcoding

**Streaming**:
Real-time transcoding for playback via HLS or DASH. Produces a **Transcode Session** on the server.
_Avoid_: live transcode, online playback

**Offline Transcoding**:
Background server-side conversion that produces a downloadable file. The legacy mobile downloads feature.
_Avoid_: sync, mobile sync, background transcode

**Download Queue**:
Queue-based variant of **Offline Transcoding**. Items are added to a server queue; each is processed in order and downloadable via byte-range requests when ready.
_Avoid_: sync queue, download list

**Transcode Session**:
Server-side state for an active **Streaming** transcode.
_Avoid_: stream, session

### Server Ownership

**Claimed Server**:
A **Server** linked to a **MyPlex** account. Can be authenticated via MyPlex token.
_Avoid_: registered server, owned server

**Unclaimed Server**:
A **Server** not linked to any **MyPlex** account. Accessible by direct token only.
_Avoid_: local server, anonymous server

### Testing

**Offline Tests**:
Tests that do not require a real **Server**. Use mocked HTTP data.
_Avoid_: unit tests

**Online Tests**:
Tests that run against a real **Server** instance (Docker). Split into **claimed** and **unclaimed** variants by feature flag.
_Avoid_: integration tests, live tests

## Relationships

- **MyPlex** holds a registry of **Devices**; each **Device** resolves to a **Server** or **Player**
- A **Claimed Server** is linked to exactly one **MyPlex** account
- An **Unclaimed Server** has no **MyPlex** association
- A **Server** contains one or more **Libraries**
- A **Library** contains zero or more **Items**; a **MediaItem** is an **Item** with playable media
- **MyPlex** manages **Friends** and their access to **Servers** via Sharing
- **Plex Home** is managed through **MyPlex**; it owns zero or more **Managed Users**
- **Streaming** creates a **Transcode Session**; **Offline Transcoding** and **Download Queue** do not

## Example dialogue

> **Dev:** "Should connecting to a **Player** go through **MyPlex** or can I do it directly?"
> **Domain expert:** "Either. If you have a **MyPlex** session you can get the **Device** from the registry and resolve it to a **Player**. Or build a direct client if you already know the address."

> **Dev:** "Is an unclaimed **Server** just a **Server** you haven't added to **MyPlex** yet?"
> **Domain expert:** "Yes — **Unclaimed** means no **MyPlex** account owns it. You auth with a direct token, not a MyPlex token. Claiming it is a one-time operation that ties it to your **MyPlex** account permanently."

> **Dev:** "What's the difference between **Offline Transcoding** and **Download Queue**?"
> **Domain expert:** "Same idea, different protocol. **Offline Transcoding** is the old mobile sync — request one file, server converts it, you download it. **Download Queue** is the newer version — queue multiple items, byte-range download when ready. More stable."

## Flagged ambiguities

- "user" was used loosely to mean **Friend** (another plex.tv account), **Managed User** (a Plex Home sub-account), or the current **MyPlex** account holder — these are three distinct concepts.
- "sync" was used to mean both **Offline Transcoding** and **Download Queue** — resolved: use the specific term, not "sync".
