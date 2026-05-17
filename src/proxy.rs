use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Duration;

use ratatui::{
    layout::Alignment,
    style::Style,
    text::{Line, Span},
};

use crate::adb::Adb;
use crate::theme;

pub fn proxy_bar(proxy: String) -> Line<'static> {
    let t = theme::current();

    Line::from(vec![
        Span::styled(format!(" {proxy} "), Style::new().fg(t.accent)),
    ])
    .alignment(Alignment::Right)
}

pub fn spawn_poller(
    adb: Arc<dyn Adb>,
    serial: String,
    connectivity: Arc<AtomicBool>,
) -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let interval = Duration::from_secs(1);
        loop {
            if connectivity.load(Ordering::Relaxed)
                && let Ok(proxy) = adb.get_system_proxy(&serial)
                && tx.send(proxy).is_err()
            {
                return;
            }
            std::thread::sleep(interval);
        }
    });
    rx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn battery_color_red_below_10() {
        let t = theme::current();
        assert_eq!(battery_color(0), t.danger);
        assert_eq!(battery_color(9), t.danger);
    }

    #[test]
    fn battery_color_yellow_10_to_25() {
        let t = theme::current();
        assert_eq!(battery_color(10), t.warning);
        assert_eq!(battery_color(25), t.warning);
    }

    #[test]
    fn battery_color_normal_above_25() {
        let t = theme::current();
        assert_eq!(battery_color(26), t.fg);
        assert_eq!(battery_color(100), t.fg);
    }
}
