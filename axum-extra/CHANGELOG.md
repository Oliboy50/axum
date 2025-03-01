# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

# Unreleased

- **added:** Add type safe routing. See `axum_extra::routing::typed` for more details ([#756])
- **breaking:** `CachedRejection` has been removed ([#699])
- **breaking:** `<Cached<T> as FromRequest>::Rejection` is now `T::Rejection`. ([#699])
- **breaking:** `middleware::from_fn` has been moved into the main axum crate ([#719])

[#666]: https://github.com/tokio-rs/axum/pull/666
[#699]: https://github.com/tokio-rs/axum/pull/699
[#719]: https://github.com/tokio-rs/axum/pull/719
[#756]: https://github.com/tokio-rs/axum/pull/756

# 0.1.2 (13. January, 2021)

- **fix:** Depend on tower with `default_features = false` ([#666])

# 0.1.1 (27. December, 2021)

- Add `middleware::from_fn` for creating middleware from async functions ([#656])
- Add support for returning pretty JSON response in `response::ErasedJson` ([#662])

[#656]: https://github.com/tokio-rs/axum/pull/656
[#662]: https://github.com/tokio-rs/axum/pull/662

# 0.1.0 (02. December, 2021)

- Initial release.
