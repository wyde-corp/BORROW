use super::layout::page;

pub fn show() -> String {
    let body = r##"<section class="hero">
        <h1>LEARN RUST</h1>
        <h2>WITHOUT THE 700-PAGE DETOUR.</h2>
        <p>Short lessons. Real practice. Small projects. Learn the fundamentals by actually writing Rust.</p>
        <div class="hero-actions">
            <a href="/dashboard" class="btn btn-primary">START LEARNING</a>
            <a href="/dashboard" class="btn btn-secondary">VIEW CURRICULUM</a>
        </div>
    </section>

    <section class="stats">
        <div class="stat"><span class="stat-value">11</span><span class="stat-label">MODULES</span></div>
        <div class="stat"><span class="stat-value">40+</span><span class="stat-label">EXERCISES</span></div>
        <div class="stat"><span class="stat-value">5</span><span class="stat-label">PROJECTS</span></div>
    </section>"##;

    page("", "Learn Rust", body)
}
