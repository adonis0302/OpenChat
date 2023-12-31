# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [unreleased]

### Changed

- Add `name` and `symbol` to rendering of 'Update token' proposals ([#4167](https://github.com/open-chat-labs/open-chat/pull/4167))

## [[2.0.797](https://github.com/open-chat-labs/open-chat/releases/tag/v2.0.797-registry)] - 2023-08-08

### Added

- Automate adding SNS tokens to the registry ([#4124](https://github.com/open-chat-labs/open-chat/pull/4124))

### Changed

- Store root and governance canisters for SNS tokens ([#4001](https://github.com/open-chat-labs/open-chat/pull/4001))
- Rename `sns_canisters` to `nervous_system` ([#4012](https://github.com/open-chat-labs/open-chat/pull/4012))
- Add ICP token details to registry upon init ([#4013](https://github.com/open-chat-labs/open-chat/pull/4013))
- Support updating token name and symbol ([#4128](https://github.com/open-chat-labs/open-chat/pull/4128))

## [[2.0.756](https://github.com/open-chat-labs/open-chat/releases/tag/v2.0.756-registry)] - 2023-07-20

### Changed

- Remove dependencies on IC repo ([#3970](https://github.com/open-chat-labs/open-chat/pull/3970))

## [[2.0.739](https://github.com/open-chat-labs/open-chat/releases/tag/v2.0.739-registry)] - 2023-07-06

### Changed

- Fetch token logo when adding token to registry ([#3917](https://github.com/open-chat-labs/open-chat/pull/3917))
- Implement `update_token` ([#3921](https://github.com/open-chat-labs/open-chat/pull/3921))
- Give option to manually specify logo in `add_token` ([#3921](https://github.com/open-chat-labs/open-chat/pull/3921))
