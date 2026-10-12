/* BORROW_ download site — main.js
   starfield · sparkle cursor · OS detection · hit counter
   all vanilla, no deps, no build step */

(() => {
  "use strict";

  const reduceMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;

  /* ======================================================================
     1. STARFIELD  (with occasional shooting stars)
     ====================================================================== */
  const canvas = document.getElementById("starfield");
  if (canvas) {
    const ctx = canvas.getContext("2d");
    let w = 0, h = 0, dpr = 1;
    let stars = [];
    let shooters = [];
    let raf = 0;

    const COLORS = ["#F1EDE6", "#F1EDE6", "#F1EDE6", "#D9CFC0", "#FFB08A"];

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
      const count = Math.floor((w * h) / 5200);
      stars = Array.from({ length: count }, () => ({
        x: Math.random() * w,
        y: Math.random() * h,
        r: Math.random() * 1.4 + 0.3,
        c: COLORS[(Math.random() * COLORS.length) | 0],
        phase: Math.random() * Math.PI * 2,
        speed: 0.4 + Math.random() * 1.4,
      }));
      shooters = [];
    }

    function drawStatic() {
      ctx.clearRect(0, 0, w, h);
      for (const s of stars) {
        ctx.globalAlpha = 0.55;
        ctx.fillStyle = s.c;
        ctx.fillRect(s.x, s.y, s.r * 1.6, s.r * 1.6);
      }
      ctx.globalAlpha = 1;
    }

    function spawnShooter() {
      const fromLeft = Math.random() < 0.5;
      shooters.push({
        x: fromLeft ? -20 : w + 20,
        y: Math.random() * h * 0.5,
        vx: fromLeft ? 7 + Math.random() * 5 : -(7 + Math.random() * 5),
        vy: 1.6 + Math.random() * 1.6,
        life: 1,
      });
    }

    let last = 0;
    function frame(t) {
      const dt = Math.min((t - last) / 16.67, 3) || 1;
      last = t;

      ctx.clearRect(0, 0, w, h);

      for (const s of stars) {
        s.phase += 0.03 * s.speed * dt;
        const tw = 0.45 + 0.55 * (0.5 + 0.5 * Math.sin(s.phase));
        ctx.globalAlpha = tw;
        ctx.fillStyle = s.c;
        ctx.fillRect(s.x, s.y, s.r * 1.8, s.r * 1.8);
      }
      ctx.globalAlpha = 1;

      for (let i = shooters.length - 1; i >= 0; i--) {
        const sh = shooters[i];
        sh.x += sh.vx * dt;
        sh.y += sh.vy * dt;
        sh.life -= 0.012 * dt;
        if (sh.life <= 0 || sh.x < -80 || sh.x > w + 80) {
          shooters.splice(i, 1);
          continue;
        }
        const grad = ctx.createLinearGradient(sh.x, sh.y, sh.x - sh.vx * 6, sh.y - sh.vy * 6);
        grad.addColorStop(0, "rgba(255,255,255,0.95)");
        grad.addColorStop(1, "rgba(255,255,255,0)");
        ctx.strokeStyle = grad;
        ctx.lineWidth = 1.6;
        ctx.beginPath();
        ctx.moveTo(sh.x, sh.y);
        ctx.lineTo(sh.x - sh.vx * 6, sh.y - sh.vy * 6);
        ctx.stroke();
      }

      if (Math.random() < 0.0018 && shooters.length < 1) spawnShooter();

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
     2. SPARKLE CURSOR TRAIL
     ====================================================================== */
  if (!reduceMotion) {
    const SPARK_COLORS = ["#F1EDE6", "#FFB08A", "#C9A55C"];
    let lastX = 0, lastY = 0, lastT = 0, live = 0;
    const MAX_LIVE = 16;

    window.addEventListener("pointermove", (e) => {
      if (e.pointerType === "touch") return;
      const now = performance.now();
      const dx = e.clientX - lastX;
      const dy = e.clientY - lastY;
      if (now - lastT < 90 && dx * dx + dy * dy < 160) return;
      lastX = e.clientX; lastY = e.clientY; lastT = now;
      if (live >= MAX_LIVE) return;

      const el = document.createElement("div");
      el.className = "sparkle";
      el.style.left = e.clientX + "px";
      el.style.top = e.clientY + "px";
      el.style.background = SPARK_COLORS[(Math.random() * SPARK_COLORS.length) | 0];
      el.style.width = el.style.height = 4 + Math.random() * 5 + "px";
      document.body.appendChild(el);
      live++;
      el.addEventListener("animationend", () => { el.remove(); live--; }, { once: true });
      setTimeout(() => { if (el.isConnected) { el.remove(); live--; } }, 900);
    }, { passive: true });
  }

  /* ======================================================================
     3. DOWNLOAD WIRING — driven by download-widget's ready event
     (widget owns hrefs + OS detection; we own the visuals around it)
     ====================================================================== */
  const osStatus = document.getElementById("os-status");
  const shaLink = document.getElementById("sha-link");
  const versionBadge = document.getElementById("version-badge");
  const heroVersion = document.getElementById("hero-version");

  function isMobile() {
    return /Android|iPhone|iPad|iPod|Mobile/i.test(navigator.userAgent || "");
  }

  function onDownloadsReady(e) {
    const links = (e && e.detail && e.detail.links) || {};
    const detectedOS = e && e.detail ? e.detail.detectedOS : null;
    const cardFor = { macos: "mac", windows: "windows", linux: "linux" }[detectedOS] || null;

    const notes = {
      mac: "⌁ 検出: macOS — ユニバーサル .dmg をどうぞ。Apple Silicon / Intel 両対応。",
      windows: "⌁ 検出: Windows — x64 インストーラーをどうぞ。ARM 版はエミュレーションで動きます。",
      linux: "⌁ 検出: Linux — AppImage(推奨)か .deb をどうぞ。ARM ビルドはまだありません。",
    };
    if (osStatus) {
      if (cardFor) osStatus.textContent = notes[cardFor];
      else if (isMobile()) osStatus.textContent = "⌁ borrow_ はデスクトップアプリです — PC でお試しください。";
      else osStatus.textContent = "お使いのOSを検出できませんでした — 下からお選びください。";
    }

    if (cardFor) {
      const card = document.querySelector('[data-os-card="' + cardFor + '"]');
      if (card) {
        card.classList.add("is-detected");
        const ribbon = card.querySelector(".os-ribbon");
        if (ribbon) ribbon.hidden = false;
        const primaryFile = { mac: "dmg", windows: "exe", linux: "appimage" }[cardFor];
        card.querySelectorAll(".dl-btn").forEach((btn) => {
          btn.classList.toggle("is-picked", btn.dataset.file === primaryFile);
        });
      }
    }

    if (shaLink && links.sha256sums) shaLink.href = links.sha256sums;

    if (links.version) {
      if (heroVersion) heroVersion.textContent = links.version;
      if (versionBadge) {
        versionBadge.textContent = "";
        const span = document.createElement("span");
        span.textContent = "VERSION " + links.version;
        versionBadge.appendChild(span);
        versionBadge.appendChild(document.createTextNode(" · LATEST STABLE 最新安定版"));
      }
    }
    // On fallback (version null) the static prerelease badge stays — honest,
    // since the rc is the newest thing that exists.
  }

  document.addEventListener("borrow:download:ready", onDownloadsReady);

  /* ======================================================================
     4. HIT COUNTER (odometer roll-up)
     ====================================================================== */
  const counterEl = document.getElementById("hit-counter");
  if (counterEl) {
    const TARGET = 427;
    const digits = [...counterEl.querySelectorAll("span")];
    const pad = String(TARGET).padStart(digits.length, "0");

    if (reduceMotion) {
      pad.split("").forEach((d, i) => { digits[i].textContent = d; });
    } else {
      const start = performance.now();
      const DUR = 1600;
      function roll(t) {
        const p = Math.min((t - start) / DUR, 1);
        // ease-out cubic
        const e = 1 - Math.pow(1 - p, 3);
        const val = Math.floor(e * TARGET);
        const s = String(val).padStart(digits.length, "0");
        s.split("").forEach((d, i) => { digits[i].textContent = d; });
        if (p < 1) requestAnimationFrame(roll);
        else pad.split("").forEach((d, i) => { digits[i].textContent = d; });
      }
      requestAnimationFrame(roll);
    }
  }

  /* ======================================================================
     5. STAT COUNT-UP (pitch section)
     ====================================================================== */
  const statNums = document.querySelectorAll(".stat-num[data-count]");
  if (statNums.length && !reduceMotion && "IntersectionObserver" in window) {
    const io = new IntersectionObserver((entries) => {
      for (const entry of entries) {
        if (!entry.isIntersecting) continue;
        const el = entry.target;
        io.unobserve(el);
        const target = parseInt(el.dataset.count, 10) || 0;
        const t0 = performance.now();
        const DUR = 1100;
        (function step(t) {
          const p = Math.min((t - t0) / DUR, 1);
          const e = 1 - Math.pow(1 - p, 3);
          el.textContent = String(Math.floor(e * target));
          if (p < 1) requestAnimationFrame(step);
          else el.textContent = String(target);
        })(t0);
      }
    }, { threshold: 0.4 });
    statNums.forEach((el) => io.observe(el));
  } else {
    statNums.forEach((el) => { el.textContent = el.dataset.count; });
  }

  /* ======================================================================
     6. TODAY'S DATE (jp-style, topbar)
     ====================================================================== */
  const dateEl = document.getElementById("today-date");
  if (dateEl) {
    try {
      const d = new Date();
      dateEl.textContent =
        "本日は " + d.getFullYear() + "年" + (d.getMonth() + 1) + "月" + d.getDate() + "日 です";
    } catch (_) { /* keep fallback text */ }
  }
})();
