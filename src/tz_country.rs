//! Resolve an active-hours timezone from an ISO-3166-1 alpha-2 country code —
//! a port of `openhaze/core/tz_country.py`.
//!
//! The original maps country → representative IANA zone via `pytz`'s
//! `country_timezones` table (taking the first entry). Country granularity is
//! deliberate: the window is a coarse ~10-hour band and only landing on the wrong
//! *continent* matters. This table carries a representative zone for the common
//! codes; an unknown code resolves to `None` (no active-hours gating), matching
//! the original's fallback. The active-hours feature is off by default
//! (`conf::ENABLE_ACTIVE_HOURS = false`).

/// Representative IANA zone for a country code, or `None` if unknown/unresolvable.
/// (`timezone_for_country`)
pub fn timezone_for_country(country_code: Option<&str>) -> Option<&'static str> {
    let cc = country_code?.trim().to_lowercase();
    if cc.is_empty() {
        return None;
    }
    Some(match cc.as_str() {
        // EU / EEA / UK / CH (the jurisdictions geo.rs enumerates) + majors.
        "at" => "Europe/Vienna",
        "be" => "Europe/Brussels",
        "bg" => "Europe/Sofia",
        "hr" => "Europe/Zagreb",
        "cy" => "Asia/Nicosia",
        "cz" => "Europe/Prague",
        "dk" => "Europe/Copenhagen",
        "ee" => "Europe/Tallinn",
        "fi" => "Europe/Helsinki",
        "fr" => "Europe/Paris",
        "de" => "Europe/Berlin",
        "gr" => "Europe/Athens",
        "hu" => "Europe/Budapest",
        "ie" => "Europe/Dublin",
        "it" => "Europe/Rome",
        "lv" => "Europe/Riga",