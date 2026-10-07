#!/usr/bin/env python3
"""Generate src/bin/app/borrow.theme.json from gpui-component's reference theme.

Starting from the shipped dark theme rather than from scratch is deliberate:
ThemeConfigColors has 146 required fields and no serde default, so a
hand-written JSON can fail to deserialize or, worse, quietly leave a component
on shadcn's palette. Carrying the reference forward and overriding only the
values BORROW_ owns keeps every exotic token (charts, skeletons, docks)
complete while the palette we care about is ours.

Tokens are taken from the `:root` block in static/style.css so that the two
remain one palette.
"""
import json, pathlib, sys, re, glob

REPO = pathlib.Path(__file__).resolve().parent.parent

# find the gpui-component crate source
cand = sorted(glob.glob(str(pathlib.Path.home().joinpath(
    ".cargo/registry/src/*/gpui-component-*/src/theme/default-theme.json"))))
if not cand:
    sys.exit("gpui-component default-theme.json not found in the cargo registry")
ref = json.loads(pathlib.Path(cand[-1]).read_text())

# ── BORROW_ tokens, read straight out of static/style.css's :root ─────────
css = (REPO / "static/style.css").read_text()
root_block = re.search(r":root\s*\{(.*?)\n\}", css, re.S).group(1)
css_vars = dict(re.findall(r"--([a-z0-9-]+)\s*:\s*([^;]+);", root_block))
def token(name):
    return css_vars[name].strip()

B = token("background")
SURFACE = token("surface")
ELEVATED = token("elevated")
BORDER = token("border")
BORDER_STRONG = token("border-strong")
ACCENT = token("accent")
ACCENT2 = token("accent-secondary")
SUCCESS = token("success")
INFO = token("info")
RARE = token("rare")
TEXT = token("text")
MUTED = token("text-muted")

def shift(hexcol, amount):
    """Lighten (amount>0) or darken (amount<0) a #RRGGBB by `amount`/255."""
    h = hexcol.lstrip("#")
    out = []
    for i in range(0, 6, 2):
        v = int(h[i:i + 2], 16) + amount
        out.append(f"{max(0, min(255, v)):02x}")
    return "#" + "".join(out)

# Authoritative token names: the `#[serde(rename = ...)]` attributes on
# ThemeConfigColors. Validating against these catches invented names, which
# serde would otherwise ignore silently (no deny_unknown_fields), leaving a
# component on shadcn's palette while everything looks fine.
SCHEMA = sorted(glob.glob(str(pathlib.Path.home().joinpath(
    ".cargo/registry/src/*/gpui-component-*/src/theme/schema.rs"))))[-1]
_schema_text = pathlib.Path(SCHEMA).read_text()
_block = _schema_text.split("pub struct ThemeConfigColors {", 1)[1]
_block = _block.split("\n}", 1)[0]   # to the struct's closing brace
# A field's JSON name is its `rename` if it has one, else its plain Rust name.
# Two consequences worth knowing: `progress_bar.background` in the reference
# theme matches no field at all and is dropped by serde (there is no
# deny_unknown_fields), so the real key is `progress_bar`; and unknown keys
# fail silently, which is why this set is derived rather than hand-listed.
_renamed = set(re.findall(r'\#\[serde\(rename = "([^"]+)"\)\]', _block))
_plain = set(re.findall(r'^\s*pub (\w+): Option<', _block, re.M))
ALLOWED = _renamed | _plain
if len(ALLOWED) < 200:
    sys.exit(f"only found {len(ALLOWED)} colour tokens in {SCHEMA}")

OVERRIDES = {
    "background": B,
    "foreground": TEXT,
    "border": BORDER,
    "input.border": BORDER_STRONG,
    "ring": ACCENT,

    # Buttons and the primary/secondary/neutral scale. BORROW_'s primary is
    # its accent orange, and its text sits on it in the page background colour
    # — that pairing is what makes .btn-primary read as a BORROW_ button.
    "primary.background": ACCENT,
    "primary.foreground": B,
    "primary.hover.background": shift(ACCENT, 22),
    "primary.active.background": shift(ACCENT, -30),
    "button.background": ACCENT,
    "button.foreground": B,
    "button.hover.background": shift(ACCENT, 22),
    "button.active.background": shift(ACCENT, -30),
    "button.primary.background": ACCENT,
    "button.primary.foreground": B,
    "button.primary.hover.background": shift(ACCENT, 22),
    "button.primary.active.background": shift(ACCENT, -30),

    "secondary.background": SURFACE,
    "secondary.foreground": TEXT,
    "secondary.hover.background": ELEVATED,
    "secondary.active.background": BORDER,
    "button.secondary.background": SURFACE,
    "button.secondary.foreground": TEXT,
    "button.secondary.hover.background": ELEVATED,
    "button.secondary.active.background": BORDER,

    "muted.background": ELEVATED,
    "muted.foreground": MUTED,
    "accent.background": ELEVATED,
    "accent.foreground": TEXT,
    "accordion.background": SURFACE,

    # Semantic states — all four drawn from BORROW_'s own set rather than
    # from a generic red/green/blue, so feedback stays in the palette.
    "success.background": SUCCESS,
    "success.foreground": B,
    "button.success.background": SUCCESS,
    "button.success.foreground": B,
    "button.success.hover.background": shift(SUCCESS, -25),
    "button.success.active.background": shift(SUCCESS, -45),
    "info.background": INFO,
    "info.foreground": B,
    "button.info.background": INFO,
    "button.info.foreground": B,
    "button.info.hover.background": shift(INFO, -25),
    "button.info.active.background": shift(INFO, -45),
    "warning.background": ACCENT2,
    "warning.foreground": B,
    "button.warning.background": ACCENT2,
    "button.warning.foreground": B,
    "button.warning.hover.background": shift(ACCENT2, -25),
    "button.warning.active.background": shift(ACCENT2, -45),
    "danger.background": ACCENT,
    "danger.foreground": B,
    "button.danger.background": ACCENT,
    "button.danger.foreground": B,
    "button.danger.hover.background": shift(ACCENT, 22),
    "button.danger.active.background": shift(ACCENT, -30),

    "progress_bar": ELEVATED,
    # rgba(11, 10, 8, 0.82) — the modal scrim in .playground::backdrop.
    "overlay": B + "D1",

    "window.border": BORDER_STRONG,
    "title_bar.background": SURFACE,
    "title_bar.border": BORDER,
    "status_bar.background": SURFACE,
    "status_bar.border": BORDER,
    "tab_bar.background": SURFACE,
    "tab_bar.segmented.background": ELEVATED,
    "tab.background": B + "00",
    "tab.foreground": MUTED,
    "tab.active.background": B,
    "tab.active.foreground": TEXT,

    "table.background": SURFACE,
    "table.head.background": B,
    "table.head.foreground": MUTED,
    "table.row.border": BORDER,
    "table.even.background": SURFACE + "66",
    "table.hover.background": ELEVATED,
    "table.active.background": ACCENT + "33",
    "table.active.border": ACCENT,

    "list.background": SURFACE,
    "list.head.background": B,
    "list.even.background": SURFACE + "66",
    "list.hover.background": ELEVATED,
    "list.active.background": ACCENT + "33",
    "list.active.border": ACCENT,
    "popover.background": SURFACE,
    "popover.foreground": TEXT,
    "sidebar.background": SURFACE,
    "sidebar.foreground": TEXT,
    "sidebar.border": BORDER,
    "sidebar.accent.background": ELEVATED,
    "sidebar.accent.foreground": TEXT,
    "sidebar.primary.background": ACCENT,
    "sidebar.primary.foreground": B,

    "link": INFO,
    "link_active": TEXT,
    "link_hover": ACCENT,
    "selection.background": ACCENT + "4D",
    "caret": TEXT,

    "scrollbar.background": B + "00",
    "scrollbar.thumb.background": BORDER_STRONG + "E6",
    "scrollbar.thumb.hover.background": MUTED,
    "skeleton.background": ELEVATED,
    "slider_bar": BORDER,
    "slider.thumb.background": ACCENT,
    "switch.background": BORDER_STRONG,
    "drop_target.background": ACCENT + "40",
    "drag_border": ACCENT,
    "group_box.background": SURFACE,
    "group_box.foreground": TEXT,
    "description_list_label_foreground": MUTED,
    "description_list_label": SURFACE,

    # Syntax palette: derived only from BORROW_ tokens, so a code panel and
    # its surrounding chrome are recognisably the same design.
    # (syntax colours are written below, under `highlight`.)
    "base.red": ACCENT,
    "base.red.light": shift(ACCENT, 40),
    "base.green": SUCCESS,
    "base.green.light": shift(SUCCESS, 0),
    "base.blue": INFO,
    "base.blue.light": shift(INFO, 0),
    "base.yellow": ACCENT2,
    "base.yellow.light": shift(ACCENT2, 30),
    "base.magenta": RARE,
    "base.magenta.light": shift(RARE, 30),
    "base.cyan": INFO,
    "base.cyan.light": shift(INFO, 0),
}

SYNTAX = {
    "keyword": ACCENT,
    "keyword.control": ACCENT,
    "type": INFO,
    "type.builtin": INFO,
    "constructor": INFO,
    "function": TEXT,
    "function.macro": ACCENT2,
    "string": SUCCESS,
    "string.escape": SUCCESS,
    "string.regex": SUCCESS,
    "string.special": SUCCESS,
    "string.special.symbol": SUCCESS,
    "number": ACCENT2,
    "constant": ACCENT2,
    "boolean": ACCENT2,
    "comment": MUTED,
    "comment.doc": MUTED,
    "embedded": TEXT,
    "property": TEXT,
    "variable": TEXT,
    "variable.special": INFO,
    "attribute": INFO,
    "tag": ACCENT,
    "title": ACCENT2,
    "title.function": TEXT,
    "text.literal": SUCCESS,
    "text.code.span": SUCCESS,
    "link_text": INFO,
    "link_uri": INFO,
    "emphasis": {"font_style": "italic"},
    "emphasis.strong": {"font_weight": 700},
}

def build():
    dark = next(t for t in ref["themes"] if t["mode"] == "dark")

    colors = dict(dark["colors"])
    bogus = [k for k in OVERRIDES if k not in ALLOWED]
    if bogus:
        sys.exit(f"not real colour tokens: {bogus}")
    colors.update(OVERRIDES)

    highlight = dict(dark.get("highlight") or {})
    if "syntax" in highlight:
        syntax = dict(highlight["syntax"])
        for k, v in SYNTAX.items():
            syntax[k] = v if isinstance(v, dict) else {"color": v}
        highlight["syntax"] = syntax
    # Editor chrome is the lesson code panel: page background, not a lighter
    # canvas, so code sits flush with everything around it.
    highlight["editor.foreground"] = TEXT
    highlight["editor.background"] = B
    highlight["editor.line_number"] = MUTED
    highlight["editor.active_line.background"] = SURFACE
    highlight["editor.active_line_number"] = TEXT
    highlight["error.background"] = ACCENT + "33"
    highlight["error.border"] = ACCENT
    highlight["warning.background"] = ACCENT2 + "33"
    highlight["warning.border"] = ACCENT2
    highlight["success.background"] = SUCCESS + "33"
    highlight["hint.background"] = INFO + "33"
    highlight["hint.border"] = INFO
    highlight["info.background"] = INFO + "33"
    highlight["created.background"] = SUCCESS + "33"
    highlight["deleted.background"] = ACCENT + "33"
    highlight["conflict"] = ACCENT
    highlight["created"] = INFO
    highlight["modified"] = ACCENT2
    highlight["hidden"] = MUTED
    highlight["predictive"] = BORDER_STRONG
    highlight["hint"] = RARE

    theme = {
        "is_default": True,
        "name": "BORROW_",
        "mode": "dark",
        # Radii are taken from the design tokens rather than gpui-component's
        # defaults: BORROW_ is a 4/6/8 scale, so it is slightly sharper than
        # shadcn's 6/8.
        "radius": 4,
        "radius.lg": 8,
        # No `font.family` on purpose. IBM Plex Mono is not bundled yet
        # (`assets/fonts/` is empty), and gpui uses an explicitly named family
        # as-is rather than falling back — naming an absent font is how text
        # ends up laying out with no glyphs. The web app has the same gap and
        # falls back the same way. Bundling lands with packaging.
        "shadow": False,
        "colors": colors,
        "highlight": highlight,
    }

    return {
        "$schema": "https://github.com/longbridge/gpui-kit/raw/refs/heads/main/.theme-schema.json",
        "name": "BORROW_",
        "author": "BORROW_",
        "themes": [theme],
    }

if __name__ == "__main__":
    out = REPO / "src" / "bin" / "app" / "borrow.theme.json"
    out.write_text(json.dumps(build(), indent=2) + "\n")
    print(f"wrote {out.relative_to(REPO)}")
