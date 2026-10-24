/* BORROW_ download site — main.js
   dimmed starfield · OS-aware CTA · sticky mobile download bar
   all vanilla, no deps, no build step */

(() => {
  "use strict";

  const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  /* ======================================================================
     1. STARFIELD — the dark void of the gilded CRT. Dim, quiet, no shooters.
     ====================================================================== */
  const canvas = document.getElementById("starfield");
  if (canvas) {
    const ctx = canvas.getContext("2d");
    let w = 0, h = 0, dpr = 1;
    let stars = [];
    let raf = 0;

    const COLORS = ["#F1EDE6", "#F1EDE6", "#F1EDE6", "#D9CFC0", "#C9A55C"];

    function resize() {
      dpr = Math.min(window.devicePixelRatio || 1, 2);
      w = window.innerWidth;
      h = window.innerHeight;
      canvas.width = Math.floor(w * dpr);
      canvas.height = Math.floor(h * dpr);
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      seed();
      if (reduceMotion) drawStatic();
    }

    function seed() {
      const count = Math.floor((w * h) / 8400);
      stars = Array.from({ length: count }, () => ({
        x: Math.random() * w,
        y: Math.random() * h,
        r: Math.random() * 1.2 + 0.3,
        c: COLORS[(Math.random() * COLORS.length) | 0],
        phase: Math.random() * Math.PI * 2,
        speed: 0.4 + Math.random() * 1.2,
      }));
    }

    function drawStatic() {
      ctx.clearRect(0, 0, w, h);
      for (const s of stars) {
        ctx.globalAlpha = 0.4;
        ctx.fillStyle = s.c;
        ctx.fillRect(s.x, s.y, s.r * 1.6, s.r * 1.6);
      }
      ctx.globalAlpha = 1;
    }

    let last = 0;
    function frame(t) {
      const dt = Math.min((t - last) / 16.67, 3) || 1;
      last = t;

      ctx.clearRect(0, 0, w, h);

      for (const s of stars) {
        s.phase += 0.025 * s.speed * dt;
        const tw = 0.45 + 0.55 * (0.5 + 0.5 * Math.sin(s.phase));
        ctx.globalAlpha = tw * 0.6;
        ctx.fillStyle = s.c;
        ctx.fillRect(s.x, s.y, s.r * 1.6, s.r * 1.6);
      }
      ctx.globalAlpha = 1;

      raf = requestAnimationFrame(frame);
    }

    resize();
    window.addEventListener("resize", resize);
    if (!reduceMotion) raf = requestAnimationFrame(frame);

    document.addEventListener("visibilitychange", () => {
      if (reduceMotion) return;
      if (document.hidden) {
        cancelAnimationFrame(raf);
      } else {
        last = performance.now();
        raf = requestAnimationFrame(frame);
      }
    });
  }

  /* ======================================================================
     2. DOWNLOAD WIRING — driven by download-widget's ready event
     (widget owns href rewriting on [data-borrow-dl]; we own the hero CTA's
      label + destination, the version stamp, and the mobile fallback)
     ====================================================================== */
  const LABELS = {
    macos: "Download for macOS",
    windows: "Download for Windows",
    linux: "Download for Linux",
  };

  const ctas = document.querySelectorAll(".js-cta");
  const stickyCta = document.getElementById("sticky-cta");
  const versionEls = document.querySelectorAll("#hero-version, [data-borrow-version]");
  const osStatus = document.getElementById("os-status");

  function isMobile() {
    return /Android|iPhone|iPad|iPod|Mobile/i.test(navigator.userAgent || "");
  }

  function setLabel(el, text) {
    const label = el && el.querySelector(".cta-label");
    if (label) label.textContent = text;
  }

  function setAllLabels(text) {
    ctas.forEach((el) => setLabel(el, text));
  }

  function setAllHrefs(href) {
    ctas.forEach((el) => { el.href = href; });
  }

  const PLATFORMS = { macos: "macOS", windows: "Windows", linux: "Linux" };

  function renderAltRow(primary, links, releasesPage) {
    const alt = document.getElementById("cta-alt");
    if (!alt) return;
    const keys = Object.keys(PLATFORMS).filter((k) => !primary || k !== primary);
    alt.textContent = "";
    alt.append(primary ? "Also for " : "Available for ");
    keys.forEach((k, i) => {
      if (i) alt.append(document.createTextNode(" · "));
      const a = document.createElement("a");
      a.setAttribute("data-borrow-dl", k);
      a.href = links[k] || releasesPage;
      a.textContent = PLATFORMS[k];
      alt.appendChild(a);
    });
  }

  function onDownloadsReady(e) {
    const links = (e && e.detail && e.detail.links) || {};
    const detectedOS = e && e.detail ? e.detail.detectedOS : null;
    const releasesPage = links.page || "https://github.com/wyde-corp/BORROW/releases/latest";

    if (links.version) {
      versionEls.forEach((el) => { el.textContent = links.version; });
    }

    let primaryKey = null;
    if (detectedOS === "macos") primaryKey = "macos";
    else if (detectedOS === "windows") primaryKey = "windows";
    else if (detectedOS === "linux") primaryKey = "linux";

    if (primaryKey) {
      setAllLabels(LABELS[primaryKey]);
      if (links[primaryKey]) setAllHrefs(links[primaryKey]);
      renderAltRow(primaryKey, links, releasesPage);
    } else if (isMobile()) {
      // no desktop OS here — point everyone at the releases page, honestly
      ctas.forEach((el) => setLabel(
        el,
        el.classList.contains("cta--compact") ? "Get the app" : "Get the desktop app"
      ));
      setAllHrefs(releasesPage);
      renderAltRow(null, links, releasesPage);
      if (osStatus) osStatus.textContent = "BORROW_ is a desktop app for macOS, Windows, and Linux.";
    }
    // unknown desktop OS: leave the static defaults (they already read right)
  }

  document.addEventListener("borrow:download:ready", onDownloadsReady);

  /* ======================================================================
     3. STICKY MOBILE DOWNLOAD BAR — appears once the hero has left the view
     ====================================================================== */
  if (stickyCta && "IntersectionObserver" in window) {
    const narrow = window.matchMedia("(max-width: 60rem)");
    const hero = document.querySelector(".hero");

    const io = new IntersectionObserver((entries) => {
      for (const entry of entries) {
        const show = narrow.matches && !entry.isIntersecting;
        stickyCta.hidden = false;
        stickyCta.classList.toggle("is-visible", show);
      }
    }, { threshold: 0 });

    if (hero) io.observe(hero);
    narrow.addEventListener("change", () => {
      if (!narrow.matches) stickyCta.classList.remove("is-visible");
    });
  }
})();
