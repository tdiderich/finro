pub fn get(name: &str) -> Option<&'static str> {
    match name {
        "selectable_grid" => Some(SELECTABLE_GRID),
        "table" => Some(TABLE),
        "tabs" => Some(TABS),
        "accordion" => Some(ACCORDION),
        "deck" => Some(DECK),
        "nav" => Some(NAV),
        "reload" => Some(RELOAD),
        _ => None,
    }
}

const RELOAD: &str = r#"
(function () {
  if (!/^https?:$/.test(location.protocol)) return;
  var last = null;
  function poll() {
    fetch('/__pseudo_version__', { cache: 'no-store' })
      .then(function (r) { return r.ok ? r.text() : null; })
      .then(function (v) {
        if (v === null) return;
        if (last === null) { last = v; return; }
        if (last !== v) location.reload();
      })
      .catch(function () {});
  }
  setInterval(poll, 500);
  poll();
})();
"#;

const NAV: &str = r#"
document.addEventListener('DOMContentLoaded', function () {
  var href = window.location.pathname.split('/').pop() || 'index.html';
  document.querySelectorAll('.nav-link').forEach(function (a) {
    if (a.getAttribute('href') === href) a.classList.add('nav-link-active');
  });
});
"#;

const SELECTABLE_GRID: &str = r#"
document.querySelectorAll('[data-selectable-grid]').forEach(function (grid) {
  var mode = grid.dataset.interaction || 'single_select';
  var dim = grid.dataset.dimOthers !== 'false';
  var selected = new Set();
  function apply() {
    grid.querySelectorAll('.sel-card, .sel-dot').forEach(function (el) {
      var n = el.dataset.n;
      el.classList.remove('sel-active', 'sel-dimmed');
      if (selected.size === 0) return;
      if (selected.has(n)) el.classList.add('sel-active');
      else if (dim) el.classList.add('sel-dimmed');
    });
  }
  grid.querySelectorAll('.sel-card, .sel-dot').forEach(function (el) {
    el.addEventListener('click', function () {
      var n = el.dataset.n;
      if (mode === 'none') return;
      if (mode === 'single_select') {
        if (selected.has(n)) selected.clear();
        else { selected.clear(); selected.add(n); }
      } else {
        if (selected.has(n)) selected.delete(n); else selected.add(n);
      }
      apply();
    });
  });
});
"#;

const TABLE: &str = r#"
document.querySelectorAll('[data-pseudo-table]').forEach(function (table) {
  var tbody = table.tBodies[0];
  var sortState = { col: null, dir: 1 };
  function parse(v) {
    var n = parseFloat(v.replace(/[^0-9.\-]/g, ''));
    return isNaN(n) ? v.toLowerCase() : n;
  }
  table.querySelectorAll('th[data-sortable]').forEach(function (th, i) {
    th.addEventListener('click', function () {
      if (sortState.col === i) sortState.dir = -sortState.dir;
      else { sortState.col = i; sortState.dir = 1; }
      var rows = Array.from(tbody.rows);
      rows.sort(function (a, b) {
        var av = parse(a.cells[i].textContent.trim());
        var bv = parse(b.cells[i].textContent.trim());
        if (av < bv) return -1 * sortState.dir;
        if (av > bv) return 1 * sortState.dir;
        return 0;
      });
      rows.forEach(function (r) { tbody.appendChild(r); });
      table.querySelectorAll('th').forEach(function (h) {
        h.classList.remove('sort-asc', 'sort-desc');
      });
      th.classList.add(sortState.dir === 1 ? 'sort-asc' : 'sort-desc');
    });
  });
  var filterInput = table.parentElement.querySelector('[data-table-filter]');
  if (filterInput) {
    filterInput.addEventListener('input', function () {
      var q = filterInput.value.toLowerCase();
      Array.from(tbody.rows).forEach(function (r) {
        r.style.display = r.textContent.toLowerCase().includes(q) ? '' : 'none';
      });
    });
  }
});
"#;

const TABS: &str = r#"
document.querySelectorAll('[data-tabs]').forEach(function (root) {
  var buttons = root.querySelectorAll('.tab-btn');
  var panels = root.querySelectorAll('.tab-panel');
  function show(i) {
    buttons.forEach(function (b, j) { b.classList.toggle('tab-btn-active', i === j); });
    panels.forEach(function (p, j) { p.style.display = i === j ? '' : 'none'; });
  }
  buttons.forEach(function (b, i) { b.addEventListener('click', function () { show(i); }); });
  show(0);
});
"#;

const ACCORDION: &str = r#"
document.querySelectorAll('[data-accordion-item]').forEach(function (item) {
  var btn = item.querySelector('.accordion-head');
  var body = item.querySelector('.accordion-body');
  body.style.display = 'none';
  btn.addEventListener('click', function () {
    var open = body.style.display !== 'none';
    body.style.display = open ? 'none' : '';
    item.classList.toggle('accordion-open', !open);
  });
});
"#;

const DECK: &str = r#"
(function () {
  var track = document.querySelector('.deck-track');
  var slides = document.querySelectorAll('.deck-slide');
  var label = document.getElementById('deck-label');
  var prev = document.getElementById('deck-prev');
  var next = document.getElementById('deck-next');
  var labels = Array.from(slides).map(function (s) { return s.dataset.label; });
  var current = 0;
  function go(n) {
    current = Math.max(0, Math.min(slides.length - 1, n));
    track.style.transform = 'translateX(-' + (current * 100) + '%)';
    label.textContent = labels[current];
    if (current === 0) { prev.style.visibility = 'hidden'; }
    else { prev.style.visibility = 'visible'; prev.textContent = '← ' + labels[current - 1]; }
    if (current === slides.length - 1) { next.style.visibility = 'hidden'; }
    else { next.style.visibility = 'visible'; next.textContent = labels[current + 1] + ' →'; }
  }
  prev.addEventListener('click', function () { go(current - 1); });
  next.addEventListener('click', function () { go(current + 1); });
  document.addEventListener('keydown', function (e) {
    if (e.key === 'ArrowRight' || e.key === 'ArrowDown') go(current + 1);
    if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') go(current - 1);
  });
  go(0);
})();
"#;
