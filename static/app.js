document.addEventListener('click', function (e) {
    const btn = e.target.closest('[data-copy]');
    if (!btn) return;
    const pre = btn.closest('.code-block').querySelector('code');
    navigator.clipboard.writeText(pre.textContent);
    btn.textContent = 'COPIED';
    setTimeout(() => { btn.textContent = 'COPY'; }, 1200);
});

// ── Practice run picker ─────────────────────────────────────────────────
//
// Everything here is optional. Without JS the checkboxes still work, the count
// still says how many exist, and a hand-picked run still submits. JS only adds
// filtering, a live selected count, and a submit button that knows whether
// anything is ticked. The filters start hidden and are revealed here so they
// never appear as dead buttons on a page with no script.
(function () {
    const picker = document.querySelector('.picker');
    if (!picker) return;

    const rows = Array.from(picker.querySelectorAll('.picker-row'));
    const filters = document.querySelector('[data-picker-filters]');
    const count = picker.querySelector('[data-picker-count]');
    const submit = picker.querySelector('[data-picker-submit]');
    const total = rows.length;

    function selected() {
        return rows.filter(function (r) {
            return r.querySelector('input[type="checkbox"]').checked;
        });
    }

    function refreshCount() {
        if (!count) return;
        const n = selected().length;
        count.textContent = n === 0
            ? total + ' selectable'
            : n + ' of ' + total + ' selected';
        count.classList.toggle('ready', n > 0);
        if (submit) submit.disabled = n === 0;
    }

    if (filters) {
        filters.hidden = false;

        filters.addEventListener('click', function (e) {
            const btn = e.target.closest('[data-filter]');
            if (!btn) return;

            const mode = btn.getAttribute('data-filter');

            filters.querySelectorAll('[data-filter]').forEach(function (b) {
                b.classList.toggle('active', b === btn);
            });

            rows.forEach(function (row) {
                const input = row.querySelector('input[type="checkbox"]');

                if (mode === 'clear') {
                    input.checked = false;
                    row.classList.remove('hidden');
                    return;
                }

                if (mode === 'all') {
                    row.classList.remove('hidden');
                    return;
                }

                const state = row.getAttribute('data-state');
                row.classList.toggle('hidden', state !== mode);
            });

            refreshCount();
        });
    }

    picker.addEventListener('change', refreshCount);
    refreshCount();
})();
