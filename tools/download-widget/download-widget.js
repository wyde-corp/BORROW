/*! borrow download widget — MIT-free, no deps. See INTEGRATION.md */
(function () {
  "use strict";

  var DEFAULT_REPO = "Gus-D-Goat/BORROW";
  var TAG = "[borrow-download]";

  function config() {
    var c = window.BORROW_DOWNLOAD_CONFIG || {};
    return {
      repo: c.repo || DEFAULT_REPO,
      apiBase: c.apiBase || "https://api.github.com",
      autoInit: c.autoInit !== false,
    };
  }

  function classify(name) {
    var n = name.toLowerCase();
    if (n.slice(-4) === ".dmg") return "macos";
    if (n.slice(-4) === ".exe") return "windows";
    if (n.slice(-9) === ".appimage") return "linux-appimage";
    if (n.slice(-4) === ".deb") return "linux-deb";
    if (n === "sha256sums") return "sha256sums";
    return null;
  }

  function detectOS() {
    var ua = navigator.userAgent || "";
    var platform =
      (navigator.userAgentData && navigator.userAgentData.platform) ||
      navigator.platform ||
      "";
    var s = (platform + " " + ua).toLowerCase();
    if (/android|iphone|ipad|ipod|mobile/.test(s)) return null;
    if (/mac|darwin/.test(s)) return "macos";
    if (/win/.test(s)) return "windows";
    if (/linux|x11|cros/.test(s)) return "linux";
    return null;
  }

  function recommendedKey(os) {
    if (os === "macos") return "macos";
    if (os === "windows") return "windows";
    if (os === "linux") return "linux-appimage";
    return null;
  }

  var LABELS = {
    macos: "Download for macOS",
    windows: "Download for Windows",
    "linux-appimage": "Download for Linux",
    "linux-deb": "Linux (.deb)",
  };

  var NOTES = {
    macos:
      "If macOS says the app is from an unidentified developer: right-click the app → Open.",
    windows: "If Windows SmartScreen appears: More info → Run anyway.",
  };

  var STYLESHEET =
    ".bdl{font:inherit;color:inherit}" +
    ".bdl-btn{display:inline-block;padding:.75rem 1.5rem;border-radius:.5rem;" +
    "background:#111;color:#fff;text-decoration:none;font-weight:600;" +
    "line-height:1.2}" +
    ".bdl-btn:hover{opacity:.85}" +
    ".bdl-note{margin:.5rem 0 0;font-size:.8rem;opacity:.65;max-width:28rem}" +
    ".bdl-other{margin-top:1rem;font-size:.85rem;opacity:.85}" +
    ".bdl-other a{margin-right:1rem;white-space:nowrap}" +
    ".bdl-verify{margin:.75rem 0 0;font-size:.75rem;opacity:.55}" +
    ".bdl-verify a{color:inherit}";

  function ensureStylesheet() {
    if (document.getElementById("borrow-download-styles")) return;
    var style = document.createElement("style");
    style.id = "borrow-download-styles";
    style.textContent = STYLESHEET;
    document.head.appendChild(style);
  }

  function collect() {
    var cfg = config();
    var url =
      cfg.apiBase +
      "/repos/" +
      cfg.repo +
      "/releases/latest";
    return fetch(url, { headers: { Accept: "application/vnd.github+json" } })
      .then(function (res) {
        if (!res.ok) throw new Error("HTTP " + res.status);
        return res.json();
      })
      .then(function (release) {
        var links = { version: (release.tag_name || "").replace(/^v/, "") };
        (release.assets || []).forEach(function (asset) {
          var key = classify(asset.name || "");
          if (key) links[key] = asset.browser_download_url;
        });
        return links;
      });
  }

  function fallbackLinks() {
    var cfg = config();
    var page = "https://github.com/" + cfg.repo + "/releases/latest";
    return {
      version: null,
      fallback: true,
      page: page,
      macos: page,
      windows: page,
      "linux-appimage": page,
      "linux-deb": page,
    };
  }

  function applyLinks(links) {
    var rec = recommendedKey(detectOS());
    var els = document.querySelectorAll("[data-borrow-dl]");
    Array.prototype.forEach.call(els, function (el) {
      var key = el.getAttribute("data-borrow-dl");
      if (links[key]) el.setAttribute("href", links[key]);
      if (rec && key === rec) el.setAttribute("data-recommended", "true");
    });
    Array.prototype.forEach.call(
      document.querySelectorAll("[data-borrow-version]"),
      function (el) {
        if (links.version) el.textContent = links.version;
      }
    );
    return rec;
  }

  function autoRender(container, links) {
    ensureStylesheet();
    var os = detectOS();
    var rec = recommendedKey(os);
    container.classList.add("bdl");
    container.innerHTML = "";

    var primaryKey = rec || null;
    if (primaryKey && links[primaryKey]) {
      var btn = document.createElement("a");
      btn.className = "bdl-btn";
      btn.setAttribute("data-borrow-dl", primaryKey);
      btn.setAttribute("href", links[primaryKey]);
      btn.textContent = LABELS[primaryKey];
      container.appendChild(btn);

      if (NOTES[primaryKey]) {
        var note = document.createElement("p");
        note.className = "bdl-note";
        note.textContent = NOTES[primaryKey];
        container.appendChild(note);
      }
    }

    var order = ["macos", "windows", "linux-appimage", "linux-deb"];
    var others = order.filter(function (k) {
      return k !== primaryKey && links[k];
    });
    if (others.length) {
      var wrap = document.createElement("p");
      wrap.className = "bdl-other";
      if (!primaryKey) {
        var lead = document.createElement("span");
        lead.textContent = "Download BORROW_: ";
        wrap.appendChild(lead);
      } else {
        var label = document.createElement("span");
        label.textContent = "Other platforms: ";
        wrap.appendChild(label);
      }
      others.forEach(function (k) {
        var a = document.createElement("a");
        a.setAttribute("data-borrow-dl", k);
        a.setAttribute("href", links[k]);
        a.textContent = LABELS[k];
        wrap.appendChild(a);
      });
      container.appendChild(wrap);
    }

    if (links.sha256sums) {
      var verify = document.createElement("p");
      verify.className = "bdl-verify";
      var va = document.createElement("a");
      va.setAttribute("href", links.sha256sums);
      va.textContent = "Verify download (SHA-256)";
      verify.appendChild(va);
      container.appendChild(verify);
    }

    if (links.fallback) {
      var fb = document.createElement("p");
      fb.className = "bdl-note";
      fb.textContent =
        "Download from the GitHub releases page.";
      container.appendChild(fb);
    }

    applyLinks(links);
  }

  function run() {
    var own = document.querySelectorAll("[data-borrow-dl]").length > 0;
    var container = own ? null : document.getElementById("borrow-download");

    if (!container && !own) return;

    collect()
      .catch(function () {
        return fallbackLinks();
      })
      .then(function (links) {
        if (container) autoRender(container, links);
        else applyLinks(links);

        document.dispatchEvent(
          new CustomEvent("borrow:download:ready", {
            detail: { links: links, detectedOS: detectOS() },
          })
        );
      });
  }

  function init() {
    if (document.readyState === "loading") {
      document.addEventListener("DOMContentLoaded", run);
    } else {
      run();
    }
  }

  window.BorrowDownload = { init: init, run: run, detectOS: detectOS };

  if (config().autoInit) init();
})();
