use gloo_net::http::Request;
use shared::{SampleRecord, SampleSummary};
use yew::prelude::*;

const PLOT_WIDTH: u32 = 600;
const PLOT_HEIGHT: u32 = 80;
const WINDOW: usize = 20;

fn gc_content(seq: &[u8]) -> f64 {
    if seq.is_empty() {
        return 0.0;
    }
    let gc = seq
        .iter()
        .filter(|&&b| matches!(b, b'G' | b'C' | b'g' | b'c'))
        .count();
    gc as f64 / seq.len() as f64
}

fn base_counts(seq: &[u8]) -> (usize, usize, usize, usize, usize) {
    let mut a = 0usize;
    let mut c = 0usize;
    let mut g = 0usize;
    let mut t = 0usize;
    let mut other = 0usize;
    for &b in seq {
        match b {
            b'A' | b'a' => a += 1,
            b'C' | b'c' => c += 1,
            b'G' | b'g' => g += 1,
            b'T' | b't' => t += 1,
            _ => other += 1,
        }
    }
    (a, c, g, t, other)
}

/// GC content for every contiguous window of length `window` in `seq`.
fn sliding_window_gc(seq: &[u8], window: usize) -> Vec<f64> {
    if window == 0 || seq.len() < window {
        return Vec::new();
    }
    seq.windows(window).map(gc_content).collect()
}

/// Render a series of GC fractions as an SVG polyline `points` string.
///
/// X spreads evenly across `width`; Y is mapped so that GC=0 is at the
/// bottom and GC=1 is at the top.
fn polyline_points(gcs: &[f64], width: u32, height: u32) -> String {
    let n = gcs.len();
    if n == 0 {
        return String::new();
    }
    let mut points = String::with_capacity(n * 12);
    let denom = if n == 1 { 1.0 } else { (n - 1) as f64 };
    for (i, &gc) in gcs.iter().enumerate() {
        let x = (i as f64 / denom) * width as f64;
        let y = height as f64 - gc * height as f64;
        if i > 0 {
            points.push(' ');
        }
        points.push_str(&format!("{:.1},{:.1}", x, y));
    }
    points
}

#[function_component(App)]
fn app() -> Html {
    let samples = use_state(Vec::<SampleSummary>::new);
    let selected = use_state(|| None::<SampleRecord>);
    let typed = use_state(String::new);

    // Fetch the sample list once on mount.
    {
        let samples = samples.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/api/samples").send().await {
                    if let Ok(data) = resp.json::<Vec<SampleSummary>>().await {
                        samples.set(data);
                    }
                }
            });
            || ()
        });
    }

    // Click a sample -> fetch its full record, drop the sequence into
    // the textarea so all the reactive statistics react to it like any
    // other input.
    let on_pick = {
        let selected = selected.clone();
        let typed = typed.clone();
        Callback::from(move |id: String| {
            let selected = selected.clone();
            let typed = typed.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/samples/{}", id);
                if let Ok(resp) = Request::get(&url).send().await {
                    if let Ok(data) = resp.json::<SampleRecord>().await {
                        typed.set(data.sequence.clone());
                        selected.set(Some(data));
                    }
                }
            });
        })
    };

    let on_input = {
        let typed = typed.clone();
        Callback::from(move |e: InputEvent| {
            let target = e.target_unchecked_into::<web_sys::HtmlTextAreaElement>();
            typed.set(target.value());
        })
    };

    let seq_bytes = typed.as_bytes();
    let length = seq_bytes.len();
    let gc = gc_content(seq_bytes);
    let (a, c, g, t, other) = base_counts(seq_bytes);
    let window_gcs = sliding_window_gc(seq_bytes, WINDOW);

    html! {
        <div>
            <h1>{ "Sequence inspector" }</h1>

            <h2>{ "Pick a sample" }</h2>
            <ul>
                { for samples.iter().map(|s| {
                    let id = s.id.clone();
                    let on_pick = on_pick.clone();
                    html! {
                        <li>
                            <button onclick={move |_| on_pick.emit(id.clone())}>
                                { &s.name }
                            </button>
                        </li>
                    }
                })}
            </ul>

            <h2>{ "...or paste your own" }</h2>
            <textarea
                value={(*typed).clone()}
                oninput={on_input}
                placeholder="paste DNA here (ACGT)"
            />

            <h2>{ "Statistics" }</h2>
            <table>
                <tr><th>{ "length" }</th><td>{ length }</td></tr>
                <tr><th>{ "GC content" }</th><td>{ format!("{:.2}%", gc * 100.0) }</td></tr>
                <tr><th>{ "A" }</th><td>{ a }</td></tr>
                <tr><th>{ "C" }</th><td>{ c }</td></tr>
                <tr><th>{ "G" }</th><td>{ g }</td></tr>
                <tr><th>{ "T" }</th><td>{ t }</td></tr>
                <tr><th>{ "other" }</th><td>{ other }</td></tr>
            </table>

            {
                if !window_gcs.is_empty() {
                    let points = polyline_points(&window_gcs, PLOT_WIDTH, PLOT_HEIGHT);
                    html! {
                        <>
                            <h2>{ format!("GC sliding window (w = {})", WINDOW) }</h2>
                            <svg
                                width={PLOT_WIDTH.to_string()}
                                height={PLOT_HEIGHT.to_string()}
                                viewBox={format!("0 0 {} {}", PLOT_WIDTH, PLOT_HEIGHT)}
                                style="border: 1px solid #ccc; background: white;"
                            >
                                <line
                                    x1="0"
                                    y1={(PLOT_HEIGHT / 2).to_string()}
                                    x2={PLOT_WIDTH.to_string()}
                                    y2={(PLOT_HEIGHT / 2).to_string()}
                                    stroke="#eee"
                                    stroke-width="1"
                                    stroke-dasharray="2,2"
                                />
                                <polyline
                                    points={points}
                                    fill="none"
                                    stroke="#2266aa"
                                    stroke-width="1.5"
                                />
                            </svg>
                        </>
                    }
                } else {
                    html! {
                        <p style="color: #888;">
                            { format!("(sequence must be at least {} bp for the sliding-window plot)", WINDOW) }
                        </p>
                    }
                }
            }

            {
                if let Some(rec) = &*selected {
                    html! {
                        <>
                            <h2>{ format!("Loaded: {}", rec.name) }</h2>
                            <div class="sequence">{ &rec.sequence }</div>
                        </>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
