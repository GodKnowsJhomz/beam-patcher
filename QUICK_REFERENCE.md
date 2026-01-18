# Beam Patcher - Quick Reference Guide

## Essential Configuration (config.yml)

### Minimal Setup
```yaml
app:
  name: "MyRO"
  client_exe: "Ragnarok.exe"

patcher:
  mirrors:
    - name: "Main"
      url: "https://patch.yourserver.com"
      priority: 1
  patch_list_url: "https://patch.yourserver.com/patchlist.txt"
```

---

## Patch Format Comparison

| Format | Speed | Size | GRF Editor | Recommended |
|--------|-------|------|------------|-------------|
| BEAM   | ⚡⚡⚡  | 📦   | ❌         | ✅ New servers |
| THOR   | ⚡⚡   | 📦📦 | ✅         | ✅ Compatibility |
| RGZ    | ⚡⚡   | 📦📦 | ✅         | ⚠️ Alternative |
| GRF    | ⚡    | 📦📦📦 | ✅       | ⚠️ Full update only |

---

## File Locations

```
project/
├── config.yml                    # Main configuration
├── patchlist.txt                 # Patch list (optional for local testing)
├── version.json                  # Auto-update info (optional)
├── assets/                       # Media files
│   ├── bgm.mp3                   # Background music
│   ├── trailer.mp4               # Video background
│   ├── logo.png                  # Custom logo
│   ├── background.jpg            # Background image
│   └── screenshots/              # Grid images
├── themes/                       # Custom themes
│   └── custom.css                # Custom CSS
└── target/release/               # Compiled binaries
    ├── beam-patcher.exe          # Main patcher
    └── beam-tools-ui.exe         # Patch creator tool
```

---

## patchlist.txt Format

```
# Comments start with #
001_initial.beam
002_update.beam
003_hotfix.beam

# THOR format also supported
004_content.thor

# Mixed formats allowed
005_final.beam
```

---

## version.json Format

```json
{
  "version": "1.0.1",
  "required": false,
  "download_url": "https://patch.yourserver.com/BeamPatcher-1.0.1.exe",
  "changelog": "- Fixed bugs\n- New features",
  "sha256": "abc123..."
}
```

---

## Common Window Sizes

| Resolution | Size | Best For |
|------------|------|----------|
| Small      | 800x600 | 1366x768 screens |
| Medium     | 1000x650 | 1920x1080 screens |
| Large      | 1200x700 | 2K/4K screens |

---

## Compression Methods

| Format | Compression | Speed | Ratio |
|--------|-------------|-------|-------|
| BEAM   | Zstd        | Fast  | Best  |
| THOR   | Zlib        | Medium| Good  |
| RGZ    | Gzip        | Medium| Good  |
| GRF    | Zlib        | Slow  | Good  |

---

## Creating Patches (Beam Tools UI)

### Directory Structure
```
Before:
D:\update\
  └── data\
      ├── texture\
      ├── model\
      └── sprite\

Settings:
- Input Path: D:\update\data
- Base Path: D:\update
- Output: D:\patches\001.beam

After (in patch):
root/
  └── data/
      ├── texture/
      ├── model/
      └── sprite/
```

---

## Server Setup Commands

### Apache
```bash
# Enable required modules
a2enmod headers
a2enmod deflate
a2enmod expires

# Restart Apache
systemctl restart apache2
```

### Nginx
```bash
# Edit config
nano /etc/nginx/sites-available/default

# Test config
nginx -t

# Reload
systemctl reload nginx
```

---

## Testing Checklist

- [ ] Config validates (YAML syntax)
- [ ] Patcher starts without errors
- [ ] Patches download successfully
- [ ] Files extract correctly
- [ ] Game launches after patching
- [ ] All UI elements display properly
- [ ] Social links work
- [ ] Auto-updater functions (if enabled)

---

## Troubleshooting Quick Fixes

| Problem | Solution |
|---------|----------|
| Patcher won't start | Check config.yml syntax |
| Can't download patches | Verify mirror URLs |
| Slow downloads | Add more mirrors / Use CDN |
| High bandwidth | Enable server compression |
| UI looks broken | Check theme files exist |
| Game won't launch | Verify client_exe path |

---

## Performance Optimization

### Client Side
```yaml
# Disable resource-heavy features
video_background_enabled: false
bgm_autoplay: false

# Reduce window size
layout:
  width: 800
  height: 600
```

### Server Side
```
1. Enable GZIP/Brotli compression
2. Use CDN (Cloudflare, CloudFront)
3. Enable caching headers
4. Optimize image sizes
5. Use multiple mirrors
```

---

## Security Best Practices

1. **Always use HTTPS** for production
2. **Verify checksums** enabled by default
3. **Don't expose** sensitive API keys in config
4. **Use CDN** to protect origin server
5. **Rate limit** patch downloads
6. **Monitor** for unusual download patterns

---

## Command Line Usage

### Run Patcher
```bash
# Normal mode
./beam-patcher.exe

# With custom config
./beam-patcher.exe --config custom-config.yml

# Debug mode
./beam-patcher.exe --debug
```

### Create Patches
```bash
# Using Beam Tools UI
./beam-tools-ui.exe

# Or use command line tools
./beam-create --input data/ --output patch.beam
```

---

## URL Patterns

```
Base URL: https://patch.yourserver.com

Required:
- /patchlist.txt          # Patch list file
- /001_initial.beam       # Patch files
- /002_update.beam

Optional:
- /version.json           # Patcher updates
- /patcher/latest.exe     # Patcher binary
- /api/news               # News feed
- /api/status             # Server status
```

---

## Environment Variables (Optional)

```bash
# Override config file location
export BEAM_CONFIG=/path/to/config.yml

# Override log level
export BEAM_LOG_LEVEL=debug

# Override game directory
export BEAM_GAME_DIR=/path/to/game
```

---

## Build from Source

```bash
# Clone repository
git clone https://github.com/beamguides/beam-patcher.git
cd beam-patcher

# Build release version
cargo build --release

# Binaries located at:
# target/release/beam-patcher.exe
# target/release/beam-tools-ui.exe
```

---

## Support

- **Documentation**: See CONFIGURATION_GUIDE.md
- **Issues**: https://github.com/beamguides/beam-patcher/issues
- **Discord**: https://discord.gg/DeMpCu2Q
- **GitHub**: https://github.com/beamguides/beam-patcher
