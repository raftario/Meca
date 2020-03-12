#[macro_export]
macro_rules! with_progress {
    ($f:expr, $m:expr) => {{
        let p = indicatif::ProgressBar::new_spinner();
        p.set_message($m);
        p.enable_steady_tick(250);

        let result = $f;

        p.finish_and_clear();
        result
    }};
}
