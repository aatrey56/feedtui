# feedtui

A configurable terminal dashboard for browsing news, stocks, sports, and more - with a virtual pet companion!

## Features

- **Hacker News** - Browse top, new, and best stories
- **Stock Ticker** - Track your portfolio in real-time
- **RSS Feeds** - Subscribe to your favorite news sources
- **Sports Scores** - Follow NBA, NFL, EPL, and more
- **Twitter/X** - Post, reply, search tweets via Bird CLI integration
- **Pixel Art** - Convert images to beautiful terminal pixel art
- **World Clock** - Multi-timezone clock with integrated stopwatch
- **Tui** - Your virtual companion creature that levels up as you use the terminal!

## Installation

### Option 1: Install via pip (Recommended)

No Rust toolchain required! Install directly from PyPI:

```bash
pip install feedtui
```

Or with pipx for isolated installation:

```bash
pipx install feedtui
```

### Option 2: From crates.io (Rust)

```bash
cargo install feedtui
```

### Option 3: Quick Install Script

```bash
git clone https://github.com/muk2/feedtui
cd feedtui
./install.sh
```

### Option 4: Using Make

```bash
git clone https://github.com/muk2/feedtui
cd feedtui
make install
```

### Option 5: Manual Install

```bash
git clone https://github.com/muk2/feedtui
cd feedtui
cargo install --path .
```

All Rust-based methods install the `feedtui` binary to `~/.cargo/bin/`. Make sure this directory is in your PATH.

## Getting Started

### Interactive Configuration Wizard

The easiest way to get started is to run the configuration wizard:

```bash
feedtui init
```

This will guide you through setting up your dashboard with an interactive prompt.

### Manual Configuration

Alternatively, create a `.feedtui` folder in your home directory and add a `config.toml` file:

```bash
mkdir -p ~/.feedtui
cp config.example.toml ~/.feedtui/config.toml
```

Edit the config to customize your dashboard layout and feeds.

## Usage

### Run the dashboard

```bash
feedtui
```

### Command-line options

```bash
# Use a custom config file
feedtui --config /path/to/config.toml

# Override refresh interval
feedtui --refresh 30

# View configuration status
feedtui config

# Reconfigure with wizard
feedtui init --force

# Show installation help
feedtui install

# Show version
feedtui --version

# Show help
feedtui --help
```

## Meet Tui!

Tui (pronounced "chew-ee") is your virtual companion creature that lives in your terminal. The more you use feedtui, the more Tui grows!

### Features

- **10 Different Species** - Choose from Blob, Bird, Cat, Dragon, Fox, Owl, Penguin, Robot, Spirit, or Octopus
- **Leveling System** - Earn XP just by using the terminal
- **Skill Tree** - Unlock skills with points earned from leveling up
- **Outfits** - Customize Tui with unlockable outfits like Hacker, Wizard, Ninja, Astronaut, and more
- **Moods** - Tui reacts to how often you visit
- **Persistent Progress** - Your creature's progress is saved automatically

### Keybindings

| Key | Action |
|-----|--------|
| `t` | Toggle Tui menu |
| `Tab` / `Shift+Tab` | Switch between menu tabs / widgets |
| `j` / `k` or arrows | Navigate lists |
| `Enter` | Select/purchase items in menu |
| `r` | Refresh feeds |
| `q` | Quit |

### Skill Tree

Unlock skills by spending points:

- **Greeting** (Free) - Tui greets you on startup
- **News Digest** (10 pts) - Highlights important news
- **Stock Alert** (15 pts) - Alerts on significant movements
- **Quick Learner** (15 pts) - +10% XP gain
- **Speed Read** (20 pts) - Faster feed refresh
- **Fast Learner** (30 pts) - +25% XP gain
- **Cosmic Insight** (50 pts) - Trending topic insights
- **Fire Breath** (40 pts) - Cosmetic fire animation
- **Omniscience** (100 pts) - Maximum XP boost

### Outfit Unlocks

Outfits unlock as you level up:

| Level | Outfit |
|-------|--------|
| 1 | Default |
| 5 | Hacker |
| 10 | Wizard |
| 15 | Ninja |
| 20 | Astronaut |
| 25 | Robot |
| 30 | Dragon |
| 50 | Legendary |

## Available Widgets

feedtui supports the following configurable widgets. Each widget can be positioned in a grid layout and customized with various options.

### Creature Widget

Your virtual companion that lives in your terminal and levels up as you use feedtui!

**Configuration:**
```toml
[[widgets]]
type = "creature"
title = "Tui"                  # Widget title
show_on_startup = true         # Show creature menu on startup
position = { row = 0, col = 0 }  # Grid position
```

**Features:**
- 10 different species (Blob, Bird, Cat, Dragon, Fox, Owl, Penguin, Robot, Spirit, Octopus)
- Leveling system with XP progression
- Unlockable skills and outfits
- Mood system based on usage
- Press `t` to open the Tui menu

### Hacker News Widget

Browse Hacker News stories directly in your terminal.

**Configuration:**
```toml
[[widgets]]
type = "hackernews"
title = "Hacker News"          # Widget title
story_count = 10               # Number of stories to display (default: 10)
story_type = "top"             # Story type: "top", "new", or "best" (default: "top")
position = { row = 0, col = 1 }  # Grid position
```

**Features:**
- Browse top, new, or best stories
- Configurable story count
- Direct links to discussions

### Stocks Widget

Track your stock portfolio with real-time price updates.

**Configuration:**
```toml
[[widgets]]
type = "stocks"
title = "Portfolio"            # Widget title
symbols = ["AAPL", "GOOGL", "MSFT", "NVDA", "TSLA"]  # Stock ticker symbols
position = { row = 0, col = 2 }  # Grid position
```

**Features:**
- Real-time stock price tracking
- Multiple symbols support
- Price change indicators

### RSS Widget

Subscribe to your favorite RSS feeds and stay updated with the latest content.

**Configuration:**
```toml
[[widgets]]
type = "rss"
title = "Tech News"            # Widget title
feeds = [                      # List of RSS feed URLs
  "https://feeds.arstechnica.com/arstechnica/technology-lab",
  "https://www.theverge.com/rss/index.xml"
]
max_items = 10                 # Maximum items to display per feed (default: 15)
position = { row = 1, col = 0 }  # Grid position
```

**Features:**
- Multiple RSS feed support
- Configurable item limit
- Feed aggregation

### Sports Widget

Follow live scores and updates from major sports leagues.

**Configuration:**
```toml
[[widgets]]
type = "sports"
title = "Sports"               # Widget title
leagues = ["nba", "nfl", "mlb", "nhl", "epl", "mls", "ncaaf", "ncaab"]  # Leagues to follow
position = { row = 1, col = 1 }  # Grid position
```

**Supported Leagues:**
- `nba` - NBA Basketball
- `nfl` - NFL Football
- `mlb` - Major League Baseball
- `nhl` - NHL Hockey
- `epl` or `premier-league` - English Premier League
- `mls` - Major League Soccer
- `ncaaf` or `college-football` - College Football
- `ncaab` or `college-basketball` - College Basketball

**Features:**
- Live scores and game status
- Multiple league support
- Real-time updates powered by ESPN API

### GitHub Widget

Comprehensive GitHub dashboard with notifications, pull requests, and recent commits.

**Configuration:**
```toml
[[widgets]]
type = "github"
title = "GitHub Dashboard"     # Widget title
token = "${GITHUB_TOKEN}"      # GitHub personal access token (use env variable)
username = "your-username"     # Your GitHub username
show_notifications = true      # Show notifications tab (default: true)
show_pull_requests = true      # Show pull requests tab (default: true)
show_commits = true            # Show commits tab (default: true)
max_notifications = 20         # Max notifications to display (default: 20)
max_pull_requests = 10         # Max PRs to display (default: 10)
max_commits = 10               # Max commits to display (default: 10)
position = { row = 1, col = 2 }  # Grid position
```

**Setup:**
1. Create a GitHub personal access token with `notifications` and `repo` scopes
2. Set environment variable: `export GITHUB_TOKEN=your_token_here`
3. Use `${GITHUB_TOKEN}` in config to reference the environment variable

**Features:**
- GitHub notifications feed
- Pull request tracking
- Recent commit history
- Use `h`/`l` or arrow keys to switch between tabs

### YouTube Widget

Display videos from YouTube channels or search queries.

**Configuration:**
```toml
[[widgets]]
type = "youtube"
title = "YouTube"              # Widget title
api_key = "${YOUTUBE_API_KEY}" # YouTube Data API v3 key (use env variable)
channels = ["UCXuqSBlHAE6Xw-yeJA0Tunw"]  # Optional: List of channel IDs
search_query = "rust programming"  # Optional: Search query for videos
max_videos = 15                # Maximum videos to display (default: 15)
position = { row = 2, col = 0 }  # Grid position
```

**Setup:**
1. Get a YouTube Data API v3 key from [Google Cloud Console](https://console.cloud.google.com/apis/credentials)
2. Set environment variable: `export YOUTUBE_API_KEY=your_key_here`
3. Use `${YOUTUBE_API_KEY}` in config to reference the environment variable

**Features:**
- Display videos from specific channels
- Search for videos by query
- Configurable video limit
- Video titles and metadata

### Twitter/X Widget

Interactive Twitter/X feed powered by [Bird CLI](https://github.com/xrehpicx/bird) for posting, replying, searching, and reading tweets directly from your terminal.

**Prerequisites:**
- Bird CLI installed: `bun install -g bird-cli`
- Twitter/X authentication tokens set as environment variables:
  - `CT0` - Cookie token from twitter.com
  - `AUTH_TOKEN` - Authentication token from twitter.com

**Configuration:**
```toml
[[widgets]]
type = "twitter"
title = "Twitter/X"             # Widget title
position = { row = 2, col = 2 }  # Grid position
```

**Setup:**
1. Install Bird CLI: `bun install -g bird-cli`
2. Extract cookies from twitter.com (use browser dev tools):
   - `CT0` cookie value
   - `auth_token` cookie value
3. Set environment variables:
```bash
export CT0="your_ct0_token"
export AUTH_TOKEN="your_auth_token"
```

**Features:**
- Tweet composition with modal interface
- Reply to tweets
- Search Twitter/X
- View mentions
- Read individual tweets and threads
- When Twitter widget is selected:
  - Press `t` to compose a new tweet
  - Press `r` to reply to selected tweet
  - Press `/` to open search
  - Press `m` to load mentions
  - Press `Enter` to read selected tweet
  - Press `Esc` to close modals

**Note:** This widget requires external authentication and Bird CLI to be properly configured.

### Pixel Art Widget

Convert images into beautiful terminal-rendered pixel art. Supports PNG, JPEG, and WebP formats with adjustable pixel resolution.

**Configuration:**
```toml
[[widgets]]
type = "pixelart"
title = "Pixel Art"             # Widget title
image_path = "/path/to/image.png"  # Path to image file (optional)
pixel_size = 32                 # Target pixel resolution (optional, default: 32)
position = { row = 3, col = 0 }  # Grid position
```

**Supported Image Formats:**
- PNG
- JPEG
- WebP

**Features:**
- Image-to-pixel art conversion with nearest-neighbor scaling
- Adjustable pixel resolution (8×8 to 128×128)
- True color terminal rendering (24-bit RGB)
- Aspect ratio preservation
- Scrollable output for large images
- Real-time pixel size adjustment

**Usage:**
1. Configure `image_path` in your config.toml
2. Select the widget with Tab
3. Use keybindings to interact:
   - Press `+` to increase pixel size (8 → 16 → 32 → 64 → 128)
   - Press `-` to decrease pixel size (128 → 64 → 32 → 16 → 8)
   - Use `↑↓` or `j`/`k` to scroll through large images

**Display Information:**
- Original image dimensions
- Pixelated dimensions
- Current pixel size setting
- Scroll indicator when image exceeds viewport

**Example Use Cases:**
- Display profile pictures as pixel art
- Create retro-style avatars
- Preview game sprites
- Terminal art galleries
- NFT-style pixel aesthetics

### Clock Widget

Multi-timezone world clock with an integrated stopwatch for productivity tracking.

**Configuration:**
```toml
[[widgets]]
type = "clock"
title = "World Clock"          # Widget title
timezones = [                  # List of IANA timezone identifiers
  "America/New_York",
  "Europe/London",
  "Asia/Tokyo"
]
position = { row = 2, col = 1 }  # Grid position
```

**Common Timezones:**
- `America/New_York` - Eastern Time (US)
- `America/Los_Angeles` - Pacific Time (US)
- `America/Chicago` - Central Time (US)
- `Europe/London` - UK Time
- `Europe/Paris` - Central European Time
- `Asia/Tokyo` - Japan Time
- `Asia/Shanghai` - China Time
- `UTC` - Coordinated Universal Time

**Features:**
- Multiple timezone support with IANA timezone database
- Real-time clock updates (every second)
- Local timezone highlighting
- Built-in stopwatch with start/pause/reset controls
- When clock widget is selected:
  - Press `s` to Start/Pause stopwatch
  - Press `r` to Reset stopwatch
- Non-blocking time updates for smooth UI

## Example Config

Here's a complete example showing all available widgets:

```toml
[general]
refresh_interval_secs = 60
theme = "dark"

# Tui - Your companion creature! - top left
# Press 't' to open the Tui menu and customize your creature
[[widgets]]
type = "creature"
title = "Tui"
show_on_startup = true
position = { row = 0, col = 0 }

# Hacker News - top middle
[[widgets]]
type = "hackernews"
title = "Hacker News"
story_count = 10
story_type = "top"  # top, new, best
position = { row = 0, col = 1 }

# Stocks - top right
[[widgets]]
type = "stocks"
title = "Portfolio"
symbols = ["AAPL", "GOOGL", "MSFT", "NVDA", "TSLA"]
position = { row = 0, col = 2 }

# Tech News (RSS) - bottom left
[[widgets]]
type = "rss"
title = "Tech News"
feeds = [
  "https://feeds.arstechnica.com/arstechnica/technology-lab",
  "https://www.theverge.com/rss/index.xml"
]
max_items = 10
position = { row = 1, col = 0 }

# Sports - bottom middle
[[widgets]]
type = "sports"
title = "Sports"
leagues = ["nba", "nfl", "epl"]
position = { row = 1, col = 1 }

# GitHub Dashboard - bottom right
# Requires a GitHub personal access token with notifications and repo scope
# Set environment variable: export GITHUB_TOKEN=your_token_here
[[widgets]]
type = "github"
title = "GitHub Dashboard"
token = "${GITHUB_TOKEN}"
username = "your-username"
show_notifications = true
show_pull_requests = true
show_commits = true
max_notifications = 20
max_pull_requests = 10
max_commits = 10
position = { row = 1, col = 2 }

# YouTube Widget - Optional
# Display YouTube videos from channels or search queries
# Requires a YouTube Data API v3 key
# Get your API key from: https://console.cloud.google.com/apis/credentials
# Set environment variable: export YOUTUBE_API_KEY=your_key_here
# [[widgets]]
# type = "youtube"
# title = "YouTube"
# api_key = "${YOUTUBE_API_KEY}"
# channels = []  # Optional: List of channel IDs to display videos from
# search_query = "rust programming"  # Optional: Search query for videos
# max_videos = 15
# position = { row = 2, col = 0 }
```

## Python API

If you installed via pip, you can also use feedtui as a Python library:

```python
import feedtui

# Run the TUI
feedtui.run()

# Run with custom config
feedtui.run(config_path="/path/to/config.toml")

# Run with custom refresh interval
feedtui.run(refresh_interval=30)

# Initialize a new config file
config_path = feedtui.init_config()
print(f"Config created at: {config_path}")

# Get config path
print(feedtui.get_config_path())

# Get version
print(feedtui.version())
```

## Development

### Running from source (without installing)

```bash
# Debug mode
cargo run

# Release mode
cargo run --release

# Or use make
make dev    # debug mode
make run    # release mode
```

### Common development tasks

```bash
# Format code
cargo fmt
# or
make fmt

# Run linter
cargo clippy
# or
make clippy

# Run tests
cargo test
# or
make test

# Clean build artifacts
cargo clean
# or
make clean
```

## License

MIT
