use alloc::rc::Rc;
use core::time::Duration;

use slint::platform::software_renderer::MinimalSoftwareWindow;
use Mstd::time::{now_timeval, TimeSpec};

pub struct MyPlatform {
    window: Rc<MinimalSoftwareWindow>,
    start_timer: TimeSpec,
}

impl slint::platform::Platform for MyPlatform {
    fn create_window_adapter(
        &self,
    ) -> Result<Rc<dyn slint::platform::WindowAdapter>, slint::PlatformError> {
        Ok(self.window.clone())
    }
    fn duration_since_start(&self) -> Duration {
        let old_time = self.start_timer;
        let now = now_timeval();
        let new_time = TimeSpec::new(now.tv_sec, now.tv_usec * 1000);
        Duration::new(new_time.tv_sec as u64, new_time.tv_nsec as u32)
            - Duration::new(old_time.tv_sec as u64, old_time.tv_nsec as u32)
    }
}

impl MyPlatform {
    pub fn new(window: Rc<MinimalSoftwareWindow>) -> Self {
        let now = now_timeval();
        Self {
            window,
            start_timer: TimeSpec::new(now.tv_sec, now.tv_usec * 1000),
        }
    }
}
