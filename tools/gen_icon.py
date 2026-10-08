#!/usr/bin/env python3
"""The app icon: BORROW_'s monogram on the page background.

Writes `assets/icon/borrow-512.png`, the single source the installers are
built from — cargo-packager turns it into the .icns macOS wants and the .ico
Windows wants, and ships it as-is for the Linux hicolor theme.

Deliberately the same recipe as the theme: one file in the repo generates the
artwork, so a palette change is a script run rather than a hunt through
binaries. Run it after editing anything below:

    python3 tools/gen_icon.py
"""

from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

REPO = Path(__file__).resolve().parent.parent
FONT = REPO / "assets" / "fonts" / "IBMPlexMono-Bold.ttf"
OUT = REPO / "assets" / "icon" / "borrow-512.png"

SIZE = 512
RADIUS = 108

# From static/style.css: the page background and the accent, so the icon
# sits in the same palette as the window it stands for.
BACKGROUND = "#0B0A08"
ACCENT = "#FF6B35"
MUTED = "#A9A197"


def main() -> None:
    image = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(image)

    draw.rounded_rectangle([0, 0, SIZE - 1, SIZE - 1], radius=RADIUS, fill=BACKGROUND)

    # A keyline in the accent so the mark still reads as BORROW_ on a light
    # dock or a white taskbar, where a dark square alone would disappear.
    inset = 16
    draw.rounded_rectangle(
        [inset, inset, SIZE - 1 - inset, SIZE - 1 - inset],
        radius=RADIUS - inset,
        outline=ACCENT,
        width=8,
    )

    font = ImageFont.truetype(str(FONT), 240)
    caret_font = ImageFont.truetype(str(FONT), 240)

    letter = "B"
    underscore = "_"
    letter_box = draw.textbbox((0, 0), letter, font=font)
    underscore_box = draw.textbbox((0, 0), underscore, font=caret_font)
    width = (letter_box[2] - letter_box[0]) + (underscore_box[2] - underscore_box[0])
    height = max(letter_box[3] - letter_box[1], underscore_box[3] - underscore_box[1])

    x = (SIZE - width) / 2
    y = (SIZE - height) / 2 - letter_box[1]

    # The B carries the accent; the cursor that gives the wordmark its
    # underscore is muted, the way a caret reads on a terminal.
    draw.text((x, y), letter, font=font, fill=ACCENT)
    draw.text(
        (x + (letter_box[2] - letter_box[0]), y),
        underscore,
        font=caret_font,
        fill=MUTED,
    )

    OUT.parent.mkdir(parents=True, exist_ok=True)
    image.save(OUT)
    print(f"wrote {OUT.relative_to(REPO)}")


if __name__ == "__main__":
    main()
