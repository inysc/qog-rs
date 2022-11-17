use log;

const MILLIS_SECONDS_PER_SECOND: u128 = 1000;
const MILLIS_SECONDS_PER_MINUTE: u128 = 60 * 1000;
const MILLIS_SECONDS_PER_HOUR: u128 = 60 * MILLIS_SECONDS_PER_MINUTE;
const MILLIS_SECONDS_PER_DAY: u128 = 24 * MILLIS_SECONDS_PER_HOUR;
const DAYS_PER400_YEARS: u128 = 365 * 400 + 97;
const DAYS_PER100_YEARS: u128 = 365 * 100 + 24;
const DAYS_PER4_YEARS: u128 = 365 * 4 + 1;
const DAYS_BEFORE: [u128; 13] = [
    0,
    31,
    31 + 28,
    31 + 28 + 31,
    31 + 28 + 31 + 30,
    31 + 28 + 31 + 30 + 31,
    31 + 28 + 31 + 30 + 31 + 30,
    31 + 28 + 31 + 30 + 31 + 30 + 31,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30,
    31 + 28 + 31 + 30 + 31 + 30 + 31 + 31 + 30 + 31 + 30 + 31,
];

// 格式：2006-01-02 15:04:05.000
fn now_fmt() -> String {
    milli_fmt(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(std::time::Duration::ZERO)
            .as_millis(),
    )
}

// milli: 相对于 1970-01-01 00:00:00.000 的毫秒数
fn milli_fmt(milli: u128) -> String {
    // 加上 1970-01-01 00:00:00.000 相对于 0000-01-01 00:00:00.000 的毫秒数
    // 转到东八时区（北京）
    let milli = milli + 62135596800000 + 8 * MILLIS_SECONDS_PER_HOUR;
    let mut d = milli / MILLIS_SECONDS_PER_DAY;

    // Account for 400 year cycles.
    let mut n = d / DAYS_PER400_YEARS;
    let mut y = 400 * n;
    d -= DAYS_PER400_YEARS * n;

    // Cut off 100-year cycles.
    // The last cycle has one extra leap year, so on the last day
    // of that year, day / daysPer100Years will be 4 instead of 3.
    // Cut it back down to 3 by subtracting n>>2.
    n = d / DAYS_PER100_YEARS;
    n -= n >> 2;
    y += 100 * n;
    d -= DAYS_PER100_YEARS * n;

    n = d / DAYS_PER4_YEARS;
    y += 4 * n;
    d -= DAYS_PER4_YEARS * n;

    n = d / 365;
    n -= n >> 2;
    y += n;
    d -= 365 * n;

    let year = y + 1;
    let mut day = d;

    if is_leap(year) {
        if day > 31 + 29 - 1 {
            day -= 1;
        } else if day == 31 + 29 - 1 {
            let (hour, min, sec, mils) = clock(milli);
            return format!(
                "{}-02-29 {:0>2}:{:0>2}:{:0>2}.{:0<3}",
                year, hour, min, sec, mils
            );
        }
    }

    let mut month = (day / 31) as usize;
    let end = DAYS_BEFORE[month + 1];

    let begin = if day >= end {
        month += 1;
        end
    } else {
        DAYS_BEFORE[month]
    };

    month += 1; // because January is 1
    day = day - begin + 1;

    let (hour, min, sec, mils) = clock(milli);

    format!(
        "{}-{:0>2}-{:0>2} {:0>2}:{:0>2}:{:0>2}.{:0<3}",
        year, month, day, hour, min, sec, mils
    )
}

#[inline]
fn clock(milli: u128) -> (u128, u128, u128, u128) {
    let mut mils = milli % MILLIS_SECONDS_PER_DAY;
    let hour = mils / MILLIS_SECONDS_PER_HOUR;
    mils -= hour * MILLIS_SECONDS_PER_HOUR;
    let min = mils / MILLIS_SECONDS_PER_MINUTE;
    mils -= min * MILLIS_SECONDS_PER_MINUTE;
    let sec = mils / MILLIS_SECONDS_PER_SECOND;
    mils -= sec * MILLIS_SECONDS_PER_SECOND;

    (hour, min, sec, mils)
}

#[inline]
fn is_leap(year: u128) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

pub struct Qog {
    lvl: log::Level,
}

impl Qog {
    pub fn new(lvl: log::Level, _filename: String) -> Self {
        Qog { lvl }
    }

    pub fn init(self) {
        log::set_max_level(match self.lvl {
            log::Level::Error => log::LevelFilter::Error,
            log::Level::Warn => log::LevelFilter::Warn,
            log::Level::Info => log::LevelFilter::Info,
            log::Level::Debug => log::LevelFilter::Debug,
            log::Level::Trace => log::LevelFilter::Trace,
        });
        log::set_boxed_logger(Box::new(self)).unwrap();
    }

    pub fn default() {
        Self::new(log::Level::Debug, String::from("")).init();
    }
}

impl log::Log for Qog {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.lvl
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        println!(
            "{}|{:5}|{}:{}|{}",
            now_fmt(),
            record.level(),
            record.target(),
            record.line().unwrap_or(0),
            record.args()
        )
    }

    fn flush(&self) {
        println!("执行了flush")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use log;

    #[test]
    fn demo() {
        Qog::default();
        log::trace!("trace level");
        log::debug!("23333, {}", 234);
        log::info!("23333, {}", 234);
        log::warn!("23333, {}", 234);
        log::error!("23333, {}", 234);
    }
}
